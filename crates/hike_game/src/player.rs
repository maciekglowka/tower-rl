use rogalik::{
    math::vectors::Vector2I,
    storage::{Entity, World}
};

use crate::actions::{Action, get_action_at_dir};
use crate::board::get_free_tile;
use crate::components::{Position, Player};
use crate::globals::INVENTORY_SIZE;
use crate::utils::spawn_with_position;

pub fn spawn_player(world: &mut World) {
    let Some(position) = get_free_tile(world) else { return };

    // try reuse player
    if pin_player(world, position) { return };

    // else spawn player
    let entity = spawn_with_position(world, "Player", position)
        .unwrap();
    let _ = world.insert_component(entity, Player { 
        ..Default::default()
    });
}

pub fn get_player_position(world: &World) -> Option<Vector2I> {
    Some(world.query::<Player>().with::<Position>()
        .iter().next()?.get::<Position>()?.0)
}

pub fn get_player_entity(world: &World) -> Option<Entity> {
    Some(world.query::<Player>().with::<Position>()
        .iter().next()?.entity)
}

pub fn set_player_action_from_dir(
    world: &mut World,
    dir: Vector2I
) {
    let query = world.query::<Player>();
    let Some(player_item) = query.iter().next() else { return };
    player_item.get_mut::<Player>().unwrap().action = get_action_at_dir(player_item.entity, world, dir);
}

pub fn set_player_action(
    world: &mut World,
    action: Box<dyn Action>
) {
    let query = world.query::<Player>();
    let Some(player_item) = query.iter().next() else { return };
    player_item.get_mut::<Player>().unwrap().action = Some(action);
}

pub fn turn_end(world: &mut World) {
    // if let Some(item) = world.query::<Player>().iter().next() {
    //     world.get_component_mut::<Player>(item.entity)
    //         .unwrap().active_ability = 0;
    // }
}


pub fn unpin_player(world: &mut World) {
    let query = world.query::<Player>().with::<Position>();
    let Some(item) = query.iter().next() else { return };
    world.remove_component::<Position>(item.entity);
}

pub fn pin_player(world: &mut World, position: Vector2I) -> bool {
    let query = world.query::<Player>();
    let Some(item) = query.iter().next() else { return false };
    
    let _ = world.insert_component(item.entity, Position(position));
    true
}

