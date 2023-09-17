use rogalik::{
    math::vectors::Vector2I,
    storage::{Entity, World}
};
use std::collections::{HashMap, VecDeque};

use crate::actions::{
    Action, ActorQueue, Damage, DropLoot, PendingActions, get_npc_action,
};
use crate::board::{Board, update_visibility};
use crate::components::{
    Actor, Consumable, ConsumableKind, Durability, Immune, Stunned, Health, Player, Position, Poisoned
};
use crate::globals::BOARD_SIZE;
use crate::GameManager;
use crate::player;
use crate::utils::get_entities_at_position;

pub fn board_start(world: &mut World, manager: &mut GameManager) {
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
    manager.action_events.publish(crate::ActionEvent::BoardReady);
}

pub fn board_end(world: &mut World) {
    // unpin player
    player::unpin_player(world);
    // despawn board objects
    let objects = world.query::<Position>().iter()
        .map(|i| i.entity)
        .collect::<Vec<_>>();
    for entity in objects {
        world.despawn_entity(entity);
    }
}

pub fn turn_step(world: &mut World, manager: &mut GameManager) {
    // hit_projectiles(world);
    update_visibility(world);
    kill_units(world, manager);
    destroy_items(world);
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
    }
}

fn get_current_actor(world: &mut World) -> Option<Entity> {
    let queue = world.get_resource::<ActorQueue>()?;
    queue.0.get(0).map(|&e| e)
}

fn process_actor(entity: Entity, world: &mut World, manager: &mut GameManager) -> bool {
    // returns true if the actor is done
    if process_stunned(world, entity) { return true };
    let Some(selected) = get_new_action(entity, world) else { return false };
    execute_action(selected, world, manager).is_ok()
}

fn get_new_action(entity: Entity, world: &mut World) -> Option<Box<dyn Action>> {
    let Some(actor) = world.get_component::<Actor>(entity) else {
        // remove actor from the queue as it might have been killed or smth
        world.get_resource_mut::<ActorQueue>()?.0.retain(|a| *a != entity);
        return None;
    };

    // if it's player's turn and no action is selected -> return (to wait for input)
    if let Some(mut player) = world.get_component_mut::<Player>(entity) { 
        return player.action.take();
    };

    Some(get_npc_action(entity, world))
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

// fn hit_projectiles(world: &mut World) {
//     // this should be called before actions are exectued
//     // to clear projectiles spawned at the previous tick
//     let query = world.query::<Projectile>();
//     let health_query = world.query::<Health>().with::<Position>();

//     if let Some(mut pending) = world.get_resource_mut::<PendingActions>() {
//         for item in query.iter() {
//             let projectile = item.get::<Projectile>().unwrap();
//             let target = health_query.iter()
//                 .filter(|a| a.get::<Position>().unwrap().0 == projectile.target)
//                 .next();
//             if let Some(target) = target {
//                 pending.0.push_back(
//                     Box::new(Damage { entity: target.entity, value: projectile.damage })
//                 );
//             }
//         }
//     };

//     // despawn projectiles
//     let entities = query.iter()
//         .map(|a| a.entity)
//         .collect::<Vec<_>>();
//     for entity in entities {
//         world.despawn_entity(entity);
        
//     }
// }

// fn handle_consumable(
//     world: &mut World
// ) {
//     let Some(player_v) = player::get_player_position(world) else { return };
//     let player = world.query::<Player>().iter().next().unwrap().entity;

//     let actions = get_entities_at_position(world, player_v).iter()
//         .filter(|e| world.get_component::<Consumable>(**e).is_some())
//         .map(|&entity| Box::new(Consume { entity, consumer: player }) as Box<dyn Action> )
//         .collect::<Vec<_>>();
//     if let Some(mut pending) = world.get_resource_mut::<PendingActions>() {
//         pending.0.extend(actions);
//     }
// }

fn destroy_items(
    world: &mut World
) {
    let to_remove = world.query::<Durability>().iter()
        .filter(|i| i.get::<Durability>().unwrap().0 == 0)
        .map(|i| i.entity)
        .collect::<Vec<_>>();
    for entity in to_remove {
        world.despawn_entity(entity);

        if let Some(player_item) = world.query::<Player>().iter().next() {
            let mut player = player_item.get_mut::<Player>().unwrap();
            for idx in 0..player.items.len() {
                if player.items[idx] == Some(entity) {
                    player.items[idx] = None;
                }
            }
        }
    }
}

fn kill_units(world: &mut World, manager: &mut GameManager) {
    let query = world.query::<Health>();
    let entities = query.iter()
        .filter(|a| a.get::<Health>().unwrap().0.current == 0)
        .map(|a| a.entity)
        .collect::<Vec<_>>();
    for entity in entities {
        let _ = execute_action(
            Box::new(DropLoot { entity }),
            world,
            manager
        );
        world.despawn_entity(entity);
    }
}

fn collect_actor_queue(world: &mut World) {
    let Some(mut queue) = world.get_resource_mut::<ActorQueue>() else { return };
    let mut actors = world.query::<Actor>().iter().map(|a| a.entity).collect::<Vec<_>>();
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
    for item in world.query::<Poisoned>().with::<Health>().iter() {
        let mut poisoned = item.get_mut::<Poisoned>().unwrap();
        poisoned.0 = poisoned.0.saturating_sub(1);
        if poisoned.0 <= 0 {
            to_remove.push(item.entity);
        }
        pending.0.push_back(Box::new(Damage { entity: item.entity, value: 1 }))
    }
    drop(pending);
    for entity in to_remove {
        world.remove_component::<Poisoned>(entity);
    }
}

fn process_immune(world: &mut World) {
    let mut to_remove = Vec::new();
    for item in world.query::<Immune>().iter() {
        let mut immune = item.get_mut::<Immune>().unwrap();
        immune.0 = immune.0.saturating_sub(1);
        if immune.0 <= 0 {
            to_remove.push(item.entity);
        }
    }
    for entity in to_remove {
        world.remove_component::<Immune>(entity);
    }
}

fn turn_end(world: &mut World) {
    collect_actor_queue(world);
    player::turn_end(world);
    process_poisoned(world);
    process_immune(world);
}
