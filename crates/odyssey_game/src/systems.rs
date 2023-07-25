use rogalik::{
    math::vectors::Vector2I,
    storage::{Entity, World}
};
use std::collections::HashMap;

use crate::GameManager;
use crate::actions::{
    Action, ActionResult, ActorQueue, Damage, Pause, PendingActions, SelectedAction
};
use crate::components::{
    Actor, Card, Cooldown, Health, PlayerCharacter, Position, Projectile, Proximity
};
use crate::wind::Wind;

pub fn game_step(world: &mut World, manager: &mut GameManager) {
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
        process_proximity(actor, world);
    }
}

fn get_current_actor(world: &mut World) -> Option<Entity> {
    let queue = world.get_resource::<ActorQueue>()?;
    queue.0.get(0).map(|&e| e)
}

fn process_actor(entity: Entity, world: &mut World, manager: &mut GameManager) -> bool {
    let Some(selected) = get_new_action(entity, world) else { return false };

    if let Ok(_) = execute_action(selected.action, world, manager) {
        if let Some(card) = selected.card {
            if let Some(mut cooldown) = world.get_component_mut::<Cooldown>(card) {
                cooldown.current = cooldown.base;
            }
        }
    }
    true
}

fn get_new_action(entity: Entity, world: &mut World) -> Option<SelectedAction> {
    let Some(mut actor) = world.get_component_mut::<Actor>(entity) else {
        // remove actor from the queue as it might have been killed or smth
        world.get_resource_mut::<ActorQueue>()?.0.retain(|a| *a != entity);
        return None;
    };
    if let Some(action) = actor.action.take() { return Some(action) };

    // if it's player's turn and no action is selected -> return (to wait for input)
    if world.get_component::<PlayerCharacter>(entity).is_some() { return None };

    // otherwise choose npcs actions
    let mut possible_actions = actor.cards.iter()
        .map(|e| get_card_actions(entity, *e, world)
            .drain()
            .map(|a| (a.1, e))
            .collect::<Vec<_>>()
        )
        .flatten()
        .collect::<Vec<_>>();

    possible_actions.sort_by(|a, b| a.0.score(world).cmp(&b.0.score(world)));
    match possible_actions.pop() {
        Some(a) => Some(SelectedAction { action: a.0, card: Some(*a.1) }),
        _ => Some(SelectedAction { action: Box::new(Pause), card: None })
    }
}

pub fn get_card_actions(
    entity: Entity,
    card_entity: Entity,
    world: &World
) -> HashMap<Vector2I, Box<dyn Action>> {
    let Some(card) =  world.get_component::<Card>(card_entity)
        else { return HashMap::new() };
    if let Some(cooldown) = world.get_component::<Cooldown>(card_entity) {
        if cooldown.current > 0 { return HashMap::new()};
    }
    card.0.get_possible_actions(entity, world)
}

fn execute_action(
    mut action: Box<dyn Action>,
    world: &mut World,
    manager: &mut GameManager
) -> ActionResult {
    let mut side_effects = Vec::new();
    let type_id = action.type_id();

    for modifier in manager.action_modifiers.get(&type_id).iter().flat_map(|a| *a) {
        let result = modifier(world, action);
        if result.action.type_id() != type_id {
            // the action has changed it's type
            // start over and discard potential side-effects
            world.get_resource_mut::<PendingActions>().unwrap().0.push_front(result.action);
            // we treat the action as succesful
            // so eg. card's cooldown gets applied
            // TODO rethink this in the future
            return Ok(());
        }
        action = result.action;
        side_effects.extend(result.side_effects);
    }

    let res = action.execute(world);
    if res.is_ok() {
        world.get_resource_mut::<PendingActions>().unwrap().0.extend(side_effects);
        manager.action_events.publish(action.event());
    }
    res
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

fn process_proximity(entity: Entity, world: &World) {
    let Some(proximity) = world.get_component::<Proximity>(entity) else { return };
    let Some(mut pending) = world.get_resource_mut::<PendingActions>() else { return };

    pending.0.extend(proximity.0.get_actions(entity, world));
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
    for item in world.query::<Cooldown>().iter() {
        let mut cooldown = item.get_mut::<Cooldown>().unwrap();
        cooldown.current = cooldown.current.saturating_sub(1);
    }
}

fn turn_end(world: &mut World) {
    if let Some(mut wind) = world.get_resource_mut::<Wind>() {
        wind.pop_wind();
    }
    reduce_cooldown(world);
    collect_actor_queue(world);
}