use rogalik::{
    math::vectors::Vector2I,
    storage::World
};

use crate::components::{Player, PlayerCharacter, Position};
use crate::utils::spawn_with_position;

pub fn spawn_player(world: &mut World) {
    let position = Vector2I::new(0, 0);

    // try reuse player
    if pin_player(world, position) { return };

    // else spawn player
    let entity = spawn_with_position(world, "Player", position)
        .unwrap();
    let _ = world.insert_component(entity, Player);
    let _ = world.insert_component(entity, PlayerCharacter{
        active_ability: 0,
        selected_action: None
    });
}

pub fn turn_end(world: &mut World) {
    if let Some(item) = world.query::<PlayerCharacter>().iter().next() {
        world.get_component_mut::<PlayerCharacter>(item.entity)
            .unwrap().active_ability = 0;
    }
}

pub fn unpin_player(world: &mut World) {
    let query = world.query::<PlayerCharacter>().with::<Position>();
    let Some(item) = query.iter().next() else { return };
    world.remove_component::<Position>(item.entity);
}

pub fn pin_player(world: &mut World, position: Vector2I) -> bool {
    let query = world.query::<PlayerCharacter>();
    let Some(item) = query.iter().next() else { return false };

    let _ = world.insert_component(item.entity, Position(position));

    true
}
