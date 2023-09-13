use rand::prelude::*;
use rogalik::storage::{Component, Entity, World};
use rogalik::math::vectors::Vector2I;
use serde::{Deserialize, Deserializer};

use crate::actions::Action;
use crate::globals::INVENTORY_SIZE;
// use crate::items::ItemKind;

// #[derive(Deserialize, PartialEq)]
// pub enum AttackKind {
//     Freeze,
//     Hit,
//     Poison
// }

#[derive(Deserialize)]
pub enum ConsumableKind {
    Gold,
    Heal
}

pub struct ValueMax {
    pub current: u32,
    pub max: u32
}

#[derive(Deserialize, PartialEq)]
pub enum InteractionKind {
    Ascend,
    Repair(#[serde(deserialize_with="deserialize_random_u32")] u32),
    // UpgradeOffensive(#[serde(deserialize_with="deserialize_random_u32")] u32),
    UpgradeHealth(#[serde(deserialize_with="deserialize_random_u32")] u32),
}
impl InteractionKind {
    pub fn to_str(&self) -> String {
        match self {
            InteractionKind::Ascend => "Ascend".to_string(),
            InteractionKind::Repair(v) => format!("Repair({})", v),
            InteractionKind::UpgradeHealth(v) => format!("Incr. HP({})", v),
            // InteractionKind::UpgradeOffensive(v) => format!("Incr. A({})", v),
        }
    }
}

// deserialized components
#[derive(Deserialize)]
pub struct Actor;
impl Component for Actor {}

// deserialized components

#[derive(Deserialize)]
pub struct Consumable {
    pub kind: ConsumableKind,
    #[serde(deserialize_with="deserialize_random_u32")]
    pub value: u32
}
impl Component for Consumable {
    fn as_str(&self) -> String {
        let action = match self.kind {
            ConsumableKind::Gold => "Gold",
            ConsumableKind::Heal => "Heal",
        };
        format!("{} ({})", action, self.value)
    }
}

#[derive(Deserialize)]
pub struct Durability(#[serde(deserialize_with="deserialize_random_u32")] pub u32);
impl Component for Durability {
    fn as_str(&self) -> String {
        format!("D{}", self.0)
    }
}

#[derive(Deserialize)]
// fixed tile furnishings
pub struct Fixture;
impl Component for Fixture {}

#[derive(Deserialize)]
pub struct Health(pub ValueMax);
impl Component for Health {}

#[derive(Deserialize)]
pub struct Interactive{
    pub kind: InteractionKind,
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

#[derive(Deserialize)]
pub struct Item;
impl Component for Item {}

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
pub struct Tile;
impl Component for Tile {}

#[derive(Deserialize)]
pub struct Hit(#[serde(deserialize_with="deserialize_random_u32")] pub u32);
impl Component for Hit {
    fn as_str(&self) -> String {
        format!("H{}", self.0)
    }
}

#[derive(Deserialize)]
pub struct Poison(#[serde(deserialize_with="deserialize_random_u32")] pub u32);
impl Component for Poison {
    fn as_str(&self) -> String {
        format!("P{}", self.0)
    }
}

#[derive(Deserialize)]
pub struct Stun(#[serde(deserialize_with="deserialize_random_u32")] pub u32);
impl Component for Stun {
    fn as_str(&self) -> String {
        format!("S{}", self.0)
    }
}

#[derive(Deserialize)]
pub struct Swing;
impl Component for Swing {
    fn as_str(&self) -> String {
        "Sw".to_string()
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
    pub items: [Option<Entity>; INVENTORY_SIZE],
    pub active_item: usize,
    pub gold: u32
}
impl Component for Player {}

pub struct Stunned(pub u32);
impl Component for Stunned {}

pub struct Poisoned(pub u32);
impl Component for Poisoned {}

pub struct Projectile {
    pub damage: u32,
    pub source: Vector2I,
    pub target: Vector2I
}
impl Component for Projectile {}

pub struct Position(pub Vector2I);
impl Component for Position {}

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
            "Consumable" => insert_single::<Consumable>(entity, world, component_data),
            "Durability" => insert_single::<Durability>(entity, world, component_data),
            "Fixture" => insert_single::<Fixture>(entity, world, component_data),
            "Health" => insert_single::<Health>(entity, world, component_data),
            "Interactive" => insert_single::<Interactive>(entity, world, component_data),
            "Item" => insert_single::<Item>(entity, world, component_data),
            "Loot" => insert_single::<Loot>(entity, world, component_data),
            "Hit" => insert_single::<Hit>(entity, world, component_data),
            "Poison" => insert_single::<Poison>(entity, world, component_data),
            "Stun" => insert_single::<Stun>(entity, world, component_data),
            "Swing" => insert_single::<Swing>(entity, world, component_data),
            "Obstacle" => insert_single::<Obstacle>(entity, world, component_data),
            "Tile" => insert_single::<Tile>(entity, world, component_data),
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

fn deserialize_random_u32<'de, D>(d: D) -> Result<u32, D::Error>
where D: Deserializer<'de> {
    match serde_yaml::Value::deserialize(d)? {
        serde_yaml::Value::Number(n) => 
            Ok(n.as_u64().ok_or(serde::de::Error::custom("Wrong value!"))? as u32),
        serde_yaml::Value::String(s) => {
            let parts = s.split('-').collect::<Vec<_>>();
            if parts.len() != 2 { Err(serde::de::Error::custom("Wrong value!")) }
            else {
                let mut rng = thread_rng();
                let a = parts[0].parse::<u32>().map_err(serde::de::Error::custom)?;
                let b = parts[1].parse::<u32>().map_err(serde::de::Error::custom)?;
                Ok(rng.gen_range(a..=b))
            }
        }
        _ => Err(serde::de::Error::custom("Wrong value!"))
    }
}

impl<'de> Deserialize<'de> for ValueMax {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let n = u32::deserialize(deserializer)?;
        Ok(ValueMax { current: n, max: n })
    }
}