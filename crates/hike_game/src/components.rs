use rogalik::storage::{Component, Entity, World};
use rogalik::math::vectors::Vector2i;
use serde::{Deserialize, Deserializer};
use std::collections::{HashSet, HashMap};

use crate::actions::Action;
use crate::globals::MAX_WEAPONS;
use crate::structs::{Attack, Attitude, Effect, InteractionKind, ValueMax};
use crate::utils::deserialize_random_u32;


// deserialized components
#[derive(Deserialize)]
pub struct Actor {
    #[serde(skip)]
    pub target: Option<Vector2i>,
    #[serde(default)]
    pub attitude: Attitude
}
impl Component for Actor {}

#[derive(Deserialize)]
pub struct Budding;
impl Component for Budding {}

// marker for non-weapon items that can be put into inventory for later use
#[derive(Deserialize)]
pub struct Collectable;
impl Component for Collectable {}

#[derive(Deserialize)]
pub struct Durability(#[serde(deserialize_with="deserialize_random_u32")] pub u32);
impl Component for Durability {
    fn as_str(&self) -> String {
        format!("Durability({})", self.0)
    }
}

#[derive(Deserialize)]
pub struct Discoverable;
impl Component for Discoverable {}

#[derive(Deserialize)]
pub struct Effects {
    pub effects: Vec<Effect>
}
impl Component for Effects {}

#[derive(Deserialize)]
// fixed tile furnishings
pub struct Fixture;
impl Component for Fixture {}

#[derive(Deserialize)]
pub struct Health(pub ValueMax);
impl Component for Health {}

// marker component for items used automatically upon walking on them
#[derive(Deserialize)]
pub struct Instant;
impl Component for Instant {}

#[derive(Deserialize)]
pub struct Interactive{
    pub kind: InteractionKind,
    // transforms into another entity after use
    pub next: Option<String>,
    pub cost: Option<u32>
}
impl Component for Interactive {
    fn as_str(&self) -> String {
        let mut output = self.kind.to_str();
        if let Some(cost) = self.cost {
            output += &format!(" Gold({})", cost);
        }
        output
    }
}

// marker component for all item kinds (Weapon, Collectable, Instant)
#[derive(Deserialize)]
pub struct Item;
impl Component for Item {}

// shows an in-game info message
#[derive(Deserialize)]
pub struct Info {
    pub text: String
}
impl Component for Info {}

#[derive(Deserialize)]
pub struct Loot {
    pub items: Vec<String>,
    pub chance: f32
}
impl Component for Loot {}

#[derive(Deserialize)]
// actor cannot travel to a blocked tile
pub struct Obstacle;
impl Component for Obstacle {}

#[derive(Deserialize)]
// close distance: melee / traps
pub struct Offensive {
    pub attacks: Vec<Attack>
}
impl Component for Offensive {}

#[derive(Deserialize)]
pub struct Ranged {
    pub attacks: Vec<Attack>,
    pub distance: u32
}
impl Component for Ranged {}

#[derive(Deserialize)]
pub struct Summoner {
    pub creature: String,
    pub cooldown: ValueMax
}
impl Component for Summoner {}

#[derive(Deserialize)]
pub struct Tile;
impl Component for Tile {}

#[derive(Deserialize)]
pub struct Transition {
    pub next: String
}
impl Component for Transition {}

#[derive(Deserialize)]
pub struct Weapon;
impl Component for Weapon {}

#[derive(Deserialize)]
pub struct Immaterial;
impl Component for Immaterial {}

#[derive(Deserialize)]
pub struct Lunge;
impl Component for Lunge {
    fn as_str(&self) -> String {
        "Lunge".to_string()
    }
}

#[derive(Deserialize)]
pub struct Swing;
impl Component for Swing {
    fn as_str(&self) -> String {
        "Swing".to_string()
    }
}

#[derive(Deserialize)]
pub struct Push;
impl Component for Push {
    fn as_str(&self) -> String {
        "Push".to_string()
    }
}

#[derive(Deserialize)]
pub struct ViewBlocker;
impl Component for ViewBlocker {}

// context-dependet components

#[derive(Default)]
pub struct Name (pub String);
impl Component for Name {}

#[derive(Default)]
pub struct Player {
    pub action: Option<Box<dyn Action>>,
    pub discovered: HashSet<String>,
    pub collectables: Vec<Entity>,
    pub weapons: [Option<Entity>; MAX_WEAPONS],
    pub active_weapon: usize,
    pub gold: u32
}
impl Component for Player {}

pub struct Immune(pub u32);
impl Component for Immune {}

pub struct Stunned(pub u32);
impl Component for Stunned {}

pub struct Poisoned(pub u32);
impl Component for Poisoned {}

pub struct Projectile {
    pub attacks: Vec<Attack>,
    pub source: Vector2i,
    pub target: Vector2i
}
impl Component for Projectile {}

pub struct Position(pub Vector2i);
impl Component for Position {}

pub struct Regeneration(pub u32);
impl Component for Regeneration {}

pub fn insert_data_components(
    entity: Entity,
    world: &mut World,
    value: &serde_yaml::Value
) {
    let Some(data) = value.as_mapping() else { return };
    for (name, component_data) in data.iter() {
        let Some(name) = name.as_str() else { continue };
        match name {
            "Actor" => insert_single::<Actor>(entity, world, component_data),
            "Budding" => insert_single::<Budding>(entity, world, component_data),
            "Collectable" => insert_single::<Collectable>(entity, world, component_data),
            "Discoverable" => insert_single::<Discoverable>(entity, world, component_data),
            "Durability" => insert_single::<Durability>(entity, world, component_data),
            "Effects" => insert_single::<Effects>(entity, world, component_data),
            "Fixture" => insert_single::<Fixture>(entity, world, component_data),
            "Health" => insert_single::<Health>(entity, world, component_data),
            "Immaterial" => insert_single::<Immaterial>(entity, world, component_data),
            "Interactive" => insert_single::<Interactive>(entity, world, component_data),
            "Instant" => insert_single::<Instant>(entity, world, component_data),
            "Item" => insert_single::<Item>(entity, world, component_data),
            "Info" => insert_single::<Info>(entity, world, component_data),
            "Loot" => insert_single::<Loot>(entity, world, component_data),
            "Lunge" => insert_single::<Lunge>(entity, world, component_data),
            "Swing" => insert_single::<Swing>(entity, world, component_data),
            "Obstacle" => insert_single::<Obstacle>(entity, world, component_data),
            "Offensive" => insert_single::<Offensive>(entity, world, component_data),
            "Ranged" => insert_single::<Ranged>(entity, world, component_data),
            "Summoner" => insert_single::<Summoner>(entity, world, component_data),
            "Push" => insert_single::<Push>(entity, world, component_data),
            "Tile" => insert_single::<Tile>(entity, world, component_data),
            "Transition" => insert_single::<Transition>(entity, world, component_data),
            "Weapon" => insert_single::<Weapon>(entity, world, component_data),
            "ViewBlocker" => insert_single::<ViewBlocker>(entity, world, component_data),
            a => panic!("Unknown component {a}")
        };
    }
}

fn insert_single<T>(
    entity: Entity,
    world: &mut World,
    data: &serde_yaml::Value
) where for<'de> T: 'static + Component + Deserialize<'de> {
    let component = serde_yaml::from_value::<T>(data.clone()).expect(&format!("Could not parse {:?}", data));
    let _ =world.insert_component(entity, component);
}

impl<'de> Deserialize<'de> for ValueMax {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let n = u32::deserialize(deserializer)?;
        Ok(ValueMax { current: n, max: n })
    }
}