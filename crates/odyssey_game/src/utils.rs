use rogalik::{
    math::vectors::Vector2I,
    storage::{Entity, World}
};

use crate::components::{Player, Position};

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