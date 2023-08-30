use rogalik::{
    math::vectors::Vector2I,
    storage::{Entity, World}
};
use std::collections::{HashMap, VecDeque};

use crate::actions::{
    Action, ActorQueue, Consume, PendingActions, get_npc_action,
};
use crate::board::{Board, shift_dir};
use crate::components::{Actor, Consumable, ConsumableKind, Durability, Frozen, Health, Player, Position};
use crate::globals::BOARD_SIZE;
use crate::GameManager;
use crate::player;
use crate::utils::get_entities_at_position;

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
}

pub fn turn_step(world: &mut World, manager: &mut GameManager) {
    // hit_projectiles(world);
    check_board_shift(world);
    kill_units(world);
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
    if process_frozen(world, entity) { return true };
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

fn check_board_shift(world: &mut World) -> Option<Vector2I> {
    let position = world.query::<Player>().with::<Position>()
        .iter()
        .next()?
        .get::<Position>()?
        .0;

    let origin = world.get_resource::<Board>()?.origin;

    let dir = match position {
        a if a.x == origin.x => Some(Vector2I::LEFT),
        a if a.y == origin.y => Some(Vector2I::UP),
        a if a.x == origin.x + BOARD_SIZE as i32 - 1 => Some(Vector2I::RIGHT),
        a if a.y == origin.y + BOARD_SIZE as i32 - 1 => Some(Vector2I::DOWN),
        _ => None
    };

    if let Some(dir) = dir {
        shift_dir(world, dir);
    }

    None
}

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
        .filter(|i| i.get::<Durability>().unwrap().value == 0)
        .map(|i| i.entity)
        .collect::<Vec<_>>();
    for entity in to_remove {
        world.despawn_entity(entity);
    }
}

fn kill_units(world: &mut World) {
    let query = world.query::<Health>();
    let entities = query.iter()
        .filter(|a| a.get::<Health>().unwrap().0.current == 0)
        .map(|a| a.entity)
        .collect::<Vec<_>>();
    for entity in entities {
        world.despawn_entity(entity);
    }
}

fn collect_actor_queue(world: &mut World) {
    let Some(mut queue) = world.get_resource_mut::<ActorQueue>() else { return };
    let mut actors = world.query::<Actor>().iter().map(|a| a.entity).collect::<Vec<_>>();
    // actors.sort_by(|a, b| a.id.cmp(&b.id));
    actors.sort_by_key(|a| (a.version, a.id));
    queue.0 = actors.into();
}

fn process_frozen(world: &mut World, entity: Entity) -> bool {
    // returns true if the actor is still paralayzded and cannot act
    // decreases the paralyze counter
    let Some(mut frozen) = world.get_component_mut::<Frozen>(entity)
        else { return false };

    frozen.0 = frozen.0.saturating_sub(1);
    if frozen.0 > 0 { return true }

    drop(frozen);
    world.remove_component::<Frozen>(entity);
    true
}

fn turn_end(world: &mut World) {
    collect_actor_queue(world);
    player::turn_end(world);
}
