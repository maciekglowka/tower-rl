use rogalik::{
    math::vectors::Vector2i,
    storage::{Entity, World}
};

use crate::actions::{Action, ActorQueue, get_action_at_dir};
use crate::board::Board;
use crate::components::{Position, Player};

use crate::utils::spawn_with_position;

pub fn spawn_player(world: &mut World) {
    let position = if let Some(board) = world.get_resource::<Board>() {
        board.player_spawn
    } else { return };

    // try reuse player
    if pin_player(world, position) { return };

    // else spawn player
    let entity = spawn_with_position(world, "Player", position)
        .unwrap();
    let _ = world.insert_component(entity, Player { 
        ..Default::default()
    });
}

pub fn get_player_position(world: &World) -> Option<Vector2i> {
    Some(world.query::<Player>().with::<Position>()
        .build().single::<Position>()?.0)
}

pub fn get_player_entity(world: &World) -> Option<Entity> {
    world.query::<Player>().with::<Position>()
        .build().single_entity()
}

pub fn set_player_action_from_dir(
    world: &mut World,
    dir: Vector2i
) -> bool {
    let query = world.query::<Player>().build();
    let Some(entity) = query.single_entity() else { return false };
    if let Some(queue) = world.get_resource::<ActorQueue>() {
        if queue.0.get(0).map(|&e| e) == Some(entity) {
            query.single_mut::<Player>().unwrap().action = get_action_at_dir(entity, world, dir);
            return true;
        }
    }
    false
}

pub fn set_player_action(
    world: &World,
    action: Box<dyn Action>
) -> bool {
    let query = world.query::<Player>().build();
    let Some(entity) = query.single_entity() else { return false };
    if let Some(queue) = world.get_resource::<ActorQueue>() {
        if queue.0.get(0).map(|&e| e) == Some(entity) {
            query.single_mut::<Player>().unwrap().action = Some(action);
            return true
        }
    }
    false
}

pub fn turn_end(world: &mut World) {
    // if let Some(item) = world.query::<Player>().iter().next() {
    //     world.get_component_mut::<Player>(item.entity)
    //         .unwrap().active_ability = 0;
    // }
}


pub fn unpin_player(world: &mut World) {
    let Some(entity) = world.query::<Player>().with::<Position>()
        .build().single_entity() else { return };
    world.remove_component::<Position>(entity);
}

pub fn pin_player(world: &mut World, position: Vector2i) -> bool {
    let Some(entity) = world.query::<Player>().build().single_entity() else { return false };
    let _ = world.insert_component(entity, Position(position));
    true
}

