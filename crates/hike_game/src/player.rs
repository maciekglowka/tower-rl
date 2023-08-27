use rogalik::{
    math::vectors::Vector2I,
    storage::World
};

use crate::components::Player;
use crate::globals::INVENTORY_SIZE;
use crate::utils::spawn_with_position;

pub fn spawn_player(world: &mut World) {
    let position = Vector2I::new(0, 0);

    // else spawn player
    let entity = spawn_with_position(world, "Player", position)
        .unwrap();
    let _ = world.insert_component(entity, Player { 
        action: None,
        items: [None; INVENTORY_SIZE],
        active_item: 0
    });
}

pub fn turn_end(world: &mut World) {
    // if let Some(item) = world.query::<Player>().iter().next() {
    //     world.get_component_mut::<Player>(item.entity)
    //         .unwrap().active_ability = 0;
    // }
}
