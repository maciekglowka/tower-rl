use rand::prelude::*;
use rogalik::{
    events::EventBus,
    math::vectors::Vector2I,
    storage::World
};
use std::{
    any::TypeId,
    collections::{HashMap, VecDeque}
};

mod abilities;
pub mod actions;
mod action_modifiers;
mod board;
mod cards;
pub mod components;
mod events;
mod globals;
mod player;
// mod proximity;
mod systems;
mod utils;
mod wind;

pub use board::Board;
pub use events::ActionEvent;
pub use wind::Wind;
pub use systems::get_card_actions;

use action_modifiers::ActionModifier;

pub struct GameManager {
    pub action_events: EventBus<ActionEvent>,
    pub action_modifiers: HashMap<TypeId, Vec<ActionModifier>>,
}
impl GameManager {
    pub fn new() -> Self {
        GameManager { 
            action_events: EventBus::new(),
            action_modifiers: HashMap::new()
        }
    }
}

pub fn init(world: &mut World, manager: &mut GameManager) {
    register_action_modifiers(manager);
    systems::board_start(world);
}

fn register_action_modifiers(manager: &mut GameManager) {
    manager.action_modifiers = HashMap::from_iter([
        (
            TypeId::of::<actions::MeleeHit>(), [
                action_modifiers::melee_damage as action_modifiers::ActionModifier
            ].to_vec()
        )
    ]);
}

pub fn game_update(world: &mut World, manager: &mut GameManager) {
    if systems::is_board_complete(world) {
        systems::board_end(world);
        systems::board_start(world);
        return;
    }
    systems::turn_step(world, manager);
}