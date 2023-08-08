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
mod board;
pub mod components;
mod events;
mod globals;
mod player;
mod systems;
mod utils;

pub use board::Board;
pub use events::ActionEvent;
pub use systems::get_ability_actions;

pub struct GameManager {
    pub action_events: EventBus<ActionEvent>,
}
impl GameManager {
    pub fn new() -> Self {
        GameManager { 
            action_events: EventBus::new(),
        }
    }
}

pub fn init(world: &mut World, manager: &mut GameManager) {
    systems::board_start(world);
}


pub fn game_update(world: &mut World, manager: &mut GameManager) {
    if systems::is_board_complete(world) {
        systems::board_end(world);
        systems::board_start(world);
        return;
    }
    systems::turn_step(world, manager);
}