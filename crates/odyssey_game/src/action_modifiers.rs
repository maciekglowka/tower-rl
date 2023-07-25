use rogalik::storage::World;

use crate::actions::{Action, Damage, MeleeHit};

pub struct ActionModifierResult {
    pub action: Box<dyn Action>,
    pub side_effects: Vec<Box<dyn Action>>
}
impl ActionModifierResult {
    pub fn new(action: Box<dyn Action>, side_effects: Vec<Box<dyn Action>>) -> Self {
        ActionModifierResult { action, side_effects }
    }
}

pub type ActionModifier = fn(&World, Box<dyn Action>) -> ActionModifierResult;

pub fn melee_damage(_world: &World, action: Box<dyn Action>) -> ActionModifierResult {
    let mut side_effects: Vec<Box<dyn Action>> = Vec::new();
    if let Some(melee) = action.as_any().downcast_ref::<MeleeHit>() {
        side_effects.push(Box::new(
            Damage { entity: melee.target, value: melee.value }
        ));
    }
    ActionModifierResult::new(action, side_effects)
}