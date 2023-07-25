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
pub mod components;
mod events;
mod globals;
mod proximity;
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

    let sail_card = world.spawn_entity();
    let _ = world.insert_component(sail_card, components::Card(
        Box::new(abilities::Sailing)
    ));
    let cannons_card = world.spawn_entity();
    let _ = world.insert_component(cannons_card, components::Card(
        Box::new(abilities::Cannons { dist: 4, damage: 2 })
    ));
    let _ = world.insert_component(cannons_card, components::Cooldown { base: 5, current: 0 });
    let buoy_card = world.spawn_entity();
    let _ = world.insert_component(buoy_card, components::Card(
        Box::new(abilities::Buoy { health: 2 })
    ));
    let _ = world.insert_component(buoy_card, components::Cooldown { base: 3, current: 0 });

    let player = world.spawn_entity();
    let _ = world.insert_component(player, components::Position(Vector2I::new(0, 0)));
    let _ = world.insert_component(player, components::Name("Player".into()));
    let _ = world.insert_component(player, components::Blocker);
    let _ = world.insert_component(player, components::Health(1));
    let _ = world.insert_component(player, components::Player);
    let _ = world.insert_component(player, components::PlayerCharacter{
        active_card: 0
    });
    let _ = world.insert_component(player, components::Actor { 
        cards: vec![sail_card, cannons_card, buoy_card],
        action: None
    });

    let rowers_card = world.spawn_entity();
    let _ = world.insert_component(rowers_card, components::Card(
        Box::new(abilities::Swimming)
    ));

    let mut rng = thread_rng();
    for _ in 0..3 {
        let v = Vector2I::new(
            rng.gen_range(4..8),
            rng.gen_range(4..8),
        );
        let npc = world.spawn_entity();
        let _ = world.insert_component(npc, components::Position(v));
        let _ = world.insert_component(npc, components::Name("Rowers".into()));
        let _ = world.insert_component(npc, components::Health(1));
        let _ = world.insert_component(npc, components::Proximity(
            Box::new(proximity::Melee { damage: 1 })
        ));
        let _ = world.insert_component(npc, components::Blocker);
        let _ = world.insert_component(npc, components::Actor { 
            cards: vec![rowers_card],
            action: None
        });
        register_action_modifiers(manager);
    }
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