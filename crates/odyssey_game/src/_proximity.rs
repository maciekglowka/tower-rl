use rogalik::{
    math::vectors::{ORTHO_DIRECTIONS, Vector2I},
    storage::{Entity, World}
};

use crate::actions::{Action, MeleeHit};
use crate::components::Position;
use crate::utils::{are_hostile, get_entities_at_position};

pub trait ProximityEffect {
    fn description(&self) -> String;
    fn get_actions(
        &self,
        entity: Entity,
        world: &World
    ) -> Vec<Box<dyn Action>>;
}

pub struct Melee {
    pub damage: u32
}
impl ProximityEffect for Melee {
    fn description(&self) -> String {
        "Melee".into()
    }
    fn get_actions(
            &self,
            entity: Entity,
            world: &World
        ) -> Vec<Box<dyn Action>> {
        let Some(position) = world.get_component::<Position>(entity) else { return Vec::new() };
        ORTHO_DIRECTIONS.iter()
            .map(|d| position.0 + *d)
            .map(|v| {
                get_entities_at_position(world, v).iter()
                    .filter(|a| are_hostile(entity, **a, world))
                    .map(|a| *a)
                    .collect::<Vec<_>>()
            })
            .flatten()
            .map(|e| Box::new(MeleeHit { entity, target: e, value: self.damage }) as Box<dyn Action>)
            .collect()
    }
}

