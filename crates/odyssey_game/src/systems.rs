use rogalik::{
    math::vectors::Vector2I,
    storage::{Entity, World}
};
use rand::prelude::*;
use std::collections::{HashMap, VecDeque};

use crate::actions::{
    Action, ActorQueue, Damage, Pause, PendingActions
};
use crate::abilities::{get_possible_actions, Ability};
use crate::board::{Board, create_spawner, update_visibility};
use crate::components::{
    Actor, Health, Player, PlayerCharacter, Position, Projectile, Spawner, Vortex
};
use crate::GameManager;
use crate::player;
use crate::utils;

pub fn board_start(world: &mut World) {
    // replace board resource
    let mut board = Board::new();
    board.generate(world);
    world.insert_resource(board);

    // reset queues
    let queue = ActorQueue(VecDeque::new());
    world.insert_resource(queue);

    let pending = PendingActions(VecDeque::new());
    world.insert_resource(pending);

    player::spawn_player(world);
    // spawn_npcs(world);
}

pub fn board_end(world: &mut World) {
    // despawn board objects
    let objects = world.query::<Position>().iter()
        .map(|i| i.entity)
        .collect::<Vec<_>>();
    for entity in objects {
        world.despawn_entity(entity);
    }
}

pub fn turn_step(world: &mut World, manager: &mut GameManager) {
    update_visibility(world);
    hit_projectiles(world);
    kill_units(world);
    if process_pending_action(world, manager) {
        // do not process the actor queue if the pending actions were executed
        return
    }
    let Some(actor) = get_current_actor(world) else {
        turn_end(world);
        return
    };
    if process_actor(actor, world, manager) {
        // if we reached this point it should be safe to unwrap
        // on the actor queue
        world.get_resource_mut::<ActorQueue>().unwrap().0.pop_front();
        // process_proximity(actor, world);
    }
}

fn get_current_actor(world: &mut World) -> Option<Entity> {
    let queue = world.get_resource::<ActorQueue>()?;
    queue.0.get(0).map(|&e| e)
}

fn process_actor(entity: Entity, world: &mut World, manager: &mut GameManager) -> bool {
    let Some(selected) = get_new_action(entity, world) else { return false };

    if let Ok(_) = execute_action(selected, world, manager) {
        if let Some(player) = world.get_component::<PlayerCharacter>(entity) { 
            if let Some(mut actor) = world.get_component_mut::<Actor>(entity) {
                let ability = actor.abilities.get_mut(player.active_ability).unwrap();
                if let Some(ref mut cooldown) = ability.cooldown {
                    cooldown.current = cooldown.max;
                }
            }
        };
    }
    true
}

fn get_new_action(entity: Entity, world: &mut World) -> Option<Box<dyn Action>> {
    let Some(mut actor) = world.get_component_mut::<Actor>(entity) else {
        // remove actor from the queue as it might have been killed or smth
        world.get_resource_mut::<ActorQueue>()?.0.retain(|a| *a != entity);
        return None;
    };

    // if it's player's turn and no action is selected -> return (to wait for input)
    if let Some(mut player) = world.get_component_mut::<PlayerCharacter>(entity) { 
        return player.selected_action.take();
    };

    // otherwise choose npcs actions
    let mut possible_actions = actor.abilities.iter()
        .map(|ability| get_ability_actions(entity, ability, world)
            .drain()
            .map(|a| a.1)
            .collect::<Vec<_>>()
        )
        .flatten()
        .collect::<Vec<_>>();

    possible_actions.sort_by(|a, b| a.score(world).cmp(&b.score(world)));
    match possible_actions.pop() {
        Some(a) => Some(a),
        _ => Some(Box::new(Pause))
    }
}

pub fn get_ability_actions(
    entity: Entity,
    ability: &Ability,
    world: &World
) -> HashMap<Vector2I, Box<dyn Action>> {
    if let Some(cooldown) = ability.cooldown {
        if cooldown.current > 0 { return HashMap::new()};
    }
    get_possible_actions(entity, ability, world)
}

fn execute_action(
    action: Box<dyn Action>,
    world: &mut World,
    manager: &mut GameManager
) -> Result<(), ()> {
    let res = action.execute(world);
    if let Ok(res) = res {
        world.get_resource_mut::<PendingActions>().unwrap().0.extend(res);
        manager.action_events.publish(action.event());
        return Ok(())
    }
    Err(())
}

fn process_pending_action(world: &mut World, manager: &mut GameManager) -> bool {
    let Some(pending) = world.get_resource_mut::<PendingActions>()
            .unwrap()
            .0
            .pop_front() 
        else {
            return false
        };
    let _ = execute_action(pending, world, manager);
    true
}

fn hit_projectiles(world: &mut World) {
    // this should be called before actions are exectued
    // to clear projectiles spawned at the previous tick
    let query = world.query::<Projectile>();
    let health_query = world.query::<Health>().with::<Position>();

    if let Some(mut pending) = world.get_resource_mut::<PendingActions>() {
        for item in query.iter() {
            let projectile = item.get::<Projectile>().unwrap();
            let target = health_query.iter()
                .filter(|a| a.get::<Position>().unwrap().0 == projectile.target)
                .next();
            if let Some(target) = target {
                pending.0.push_back(
                    Box::new(Damage { entity: target.entity, value: projectile.damage })
                );
            }
        }
    };

    // despawn projectiles
    let entities = query.iter()
        .map(|a| a.entity)
        .collect::<Vec<_>>();
    for entity in entities {
        world.despawn_entity(entity);
        
    }
}

fn kill_units(world: &mut World) {
    let query = world.query::<Health>();
    let entities = query.iter()
        .filter(|a| a.get::<Health>().unwrap().0 == 0)
        .map(|a| a.entity)
        .collect::<Vec<_>>();
    for entity in entities {
        world.despawn_entity(entity);
    }
}

fn collect_actor_queue(world: &mut World) {
    let Some(mut queue) = world.get_resource_mut::<ActorQueue>() else { return };
    let mut actors = world.query::<Actor>().iter().map(|a| a.entity).collect::<Vec<_>>();
    actors.sort_by(|a, b| a.id.cmp(&b.id));
    queue.0 = actors.into();
}

fn reduce_cooldown(world: &mut World) {
    for item in world.query::<Actor>().iter() {
        let mut actor = item.get_mut::<Actor>().unwrap();
        for abilitiy in actor.abilities.iter_mut() {
            if let Some(ref mut cooldown) = abilitiy.cooldown {
                cooldown.current = cooldown.current.saturating_sub(1);
            }
        }
    }
}

fn turn_end(world: &mut World) {
    reduce_cooldown(world);
    collect_actor_queue(world);
    player::turn_end(world);
    spawn_npcs(world);
}

fn spawn_npcs(world: &mut World) {
    // process spawners
    let mut new = Vec::new();
    for item in world.query::<Spawner>().with::<Position>().iter() {
        let mut spawner = item.get_mut::<Spawner>().unwrap();
        spawner.countdown = spawner.countdown.saturating_sub(1);

        if spawner.countdown == 0 {
            let position = item.get::<Position>().unwrap();
            new.push((item.entity, spawner.target.clone(), position.0));
        }
    }

    for item in new {
        utils::spawn_with_position(world, &item.1, item.2);
        world.despawn_entity(item.0);
    }

    // add new spawners
    let mut rng = thread_rng();
    if rng.gen_bool(0.25) { 
        create_spawner(world);
     };
}

pub fn is_board_complete(world: &World) -> bool {
    if let Some(player) = world.query::<PlayerCharacter>().iter().next() {
        if let Some(position) = player.get::<Position>() {
            if utils::get_entities_at_position(world, position.0).iter().any(
                |e| world.get_component::<Vortex>(*e).is_some()
            ) { return true }
        }
    }
    false
}