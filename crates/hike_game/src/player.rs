use rogalik::{
    math::vectors::Vector2I,
    storage::World
};

use crate::actions::{Action, get_action_at_dir};
use crate::components::Player;
use crate::globals::INVENTORY_SIZE;
use crate::utils::spawn_with_position;

pub fn spawn_player(world: &mut World) {
    let position = Vector2I::new(1, 1);

    // else spawn player
    let entity = spawn_with_position(world, "Player", position)
        .unwrap();
    let _ = world.insert_component(entity, Player { 
        action: None,
        items: [None; INVENTORY_SIZE],
        active_item: 0,
    });
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
