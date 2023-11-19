use rand::prelude::*;
use rogalik::{
    math::vectors::{Vector2i, get_line},
    storage::{Entity, World}
};
use serde::{Deserialize, Deserializer};

use hike_data::GameData;

use crate::components::{Actor, Name, Player, Position, ViewBlocker, insert_data_components};
use crate::globals::VIEW_RANGE;
use crate::structs::Attitude;


pub fn is_hostile(entity: Entity, world: &World) -> bool {
    if let Some(actor) = world.get_component::<Actor>(entity) {
        return actor.attitude == Attitude::Hostile;
    }
    false
}

pub fn visibility(world: &World, a: Vector2i, b: Vector2i) -> bool {
    let line = get_line(a, b);
    if line.len() <= 2 { return true }
    if line.len() > VIEW_RANGE as usize { return false }
    for v in line[1..line.len() - 1].iter() {
        if get_entities_at_position(world, *v).iter()
            .any(|&e| world.get_component::<ViewBlocker>(e).is_some()) {
                return  false;
            }
    }
    true
}

pub fn get_entities_at_position(world: &World, v: Vector2i) -> Vec<Entity> {
    let query =  world.query::<Position>().build();
    query.iter::<Position>().zip(query.entities())
        .filter(|(p, _)| p.0 == v)
        .map(|(_, e)| *e)
        .collect()
}

pub fn spawn_with_position(
    world: &mut World,
    name: &str,
    position: Vector2i
) -> Option<Entity> {
    let entity = world.spawn_entity();
    let _ = world.insert_component(entity, Name(name.into()));
    let _ = world.insert_component(entity, Position(position));

    let data = world.get_resource::<GameData>()?
        .entities.get(name).expect(&format!("Could not spawn: {} - no data found!", name)).clone();
    insert_data_components(entity, world, &data.components);

    Some(entity)
}

pub fn deserialize_random_u32<'de, D>(d: D) -> Result<u32, D::Error>
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