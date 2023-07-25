use std::{
    collections::HashMap,
    f32::consts::PI
};
use rogalik::{
    math::vectors::{ORTHO_DIRECTIONS, Vector2I},
    storage::{Entity, World}
};

use crate::actions::{Action, PlaceBouy, Shoot, Travel};
use crate::board::Board;
use crate::components::{Blocker, Position};
use crate::wind::Wind;

pub trait Ability {
    fn get_possible_actions(
        &self,
        entity: Entity,
        world: &World
    ) -> HashMap<Vector2I, Box<dyn Action>>;
    fn description(&self) -> String;
}

pub struct Sailing;
impl Ability for Sailing {
    fn description(&self) -> String {
        "Sailing".into()
    }
    fn get_possible_actions(&self, entity: Entity, world: &World) -> HashMap<Vector2I, Box<dyn Action>> {
        let mut output = HashMap::new();
        let Some(wind) = world.get_resource::<Wind>() else { return output };
        let Some(position) = world.get_component::<Position>(entity) else { return output };

        for dir in ORTHO_DIRECTIONS {
            let dist = match wind.current().angle(&dir) {
                a if (PI - 0.1..PI + 0.1).contains(&a) => continue,
                a if (-0.1..0.1).contains(&a) => 2,
                _ => 1
            };
            if let Some(target) = get_furthest_traversible(position.0, dir, dist, world) {
                output.insert(target, Box::new(Travel { entity, target }));
            }
        }
        if output.len() == 0 {
            // failsafe
            output.insert(position.0, Box::new(Travel { entity, target: position.0 }));
        }
        output
    }
}

pub struct Swimming;
impl Ability for Swimming {
    fn description(&self) -> String {
        "Swimming".into()
    }
    fn get_possible_actions(&self, entity: Entity, world: &World) -> HashMap<Vector2I, Box<dyn Action>> {
        let mut output = HashMap::new();
        let Some(position) = world.get_component::<Position>(entity) else { return output };

        for dir in ORTHO_DIRECTIONS {
            let target = position.0 + dir;
            if is_tile_traversible(target, world) {
                output.insert(target, Box::new(Travel { entity, target }));
            }
        }
        output.insert(position.0, Box::new(Travel { entity, target: position.0 }));
        output
    }
}

pub struct Cannons {
    pub dist: u32,
    pub damage: u32
}
impl Ability for Cannons {
    fn description(&self) -> String {
        "Cannons".into()
    }
    fn get_possible_actions(&self, entity: Entity, world: &World) -> HashMap<Vector2I, Box<dyn Action>> {
        let mut output = HashMap::new();
        let Some(position) = world.get_component::<Position>(entity) else { return output };

        for dir in ORTHO_DIRECTIONS {
            output.insert(position.0 + dir, Box::new(Shoot {
                source: position.0,
                dir,
                dist: self.dist,
                damage: self.damage 
            }));
        }
        output
    }
}

pub struct Buoy {
    pub health: u32
}
impl Ability for Buoy {
    fn description(&self) -> String {
        "Buoy".into()
    }
    fn get_possible_actions(&self, entity: Entity, world: &World) -> HashMap<Vector2I, Box<dyn Action>> {
        let mut output = HashMap::new();
        let Some(position) = world.get_component::<Position>(entity) else { return output };

        for dir in ORTHO_DIRECTIONS {
            let target = position.0 + dir;
            if is_tile_traversible(target, world) {
                output.insert(target, Box::new(PlaceBouy { position: target, health: self.health }));
            }
        }
        output
    }
}

fn is_tile_traversible(v: Vector2I, world: &World) -> bool {
    let Some(board) = world.get_resource::<Board>() else { return false };
    if !board.tiles.contains_key(&v) { return false }
    for item in world.query::<Position>().with::<Blocker>().iter() {
        if item.get::<Position>().unwrap().0 == v { return false };
    }
    true
}

fn get_furthest_traversible(source: Vector2I, dir: Vector2I, max_dist: u32, world: &World) -> Option<Vector2I> {
    let blocker_positions = world.query::<Blocker>().with::<Position>()
        .iter()
        .map(|i| i.get::<Position>().unwrap().0)
        .collect::<Vec<_>>();

    let board = world.get_resource::<Board>()?;

    // find target - eg. the max dist or first blocker on the way
    let mut target = source;
    for _ in 1..=max_dist {
        let t = target + dir;
        if !board.tiles.contains_key(&t) { break };
        if blocker_positions.contains(&t) { break };
        target = t;
    }
    if target == source {
        None
    } else {
        Some(target)
    }
}