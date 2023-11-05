use rand::prelude::*;
use rogalik::{
    events::EventBus,
    math::vectors::Vector2i,
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
pub mod structs;
mod systems;
mod utils;

pub use player::{set_player_action, set_player_action_from_dir, get_player_position, get_player_entity};
pub use board::Board;
pub use events::GameEvent;
pub use utils::get_entities_at_position;

// pub struct GameEvents {
//     pub action_events: EventBus<ActionEvent>,
// }
// impl GameEvents {
//     pub fn new() -> Self {
//         GameEvents { 
//             action_events: EventBus::new(),
//         }
//     }
// }

pub fn init(world: &mut World, events: &mut EventBus<GameEvent>, data: hike_data::GameData) {
    world.insert_resource(GameStats::default());
    world.insert_resource(data);
    systems::board_start(world, events);
}

#[derive(Default)]
pub struct GameStats {
    pub kills: HashMap<String, u32>,
    pub win: bool
}

pub fn game_update(world: &mut World, events: &mut EventBus<GameEvent>) -> Result<(), ()> {
    if world.get_resource::<Board>().ok_or(())?.is_exit() {
        systems::board_end(world);
        systems::board_start(world, events);
        return Ok(());
    }
    systems::turn_step(world, events);
    Ok(())
}