use rogalik::{
    math::vectors::Vector2I,
    storage::World
};

use crate::components::{Player, PlayerCharacter};
use crate::utils::spawn_with_position;

pub fn spawn_player(world: &mut World) {
    let entity = spawn_with_position(world, "Player", Vector2I::new(0, 0))
        .unwrap();
    let _ = world.insert_component(entity, Player);
    let _ = world.insert_component(entity, PlayerCharacter{
        active_card: 0
    });
}

pub fn turn_end(world: &mut World) {
    if let Some(item) = world.query::<PlayerCharacter>().iter().next() {
        world.get_component_mut::<PlayerCharacter>(item.entity)
            .unwrap().active_card = 0;
    }
}