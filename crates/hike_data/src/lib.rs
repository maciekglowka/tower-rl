use rand::prelude::*;
use serde::{Deserialize, Deserializer};
use serde_yaml;
use std::collections::HashMap;

use rogalik::engine::Color;

pub mod colors;

#[derive(Default)]
pub struct GameData {
    // all entity data, by name
    pub entities: HashMap<String, EntityData>,
    pub levels: HashMap<u32, LevelData>,
    pub discoverables: Vec<String>,
    pub items: Vec<String>,
    pub npcs: Vec<String>,
    pub fixtures: Vec<String>,
    pub traps: Vec<String>,
    pub discoverable_colors: HashMap<String,  (&'static str, Color)>
}
impl GameData {
    pub fn new() -> Self {
        GameData::default()
    }
    pub fn add_entities_from_str(&mut self, s: String) -> Vec<String> {
        let mut inserted_names = Vec::new();
        let values: serde_yaml::Value = serde_yaml::from_str(&s).expect("Could not parse Yaml data");
        for (k, v) in values.as_mapping().expect("Could not parse Yaml as mapping!").into_iter() {
            let data: EntityData = serde_yaml::from_value(v.clone()).expect(
                &format!("Incorrect value for: {:?}", k)
            );
            let name = k.as_str().expect(&format!("Incorrect string key: {:?}", k));
            if self.entities.insert(name.into(), data).is_some() {
                panic!("Duplicate data at: {}", name);
            }
            inserted_names.push(name.into());
        }
        inserted_names
    }
    pub fn add_level_data_from_str(&mut self, s: String) {
        self.levels = serde_yaml::from_str(&s).expect("Invalid level data!");
    }
    pub fn assign_discoverables(&mut self) {
        let mut rng = thread_rng();
        self.discoverable_colors = HashMap::new();
        let mut pool = colors::COLORS.to_vec();
        if pool.len() < self.discoverables.len() { panic!("Not enough colors in the pool!")};

        for name in self.discoverables.iter() {
            let i = rng.gen_range(0..pool.len());
            let color = pool.remove(i);
            self.discoverable_colors.insert(name.to_string(), color);
        };
    }
}

#[derive(Clone, Deserialize)]
pub struct EntityData {
    pub sprite: SpriteData,
    pub components: serde_yaml::Value,
    #[serde(default)]
    pub min_level: u32,
    pub spawn_chance: Option<f32>,
    #[serde(default)]
    pub score: i32
}

#[derive(Clone, Deserialize)]
pub struct SpriteData {
    pub atlas_name: String,
    pub index: u32,
    #[serde(default)]
    #[serde(deserialize_with="deserialize_color")]
    pub color: Color
}

#[derive(Clone, Debug, Deserialize)]
pub struct LevelData {
    #[serde(default)]
    pub required_items: Vec<String>,
    #[serde(default)]
    pub required_npcs: Vec<String>,
    #[serde(default)]
    pub required_fixtures: Vec<String>
}

fn deserialize_color<'de, D>(d: D) -> Result<Color, D::Error>
where D: Deserializer<'de> {
    match serde_yaml::Value::deserialize(d)? {
        serde_yaml::Value::Sequence(s) => {
            if s.len() != 4 { return Err(serde::de::Error::custom("Wrong value!")) }
            let mut c = s.iter()
                .map(|v| v.as_u64().expect("Incorrect color numerical value!") as u8);
            Ok(Color(c.next().unwrap(), c.next().unwrap(), c.next().unwrap(), c.next().unwrap()))
        },
        _ => Err(serde::de::Error::custom("Wrong value!"))
    }
}