use rogalik::storage::{Component, Entity, World};
use rogalik::math::vectors::Vector2I;
use serde::Deserialize;

use crate::actions::Action;
use crate::globals::INVENTORY_SIZE;
// use crate::items::ItemKind;

#[derive(Deserialize)]
pub enum AttackKind {
    Hit
}

// deserialized components
#[derive(Deserialize)]
pub struct Actor;
impl Component for Actor {}

#[derive(Deserialize)]
pub struct Attack {
    pub kind: AttackKind,
    pub value: u32
}
impl Component for Attack {}

#[derive(Deserialize)]
pub struct Durability {
    pub value: u32
}
impl Component for Durability {}

#[derive(Deserialize)]
// fixed tile furnishings
pub struct Fixture;
impl Component for Fixture {}

#[derive(Deserialize)]
pub struct Health(pub u32);
impl Component for Health {}

#[derive(Deserialize)]
pub struct Item;
impl Component for Item {}

#[derive(Deserialize)]
// actor cannot travel to a blocked tile
pub struct Obstacle;
impl Component for Obstacle {}

#[derive(Deserialize)]
pub struct Tile;
impl Component for Tile {}


// context-dependet components

pub struct Name (pub String);
impl Component for Name {}

pub struct Player {
    pub action: Option<Box<dyn Action>>,
    pub items: [Option<Entity>; INVENTORY_SIZE],
    pub active_item: usize,
    pub used_item: Option<Entity>
}
impl Component for Player {}

pub struct Paralyzed(pub u32);
impl Component for Paralyzed {}

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
            "Attack" => insert_single::<Attack>(entity, world, component_data),
            "Durability" => insert_single::<Durability>(entity, world, component_data),
            "Fixture" => insert_single::<Fixture>(entity, world, component_data),
            "Health" => insert_single::<Health>(entity, world, component_data),
            "Item" => insert_single::<Item>(entity, world, component_data),
            "Obstacle" => insert_single::<Obstacle>(entity, world, component_data),
            "Tile" => insert_single::<Tile>(entity, world, component_data),
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