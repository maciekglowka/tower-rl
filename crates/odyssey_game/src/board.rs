use rand::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};

use rogalik::math::vectors::{Vector2I, ORTHO_DIRECTIONS, visible_tiles};
use::rogalik::storage::{Entity, World};

use crate::components::{Obstacle, PlayerCharacter, Position, Tile, Spawner, ViewBlocker};
use crate::globals::{BOARD_SIZE, VIEW_RANGE};
use crate::utils::{get_entities_at_position, spawn_with_position};

pub struct Board {
    pub tiles: HashMap<Vector2I, Entity>,
    pub visible: HashSet<Vector2I>
}
impl Board {
    pub fn new() -> Self {
        Board { tiles: HashMap::new(), visible: HashSet::new() }
    }
    pub fn generate(&mut self, world: &mut World) {
        let layout = BoardLayout::generate();
        for v in layout.tiles {
            let entity = spawn_with_position(world, "Tile", v).unwrap();
            self.tiles.insert(v, entity);
        }

        for v in layout.rocks {
            let _ = spawn_with_position(world, "Rock", v);
        }
        let _ = spawn_with_position(
            world, "Vortex", Vector2I::new(BOARD_SIZE as i32 - 1, BOARD_SIZE as i32 - 1)
        );

        let mut rng = thread_rng();
        for _ in 0..3{
            let v = Vector2I::new(
                rng.gen_range(1..BOARD_SIZE-1) as i32,
                rng.gen_range(1..BOARD_SIZE-1) as i32
            );
            let name = if rng.gen_bool(0.5) {
                "Debris"
            } else {
                "Survivor"
            };
            let _ = spawn_with_position(world, name, v);
        }
    }
}

struct BoardLayout {
    pub tiles: HashSet<Vector2I>,
    pub rocks: HashSet<Vector2I>
}
impl BoardLayout {
    pub fn generate() -> Self {
        let mut tiles = HashSet::new();

        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                tiles.insert(Vector2I::new(x as i32, y as i32));
            }
        }

        let mut rng = thread_rng();
        let rocks = (0..16).map(|_| Vector2I::new(
                rng.gen_range(1..BOARD_SIZE-1) as i32,
                rng.gen_range(1..BOARD_SIZE-1) as i32
            ))
            .collect();

        BoardLayout { tiles, rocks }
    }
}

pub fn update_visibility(world: &mut World) {
    if let Some(player) = world.query::<PlayerCharacter>().with::<Position>().iter().next() {
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

pub fn create_spawner(world: &mut World) {
    let position = get_free_tile(world);
    let Some(entity) = spawn_with_position(world, "Spawner", position) else { return };
    let mut rng = thread_rng();
    let _ = world.insert_component(entity, Spawner {
        target: if rng.gen_bool(0.6) { "Jellyfish".into() } else { "Shark".into() },
        countdown: 2
    });
}

fn get_free_tile(world: &World) -> Vector2I {
    let mut rng = thread_rng();
    loop {
        let v = Vector2I::new(
            rng.gen_range(1..BOARD_SIZE-1) as i32,
            rng.gen_range(1..BOARD_SIZE-1) as i32
        );
        if !get_entities_at_position(world, v).iter().any(
                |&e| world.get_component::<Obstacle>(e).is_some()
        ) {
            break v
        }
    }
}
