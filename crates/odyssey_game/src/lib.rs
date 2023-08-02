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
pub use systems::{game_step, get_card_actions};

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
    let mut board = board::Board::new();
    board.generate(world);
    world.insert_resource(board);

    let wind = wind::Wind::new();
    world.insert_resource(wind);

    let queue = actions::ActorQueue(VecDeque::new());
    world.insert_resource(queue);

    let pending = actions::PendingActions(VecDeque::new());
    world.insert_resource(pending);

    player::spawn_player(world);

    let mut rng = thread_rng();
    for _ in 0..3 {
        let v = Vector2I::new(
            rng.gen_range(4..8),
            rng.gen_range(4..8),
        );

        let npc = utils::spawn_with_position(world, "Jellyfish", v);
    }
    register_action_modifiers(manager);
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