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

pub mod actions;
mod board;
pub mod components;
mod events;
pub mod globals;
mod player;
mod items;
mod systems;
mod utils;

pub use player::set_player_action;
pub use board::{Board, ContentKind};
pub use events::ActionEvent;

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
    systems::turn_step(world, manager);
}