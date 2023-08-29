use rand::prelude::*;
use std::collections::{HashMap, HashSet};

use rogalik::math::vectors::{Vector2I, ORTHO_DIRECTIONS};
use::rogalik::storage::{Entity, World};

use hike_data::GameData;

use crate::components::{Frozen, Position};
use crate::globals::{BOARD_SIZE, BOARD_SHIFT};
use crate::utils::{get_entities_at_position, spawn_with_position};

#[derive(Clone, Copy)]
pub enum ContentKind {
    Unit,
    Item
}

#[derive(Default)]
pub struct Board {
    pub level: u32,
    pub tiles: HashMap<Vector2I, Entity>,
    pub origin: Vector2I,
    pub next: HashMap<Vector2I, Vec<ContentKind>>
}
impl Board {
    pub fn new() -> Self {
        Board::default()
    }
    pub fn generate(&mut self, world: &mut World) {
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                let v = Vector2I::new(x as i32, y as i32);
                let entity = spawn_with_position(world, "Tile", v).unwrap();
                self.tiles.insert(v, entity);
            }
        }

        let _ = spawn_with_position(world, "Sword", Vector2I::new(2, 1));
        // let _ = spawn_with_position(world, "Jellyfish", Vector2I::new(4, 5));
        for dir in ORTHO_DIRECTIONS {
            self.next.insert(dir, get_next_content());
        }
    }
}

fn get_next_content() -> Vec<ContentKind> {
    let mut rng = thread_rng();
    let count = rng.gen_range(1..=BOARD_SIZE / 2);

    (0..count).map(|_| {
            match rng.gen_range(0.0..1.0) {
                a if a < 0.35 => ContentKind::Item,
                _ => ContentKind::Unit
            }
        })
        .collect()
}

fn spawn_tiles(world: &mut World, vs: &HashSet<Vector2I>, content: Vec<ContentKind>) {
    for v in vs {
        let entity = spawn_with_position(world, "Tile", *v).unwrap();
        world.get_resource_mut::<Board>().unwrap().tiles.insert(*v, entity);

    }

    let level = world.get_resource::<Board>().unwrap().level;

    let mut rng = thread_rng();
    let mut pool: Vec<_> = vs.iter().collect();
    for kind in content {
        let i = rng.gen_range(0..pool.len());
        let v = pool.remove(i);
        let name = get_content_item(
            kind,
            &world.get_resource::<GameData>().unwrap(),
            level
        );
        spawn_with_position(world, &name, *v);
    }
    if pool.len() > 0 && rng.gen_bool(0.5) {
        let i = rng.gen_range(0..pool.len());
        let v = pool.remove(i);
        spawn_with_position(world, "Rock", *v);
    }
}

fn get_content_item(kind: ContentKind, data: &GameData, level: u32) -> String {
    let base = match kind {
        ContentKind::Item => &data.items,
        ContentKind::Unit => &data.npcs
    };
    let pool = get_pool(data, &base, level);
    let mut rng = thread_rng();
    pool.choose(&mut rng).unwrap().to_string()
}

fn remove_tiles(world: &mut World, vs: &HashSet<Vector2I>) {
    world.get_resource_mut::<Board>().unwrap().tiles.retain(|k, _| !vs.contains(k));

    let to_remove = vs.iter()
        .map(
            |v| world.query::<Position>().iter()
                .filter(|i| i.get::<Position>().unwrap().0 == *v)
                .map(|i| i.entity)
                .collect::<Vec<_>>()
        )
        .flatten()
        .collect::<Vec<_>>();

    for entity in to_remove {
        world.despawn_entity(entity);
    }

}

pub fn shift_dir(world: &mut World, dir: Vector2I) {
    let origin = world.get_resource::<Board>().unwrap().origin;

    let (removed_vs, new_vs) = match dir {
        a if a == Vector2I::RIGHT => (
            get_rect(origin, BOARD_SHIFT as i32 - 1, BOARD_SIZE as i32 - 1),
            get_rect(origin + Vector2I::new(BOARD_SIZE as i32, 0), BOARD_SHIFT as i32 - 1, BOARD_SIZE as i32 - 1)
        ),
        a if a == Vector2I::LEFT => (
            get_rect(origin + Vector2I::new((BOARD_SIZE - 1) as i32, 0), -(BOARD_SHIFT as i32) + 1, BOARD_SIZE as i32 - 1),
            get_rect(origin - Vector2I::new(1, 0), -(BOARD_SHIFT as i32) + 1, BOARD_SIZE as i32 - 1)
        ),
        a if a == Vector2I::DOWN => (
            get_rect(origin, BOARD_SIZE as i32 - 1, BOARD_SHIFT as i32 - 1),
            get_rect(origin + Vector2I::new(0, BOARD_SIZE as i32), BOARD_SIZE as i32 - 1, BOARD_SHIFT as i32 - 1),
        ),
        a if a == Vector2I::UP => (
            get_rect(origin + Vector2I::new(0, BOARD_SIZE as i32 - 1), BOARD_SIZE as i32 - 1, -(BOARD_SHIFT as i32) + 1),
            get_rect(origin - Vector2I::new(0, 1), BOARD_SIZE as i32 - 1, -(BOARD_SHIFT as i32) + 1),
        ),
        _ => panic!("Wrong shift dir!")
    };

    
    // TODO cleanup
    world.get_resource_mut::<Board>().unwrap().origin += dir * BOARD_SHIFT as i32;
    let content = world.get_resource::<Board>().unwrap().next[&dir].clone();
    spawn_tiles(world, &new_vs, content);
    world.get_resource_mut::<Board>().unwrap().next.insert(dir, get_next_content());
    world.get_resource_mut::<Board>().unwrap().level += 1;

    remove_tiles(world, &removed_vs);
}

fn get_rect(origin: Vector2I, w: i32, h: i32) -> HashSet<Vector2I> {
    let b = Vector2I::new(origin.x + w, origin.y + h);
    let x0 = origin.x.min(b.x);
    let x1 = origin.x.max(b.x);
    let y0 = origin.y.min(b.y);
    let y1 = origin.y.max(b.y);
    
    (x0..=x1).map(|x|
            (y0..=y1).map(move |y| {
                Vector2I::new(x, y)
            })
        )
        .flatten()
        .collect()
}

fn get_pool<'a>(data: &'a GameData, base: &'a Vec<String>, level: u32) -> Vec<&'a String> {
    base.iter()
        .filter(|s| if let Some(data) = data.entities.get(*s) {
                data.min_level <= level
            } else {
                false
            } 
        )
        .collect()
}

// fn get_free_tile(world: &World) -> Vector2I {
//     let mut rng = thread_rng();
//     loop {
//         let v = Vector2I::new(
//             rng.gen_range(1..BOARD_SIZE-1) as i32,
//             rng.gen_range(1..BOARD_SIZE-1) as i32
//         );
//         if !get_entities_at_position(world, v).iter().any(
//                 |&e| world.get_component::<Obstacle>(e).is_some()
//         ) {
//             break v
//         }
//     }
// }
