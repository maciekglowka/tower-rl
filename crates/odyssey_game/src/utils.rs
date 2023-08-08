use rogalik::{
    math::vectors::Vector2I,
    storage::{Entity, World}
};

use odyssey_data::GameData;

use crate::components::{Actor, Name, Player, Position, insert_data_components};

pub fn are_hostile(source: Entity, target: Entity, world: &World) -> bool {
    if world.get_component::<Player>(source).is_some() {
        return world.get_component::<Player>(target).is_none()
    } else {
        return world.get_component::<Player>(target).is_some()
    }
}

pub fn get_entities_at_position(world: &World, v: Vector2I) -> Vec<Entity> {
    world.query::<Position>().iter()
        .filter(|a| a.get::<Position>().unwrap().0 == v)
        .map(|a| a.entity)
        .collect()
}

pub fn spawn_with_position(
    world: &mut World,
    name: &str,
    position: Vector2I
) -> Option<Entity> {
    let entity = world.spawn_entity();
    let _ = world.insert_component(entity, Name(name.into()));
    let _ = world.insert_component(entity, Position(position));

    let data = world.get_resource::<GameData>()?
        .entities.get(name).expect(&format!("Could not spawn: {} - no data found!", name)).clone();
    insert_data_components(entity, world, &data.components);

    Some(entity)
}