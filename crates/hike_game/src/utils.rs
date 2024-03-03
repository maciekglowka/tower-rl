use rand::prelude::*;
use rogalik::{
    math::vectors::{Vector2i, get_line},
    storage::{Entity, World}
};
use serde::{Deserialize, Deserializer, Serializer};
use serde::de::Visitor;

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
    if !d.is_human_readable() {
        return u32::deserialize(d)
    }

    // Ok(d.deserialize_any(RandomU32Visitsor).unwrap())
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

// struct RandomU32Visitor;
// impl<'de> Visitor<'de> for RandomU32Visitor {
//     type Value = u32;

//     fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//         formatter.write_str("number or {}-{} string")
//     }
//     // fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
//     // where E: serde::de::Error
//     // {
//     //     println!("U32");
//     //     Ok(v)
//     // }
//     // fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
//     // where E: serde::de::Error
//     // {
//     //     println!("U64");
//     //     Ok(v as u32)
//     // }
//     fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
//     where E: serde::de::Error
//     {
//         println!("String");
//         let parts = v.split('-').collect::<Vec<_>>();
//         if parts.len() != 2 { Err(serde::de::Error::custom("Wrong value!")) }
//         else {
//             let mut rng = thread_rng();
//             let a = parts[0].parse::<u32>().map_err(serde::de::Error::custom)?;
//             let b = parts[1].parse::<u32>().map_err(serde::de::Error::custom)?;
//             Ok(rng.gen_range(a..=b))
//         }
//     }
// }

pub fn deserialize_as_none<'de, D, T>(d: D) -> Result<Option<T>, D::Error>
where D: Deserializer<'de> {
    Ok(None)
}

pub fn serialize_as_none<S, T>(_: &T, s:S) -> Result<S::Ok, S::Error>
where S: Serializer {
    s.serialize_none()
}

// pub mod random_u32 {
//     use rand::prelude::*;
//     use serde::{Serializer, Deserializer, Deserialize};

//     pub fn serialize<S>(value: &u32, s:S) -> Result<S::Ok, S::Error>
//     where S: Serializer {
//         s.collect_str(value)
//     }
//     pub fn deserialize<'de, D>(d: D) -> Result<u32, D::Error>
//     where D: Deserializer<'de> {
//         println!("RANDOM u32");
//         print!("{:?}", d.is_human_readable());
//         let s = String::deserialize(d)?;
//         let parts = s.split('-').collect::<Vec<_>>();
//         match parts.len() {
//             1 => {
//                 parts[0].parse::<u32>().map_err(serde::de::Error::custom)
//             },
//             2 => {
//                 let mut rng = thread_rng();
//                 let a = parts[0].parse::<u32>().map_err(serde::de::Error::custom)?;
//                 let b = parts[1].parse::<u32>().map_err(serde::de::Error::custom)?;
//                 Ok(rng.gen_range(a..=b))
//             },
//             _ => Err(serde::de::Error::custom("Wrong random u32 value!"))
//         }
//         // } != 2 { Err(serde::de::Error::custom("Wrong value!")) }
//         // else {
//         //     let mut rng = thread_rng();
//         //     let a = parts[0].parse::<u32>().map_err(serde::de::Error::custom)?;
//         //     let b = parts[1].parse::<u32>().map_err(serde::de::Error::custom)?;
//         //     Ok(rng.gen_range(a..=b))
//         // }
//         // Err(serde::de::Error::custom("Wrong random u32 value!"))
//     }
// }