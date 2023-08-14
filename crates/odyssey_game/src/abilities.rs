use std::{
    collections::HashMap,
    f32::consts::PI
};
use rogalik::{
    math::vectors::{ORTHO_DIRECTIONS, Vector2I},
    storage::{Entity, World}
};
use serde::Deserialize;

use crate::actions::{Action, MeleeHit, PickItem, PlaceBouy, Travel};
use crate::board::Board;
use crate::components::{Obstacle, Health, PlayerCharacter, Position};
use crate::utils::{are_hostile, get_entities_at_position};

#[derive(Clone, Copy, Deserialize)]
pub enum AbilityKind {
    Buoy,
    Melee,
    Swim,
}

#[derive(Clone, Copy, Deserialize)]
pub struct Ability {
    pub kind: AbilityKind,
    pub value: Option<u32>,
    pub cooldown: Option<Cooldown>
}
impl Ability {
    pub fn as_str(&self) -> &str {
        match self.kind {
            AbilityKind::Buoy => "Buoy",
            AbilityKind::Melee => "Melee",
            AbilityKind::Swim => "Sail",
        }
    }
}

#[derive(Clone, Copy, Deserialize)]
pub struct Cooldown {
    pub max: u32,
    pub current: u32
}

pub fn get_possible_actions(entity: Entity, ability: &Ability, world: &World) -> HashMap<Vector2I, Box<dyn Action>> {
    get_action_factory(ability)(entity, ability, world)
}

type ActionFactory = fn(Entity, &Ability, &World) -> HashMap<Vector2I, Box<dyn Action>>;

fn get_action_factory(ability: &Ability) -> ActionFactory {
    match ability.kind {
        AbilityKind::Buoy => buoy_factory,
        AbilityKind::Melee => melee_factory,
        AbilityKind::Swim => swim_factory,
    }
}

fn melee_factory(entity: Entity, ability: &Ability, world: &World) -> HashMap<Vector2I, Box<dyn Action>> {
    let Some(position) = world.get_component::<Position>(entity) else { return HashMap::new() };
    
    let v = ORTHO_DIRECTIONS.iter()
        .map(|d| position.0 + *d)
        .filter_map(|d| {
            if get_entities_at_position(world, d).iter().any(
                |e| world.get_component::<Health>(*e).is_some()
            ) { Some((d, Box::new(
                    MeleeHit { entity, target: d, value: ability.value.unwrap_or(1) }
                ) as Box<dyn Action>
            ))}
            else { None }
        });
        
    HashMap::from_iter(v)
}

fn swim_factory(entity: Entity, ability: &Ability, world: &World) -> HashMap<Vector2I, Box<dyn Action>> {
    let mut output = HashMap::new();
    let Some(position) = world.get_component::<Position>(entity) else { return output };

    for dir in ORTHO_DIRECTIONS {
        for dist in 0..=ability.value.unwrap_or(1) {
            if let Some(target) = get_furthest_traversible(position.0, dir, dist, world) {
                output.insert(target, Box::new(Travel { entity, target }));
            }
        }
    }

    if world.get_component::<PlayerCharacter>(entity).is_some() {
        output.insert(
            position.0, Box::new(PickItem { entity })
        );
    }

    output
}

fn buoy_factory(entity: Entity, ability: &Ability, world: &World) -> HashMap<Vector2I, Box<dyn Action>> {
    let mut output = HashMap::new();
    let Some(position) = world.get_component::<Position>(entity) else { return output };

    for dir in ORTHO_DIRECTIONS {
        if let Some(target) = get_furthest_traversible(position.0, dir, 1, world) {
            output.insert(target, Box::new(PlaceBouy { 
                position: target, health: ability.value.unwrap_or(1) 
            }));
        }
    }
    output
}

// pub trait Ability {
//     fn get_possible_actions(
//         &self,
//         entity: Entity,
//         world: &World
//     ) -> HashMap<Vector2I, Box<dyn Action>>;
//     fn description(&self) -> String;
// }

// pub struct Sailing;
// impl Ability for Sailing {
//     fn description(&self) -> String {
//         "Sailing".into()
//     }
//     fn get_possible_actions(&self, entity: Entity, world: &World) -> HashMap<Vector2I, Box<dyn Action>> {
//         let mut output = HashMap::new();
//         let Some(wind) = world.get_resource::<Wind>() else { return output };
//         let Some(position) = world.get_component::<Position>(entity) else { return output };

//         for dir in ORTHO_DIRECTIONS {
//             let dist = match wind.current().angle(&dir) {
//                 a if (PI - 0.1..PI + 0.1).contains(&a) => continue,
//                 a if (-0.1..0.1).contains(&a) => 2,
//                 _ => 1
//             };
//             if let Some(target) = get_furthest_traversible(position.0, dir, dist, world) {
//                 output.insert(target, Box::new(Travel { entity, target }));
//             }
//         }
//         if output.len() == 0 {
//             // failsafe
//             output.insert(position.0, Box::new(Travel { entity, target: position.0 }));
//         }
//         output
//     }
// }

// pub struct Swimming {
//     pub dist: u32
// }
// impl Ability for Swimming {
//     fn description(&self) -> String {
//         "Swimming".into()
//     }
//     fn get_possible_actions(&self, entity: Entity, world: &World) -> HashMap<Vector2I, Box<dyn Action>> {
//         let mut output = HashMap::new();
//         let Some(position) = world.get_component::<Position>(entity) else { return output };

//         for dir in ORTHO_DIRECTIONS {
//             for dist in 0..=self.dist {
//                 if let Some(target) = get_furthest_traversible(position.0, dir, dist, world) {
//                     output.insert(target, Box::new(Travel { entity, target }));
//                 }
//             }
//         }
//         output.insert(position.0, Box::new(Travel { entity, target: position.0 }));
//         output
//     }
// }

// pub struct Cannons {
//     pub dist: u32,
//     pub damage: u32
// }
// impl Ability for Cannons {
//     fn description(&self) -> String {
//         "Cannons".into()
//     }
//     fn get_possible_actions(&self, entity: Entity, world: &World) -> HashMap<Vector2I, Box<dyn Action>> {
//         let mut output = HashMap::new();
//         let Some(position) = world.get_component::<Position>(entity) else { return output };

//         for dir in ORTHO_DIRECTIONS {
//             output.insert(position.0 + dir, Box::new(Shoot {
//                 source: position.0,
//                 dir,
//                 dist: self.dist,
//                 damage: self.damage 
//             }));
//         }
//         output
//     }
// }

// pub struct Buoy {
//     pub health: u32
// }
// impl Ability for Buoy {
//     fn description(&self) -> String {
//         "Buoy".into()
//     }
//     fn get_possible_actions(&self, entity: Entity, world: &World) -> HashMap<Vector2I, Box<dyn Action>> {
//         let mut output = HashMap::new();
//         let Some(position) = world.get_component::<Position>(entity) else { return output };

//         for dir in ORTHO_DIRECTIONS {
//             let target = position.0 + dir;
//             if is_tile_traversible(target, world) {
//                 output.insert(target, Box::new(PlaceBouy { position: target, health: self.health }));
//             }
//         }
//         output
//     }
// }

// pub struct Abordage {
//     pub damage: u32
// }
// impl Ability for Abordage {
//     fn description(&self) -> String {
//         "Abordage".into()
//     }
//     fn get_possible_actions(
//             &self,
//             entity: Entity,
//             world: &World
//         ) -> HashMap<Vector2I, Box<dyn Action>> {
//         let Some(position) = world.get_component::<Position>(entity) else { return HashMap::new() };
    
//         let v = ORTHO_DIRECTIONS.iter()
//             .map(|d| position.0 + *d)
//             .map(|v| {
//                 get_entities_at_position(world, v).iter()
//                     .filter_map(|e| if are_hostile(entity, *e, world) {
//                             Some((v, Box::new(
//                                 MeleeHit { entity, target: *e, value: self.damage }
//                             ) as Box<dyn Action>))
//                         } else {
//                             None
//                         }
//                     )
//                     .collect::<Vec<_>>()
//             })
//             .flatten();
//         HashMap::from_iter(v)
//     }
// }

// fn is_tile_traversible(v: Vector2I, world: &World) -> bool {
//     let Some(board) = world.get_resource::<Board>() else { return false };
//     if !board.tiles.contains_key(&v) { return false }
//     for item in world.query::<Position>().with::<Blocker>().iter() {
//         if item.get::<Position>().unwrap().0 == v { return false };
//     }
//     true
// }

fn get_furthest_traversible(source: Vector2I, dir: Vector2I, max_dist: u32, world: &World) -> Option<Vector2I> {
    let obstacle_positions = world.query::<Obstacle>().with::<Position>()
        .iter()
        .map(|i| i.get::<Position>().unwrap().0)
        .collect::<Vec<_>>();

    let board = world.get_resource::<Board>()?;

    // find target - eg. the max dist or first blocker on the way
    let mut target = source;
    for _ in 1..=max_dist {
        let t = target + dir;
        if !board.tiles.contains_key(&t) { break };
        if obstacle_positions.contains(&t) { break };
        target = t;
    }
    if target == source {
        None
    } else {
        Some(target)
    }
}