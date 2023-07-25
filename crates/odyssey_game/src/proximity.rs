use rogalik::{
    math::vectors::Vector2I,
    storage::{Entity, World}
};

use crate::actions::Action;

pub trait ProximityEffect {
    fn description(&self) -> String;
    fn get_actions(
        &self,
        entity: Entity,
        targets: Vec<Vector2I>,
        world: &World
    ) -> Vec<Box<dyn Action>>;
}