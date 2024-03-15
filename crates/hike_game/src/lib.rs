use rand::prelude::*;
use rogalik::{
    engine::Instant,
    events::EventBus,
    math::vectors::Vector2i,
    storage::World
};
use serde::{Serialize, Deserialize};
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

pub fn init(world: &mut World, events: &mut EventBus<GameEvent>, data: hike_data::GameData) {
    world.insert_resource(GameStats::new());
    world.insert_resource(data);
    systems::board_start(world, events);
}

pub fn restore(world: &mut World, data: hike_data::GameData, saved_state: Vec<u8>) {
    world.deserialize(&saved_state).unwrap();
    world.insert_resource(data);
    world.insert_resource(actions::PendingActions(VecDeque::new()));
    world.insert_resource(actions::ActorQueue(VecDeque::new()));
}

#[derive(Serialize, Deserialize)]
pub struct GameStats {
    pub kills: HashMap<String, u32>,
    pub start: Instant,
    pub win: bool
}
impl GameStats {
    pub fn new() -> Self {
        Self {
            start: Instant::init(),
            kills: HashMap::default(),
            win: false,        }
    }
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