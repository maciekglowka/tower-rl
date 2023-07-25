use rogalik::storage::World;

use crate::actions::Action;

pub struct ActionModifierResult {
    pub action: Box<dyn Action>,
    pub side_effects: Vec<Box<dyn Action>>
}
impl ActionModifierResult {
    pub fn new(action: Box<dyn Action>, side_effects: Vec<Box<dyn Action>>) -> Self {
        ActionModifierResult { action, side_effects }
    }
}

pub type ActionModifier = fn(&mut World, Box<dyn Action>) -> ActionModifierResult;
