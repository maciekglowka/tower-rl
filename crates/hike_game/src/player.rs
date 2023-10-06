use rogalik::{
    math::vectors::Vector2i,
    storage::{Entity, World}
};

use crate::actions::{Action, get_action_at_dir};
use crate::board::get_free_tile;
use crate::components::{Position, Player};
// use crate::globals::INVENTORY_SIZE;
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
) {
    let query = world.query::<Player>().build();
    let Some(entity) = query.single_entity() else { return };
    query.single_mut::<Player>().unwrap().action = get_action_at_dir(entity, world, dir);
}

pub fn set_player_action(
    world: &mut World,
    action: Box<dyn Action>
) {
    if let Some(mut player) = world.query::<Player>().build().single_mut::<Player>() {
        player.action = Some(action);
    };
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

