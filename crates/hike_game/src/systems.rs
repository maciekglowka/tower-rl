use rogalik::{
    math::vectors::Vector2i,
    storage::{Entity, World}
};
use std::collections::{HashMap, VecDeque};

use crate::actions::{
    Action, ActorQueue, AttackAction, Damage, DropLoot, Heal, PendingActions, UseInstant, get_npc_action,
};
use crate::board::{Board, update_visibility};
use crate::components::{
    Actor, Durability, Fixture, Immune, Instant, Stunned, Health, Offensive, Projectile, Regeneration,
    Player, Position, Poisoned, Transition
};
use crate::GameEvents;
use crate::player;
use crate::structs::get_attack_action;
use crate::utils::{get_entities_at_position, spawn_with_position};

pub fn board_start(world: &mut World, events: &mut GameEvents) {
    // replace board resource
    let level = match world.get_resource::<Board>() {
        Some(b) => b.level,
        _=> 0
    };
    let mut board = Board::new(level + 1);
    board.generate(world);
    world.insert_resource(board);

    // reset queues
    let queue = ActorQueue(VecDeque::new());
    world.insert_resource(queue);

    let pending = PendingActions(VecDeque::new());
    world.insert_resource(pending);

    player::spawn_player(world);
    events.action_events.publish(crate::ActionEvent::BoardReady);
}

pub fn board_end(world: &mut World) {
    // unpin player
    player::unpin_player(world);
    // despawn board objects
    let to_remove = world.query::<Position>().build().entities()
        .copied().collect::<Vec<_>>();
    for entity in to_remove {
        world.despawn_entity(entity);
    }
}

pub fn turn_step(world: &mut World, events: &mut GameEvents) {
    hit_projectiles(world);
    update_visibility(world);
    kill_units(world, events);
    destroy_items(world);
    handle_instant(world);
    if process_pending_action(world, events) {
        // do not process the actor queue if the pending actions were executed
        return
    }
    let Some(actor) = get_current_actor(world) else {
        turn_end(world);
        return
    };
    if process_actor(actor, world, events) {
        // if we reached this point it should be safe to unwrap
        // on the actor queue
        world.get_resource_mut::<ActorQueue>().unwrap().0.pop_front();
    }
}

fn get_current_actor(world: &mut World) -> Option<Entity> {
    let queue = world.get_resource::<ActorQueue>()?;
    queue.0.get(0).map(|&e| e)
}

fn process_actor(entity: Entity, world: &mut World, events: &mut GameEvents) -> bool {
    // returns true if the actor is done
    if process_stunned(world, entity) { return true };
    let Some(selected) = get_new_action(entity, world) else { return false };
    execute_action(selected, world, events).is_ok()
}

fn get_new_action(entity: Entity, world: &mut World) -> Option<Box<dyn Action>> {
    let Some(_) = world.get_component::<Actor>(entity) else {
        // remove actor from the queue as it might have been killed or smth
        world.get_resource_mut::<ActorQueue>()?.0.retain(|a| *a != entity);
        return None;
    };

    // if it's player's turn and no action is selected -> return (to wait for input)
    if let Some(mut player) = world.get_component_mut::<Player>(entity) { 
        return player.action.take();
    };

    update_npc_target(world, entity);
    Some(get_npc_action(entity, world))
}


fn execute_action(
    action: Box<dyn Action>,
    world: &mut World,
    events: &mut GameEvents
) -> Result<(), ()> {
    let res = action.execute(world);
    if let Ok(res) = res {
        world.get_resource_mut::<PendingActions>().unwrap().0.extend(res);
        events.action_events.publish(action.event());
        return Ok(())
    }
    Err(())
}

fn process_pending_action(world: &mut World, events: &mut GameEvents) -> bool {
    let Some(pending) = world.get_resource_mut::<PendingActions>()
            .unwrap()
            .0
            .pop_front() 
        else {
            return false
        };
    let _ = execute_action(pending, world, events);
    true
}

fn hit_projectiles(world: &mut World) {
    // this should be called before actions are exectued
    // to clear projectiles spawned at the previous tick
    let query = world.query::<Projectile>().build();

    if let Some(mut pending) = world.get_resource_mut::<PendingActions>() {
        for projectile in query.iter::<Projectile>() {
            let actions: Vec<_> = projectile.attacks.iter()
                .map(|a| get_attack_action(a, projectile.target))
                .collect();
            pending.0.extend(actions);
        }
    };

    // despawn projectiles
    let entities = query.entities().map(|&e| e).collect::<Vec<_>>();
    drop(query);
    for entity in entities {
        world.despawn_entity(entity);
        
    }
}

fn handle_instant(
    world: &mut World
) {
    let Some(player_v) = player::get_player_position(world) else { return };

    let actions = get_entities_at_position(world, player_v).iter()
        .filter(|e| world.get_component::<Instant>(**e).is_some())
        .map(|&entity| Box::new(UseInstant { entity }) as Box<dyn Action> )
        .collect::<Vec<_>>();
    if let Some(mut pending) = world.get_resource_mut::<PendingActions>() {
        pending.0.extend(actions);
    }
}

fn destroy_items(
    world: &mut World
) {
    let query = world.query::<Durability>().build();
    let to_remove = query.iter::<Durability>().zip(query.entities())
        .filter(|(d, _)| d.0 == 0)
        .map(|(_, e)| *e)
        .collect::<Vec<_>>();

    for entity in to_remove {
        world.despawn_entity(entity);

        // TODO refactor?
        if let Some(mut player) = world.query::<Player>().build().single_mut::<Player>() {
            for idx in 0..player.weapons.len() {
                if player.weapons[idx] == Some(entity) {
                    player.weapons[idx] = None;
                }
            }
            player.collectables.retain(|&e| e != entity);
        }
    }
}

fn kill_units(world: &mut World, events: &mut GameEvents) {
    let query = world.query::<Health>().build();
    let entities = query.iter::<Health>().zip(query.entities())
        .filter(|(h, _)| h.0.current == 0)
        .map(|(_, e)| *e)
        .collect::<Vec<_>>();

    for entity in entities {
        let _ = execute_action(
            Box::new(DropLoot { entity }),
            world,
            events
        );
        world.despawn_entity(entity);
    }
}

fn collect_actor_queue(world: &mut World) {
    let Some(mut queue) = world.get_resource_mut::<ActorQueue>() else { return };
    let mut actors = world.query::<Actor>().build().entities().copied().collect::<Vec<_>>();
    actors.sort_by_key(|a| (a.version, a.id));
    queue.0 = actors.into();
}

fn process_stunned(world: &mut World, entity: Entity) -> bool {
    // returns true if the actor is still stunned and cannot act
    // decreases the stun counter
    let Some(mut stunned) = world.get_component_mut::<Stunned>(entity)
        else { return false };

    stunned.0 = stunned.0.saturating_sub(1);
    if stunned.0 > 0 { return true }

    drop(stunned);
    world.remove_component::<Stunned>(entity);
    true
}

fn process_poisoned(world: &mut World) {
    let mut to_remove = Vec::new();
    let Some(mut pending) = world.get_resource_mut::<PendingActions>() else { return };
    let query = world.query::<Poisoned>().with::<Health>().build();
    for (mut poisoned, &entity) in query.iter_mut::<Poisoned>().zip(query.entities()) {
        poisoned.0 = poisoned.0.saturating_sub(1);
        if poisoned.0 <= 0 {
            to_remove.push(entity);
        }
        pending.0.push_back(Box::new(Damage { entity: entity, value: 1 }))
    }
    drop(pending);
    for entity in to_remove {
        world.remove_component::<Poisoned>(entity);
    }
}

fn process_immune(world: &mut World) {
    let mut to_remove = Vec::new();
    let query = world.query::<Immune>().build();
    for (mut immune, &entity) in query.iter_mut::<Immune>().zip(query.entities()) {
        immune.0 = immune.0.saturating_sub(1);
        if immune.0 <= 0 {
            to_remove.push(entity);
        }
    }
    for entity in to_remove {
        world.remove_component::<Immune>(entity);
    }
}

fn process_regeneration(world: &mut World) {
    let mut to_remove = Vec::new();
    let query = world.query::<Regeneration>().build();
    let Some(mut pending) = world.get_resource_mut::<PendingActions>() else { return };
    for (mut regeneration, &entity) in query.iter_mut::<Regeneration>().zip(query.entities()) {
        regeneration.0 = regeneration.0.saturating_sub(1);
        if regeneration.0 <= 0 {
            to_remove.push(entity);
        }
        pending.0.push_back(Box::new(Heal { entity: entity, value: 1 }))
    }
    drop(pending);
    for entity in to_remove {
        world.remove_component::<Regeneration>(entity);
    }
}

fn process_offensive_fixtures(world: &mut World) {
    let query = world.query::<Fixture>()
        .with::<Offensive>()
        .with::<Position>()
        .build();

    let Some(mut pending) = world.get_resource_mut::<PendingActions>() else { return };
    for (&entity, position) in query.entities().zip(query.iter::<Position>()) {
        pending.0.push_back(
            Box::new(AttackAction { entity, target: position.0 })
        );
    }
}

fn process_transition(world: &mut World) {
    let query = world.query::<Transition>().with::<Position>().build();
    let mut to_despawn = Vec::new();
    let mut to_spawn = Vec::new();
    for ((transition, position), &entity) in query.iter::<Transition>()
        .zip(query.iter::<Position>())
        .zip(query.entities())
    {
        to_despawn.push(entity);
        to_spawn.push((position.0, transition.next.clone()));
    }
    for entity in to_despawn {
        world.despawn_entity(entity);
    }
    for (v, name) in to_spawn {
        spawn_with_position(world, &name, v);
    }
}

fn update_npc_target(world: &mut World, entity: Entity) {
    let Some(mut actor) = world.get_component_mut::<Actor>(entity) else { return };
    let Some(position) = world.get_component::<Position>(entity) else { return };

    if Some(position.0) == actor.target {
        actor.target = None
    };
    let Some(player_v) = player::get_player_position(world) else { return };
    if crate::utils::visibility(world, position.0, player_v) {
        actor.target = Some(player_v);
    }
}

fn turn_end(world: &mut World) {
    collect_actor_queue(world);
    player::turn_end(world);
    process_regeneration(world);
    process_poisoned(world);
    process_immune(world);
    process_transition(world);
    process_offensive_fixtures(world);
}

