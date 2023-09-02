use rand::prelude::*;
use std::collections::{HashMap, HashSet};

use rogalik::math::vectors::{Vector2I, ORTHO_DIRECTIONS, visible_tiles};
use::rogalik::storage::{Entity, World};

use hike_data::GameData;

use crate::components::{Position, Obstacle, Player, ViewBlocker, Tile};
use crate::globals::{BOARD_SIZE, VIEW_RANGE};
use crate::utils::{get_entities_at_position, spawn_with_position};

#[derive(Default)]
pub struct Board {
    pub level: u32,
    pub tiles: HashMap<Vector2I, Entity>,
    pub exit: bool,
    pub visible: HashSet<Vector2I>
}
impl Board {
    pub fn new(level: u32) -> Self {
        Board {
            level,
            ..Default::default()
        }
    }
    pub fn generate(&mut self, world: &mut World) {
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                let v = Vector2I::new(x as i32, y as i32);
                let entity = spawn_with_position(world, "Tile", v).unwrap();
                self.tiles.insert(v, entity);
            }
        }

        for v in get_wall_layout() {
            let _ = spawn_with_position(world, "Rock", v);
        }
    }
    pub fn is_exit(&self) -> bool {
        self.exit
    }
}

pub fn furnish_board(world: &mut World) {
    let _ = spawn_with_position(world, "Stair", get_free_tile(world).unwrap());
    let _ = spawn_with_position(world, "Sword", get_free_tile(world).unwrap());

    let data = world.get_resource::<GameData>().unwrap();
    let board = world.get_resource::<Board>().unwrap();

    let npc_pool = get_pool(
        &data, &data.npcs, board.level
    );
    drop(data);
    drop(board);
    let mut rng = thread_rng();
    for _ in 0..3 {
        let npc = &npc_pool.choose_weighted(&mut rng, |a| a.0).unwrap().1;
        let Some(v) = get_free_tile(world) else { continue };
        let _ = spawn_with_position(world, npc, v);
    }

    let _ = spawn_with_position(world, "Workshop", get_free_tile(world).unwrap());
    let _ = spawn_with_position(world, "Food", get_free_tile(world).unwrap());
}

fn get_wall_layout() -> HashSet<Vector2I> {
    let size = 4;
    let sectors = BOARD_SIZE / size;
    let chance = 0.75;
    let mut walls = HashSet::new();
    let mut rng = thread_rng();

    for x in 0..sectors {
        for y in 0..sectors {
            let vertical = (x + y) % 2 != 0;
            let base_offset_amount = [1_i32, 2_i32].choose(&mut rng).unwrap();
            let base_offset = if vertical { Vector2I::new(*base_offset_amount, 0) } else { Vector2I::new(0, *base_offset_amount) };
            let offset = if vertical { Vector2I::new(0, 1) } else { Vector2I::new(1, 0) };
            let base = Vector2I::new(x as i32 * size as i32, y as i32 * size as i32) + base_offset;
            for i in 0..size {
                if !rng.gen_bool(chance) { continue; }
                walls.insert(base + offset * i as i32);
            }
        }
    }
    walls
}

fn get_pool<'a>(data: &'a GameData, base: &'a Vec<String>, level: u32) -> Vec<(f32, String)> {
    base.iter()
        .filter_map(|name| data.entities.get(name).map(|d| (name, d)))
        .filter(|(name, d)| d.min_level <= level)
        .map(|(name, d)| (d.spawn_chance.unwrap_or(1.), name.to_string()))
        .collect()
}

pub fn get_free_tile(world: &World) -> Option<Vector2I> {
    let mut rng = thread_rng();
    let board = world.get_resource::<Board>()?;
    let tiles = board.tiles.keys()
        .filter(|&&v| !get_entities_at_position(world, v)
            .iter()
            .any(|&e| world.get_component::<Tile>(e).is_none())
        );
    tiles.choose(&mut rng).map(|&v| v)
}

pub fn update_visibility(world: &mut World) {
    if let Some(player) = world.query::<Player>().with::<Position>().iter().next() {
        let Some(mut board) = world.get_resource_mut::<Board>() else { return };
        let position = player.get::<Position>().unwrap().0;
        let blockers = world.query::<Position>().with::<ViewBlocker>().iter()
            .map(|i| i.get::<Position>().unwrap().0)
            .collect();
        let currently_visible = visible_tiles(
            position,
            &HashSet::from_iter(board.tiles.keys().map(|&v| v)),
            &blockers,
            VIEW_RANGE
        );
        board.visible.extend(currently_visible);
    }
}
