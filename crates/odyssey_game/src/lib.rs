use std::{
    any::TypeId,
    collections::HashMap
};

mod abilities;
mod actions;
mod action_modifiers;
mod board;
mod components;
mod globals;
mod proximity;
mod systems;
mod wind;

use action_modifiers::ActionModifier;

pub struct GameManager {
    pub action_modifiers: HashMap<TypeId, Vec<ActionModifier>>
}