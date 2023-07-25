use::rogalik::storage::{Component, Entity};

use rogalik::math::vectors::Vector2I;

use crate::abilities::Ability;
use crate::actions::SelectedAction;
use crate::proximity::ProximityEffect;

pub struct Actor {
    pub cards: Vec<Entity>,
    pub action: Option<SelectedAction>
}
impl Component for Actor {}

// actor cannot travel to a blocked tile
pub struct Blocker;
impl Component for Blocker {}

pub struct Card(pub Box<dyn Ability>);
impl Component for Card {
    fn as_str(&self) -> String {
        self.0.description()
    }
}

pub struct Cooldown {
    pub base: u32,
    pub current: u32
}
impl Component for Cooldown {
    fn as_str(&self) -> String {
        format!("Cooldown ({})", self.current)
    }
}

// fixed tile furnishings
pub struct Fixture;
impl Component for Fixture {}

pub struct Health(pub u32);
impl Component for Health {}

// automatic close-distance ability (like melee)
pub struct Proximity(pub Box<dyn ProximityEffect>);
impl Component for Proximity {}

pub struct Name (pub String);
impl Component for Name {}

// many can exist in the world
// marks entities 'allied' or spawned by the player

pub struct Player;
impl Component for Player {}

// only on in the game world
// the actual player
pub struct PlayerCharacter {
    pub active_card: usize
}
impl Component for PlayerCharacter {}

pub struct Position(pub Vector2I);
impl Component for Position {}

pub struct Projectile{
    pub damage: u32,
    pub source: Vector2I,
    pub target: Vector2I
}
impl Component for Projectile {}

pub struct Tile;
impl Component for Tile {}