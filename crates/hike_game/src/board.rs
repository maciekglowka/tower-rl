use rand::prelude::*;
use std::collections::{HashMap, HashSet};

use rogalik::math::vectors::{Vector2i, ORTHO_DIRECTIONS, visible_tiles};
use::rogalik::storage::{Entity, World};

use hike_data::GameData;

use crate::components::{Position, Player, ViewBlocker, Tile};
use crate::globals::{BOARD_SIZE, VIEW_RANGE, LEVEL_COUNT};
use crate::utils::{get_entities_at_position, spawn_with_position};

#[derive(Default)]
pub struct Board {
    pub level: u32,
    pub tiles: HashMap<Vector2i, Entity>,
    pub exit: bool,
    pub player_spawn: Vector2i,
    pub visible: HashSet<Vector2i>
}
impl Board {
    pub fn new(level: u32) -> Self {
        Board {
            level,
            ..Default::default()
        }
    }
    pub fn generate(&mut self, world: &mut World) {
        let mut tile_pool = tile_range(
            Vector2i::ZERO,
            Vector2i::new(BOARD_SIZE as i32 - 1, BOARD_SIZE as i32 - 1)
        );
        for v in tile_pool.iter() {
            let entity = spawn_with_position(world, "Tile", *v).unwrap();
            self.tiles.insert(*v, entity);
        }

        let layout = get_bsp_layout();
        for v in layout.0.iter() {
            let _ = spawn_with_position(world, "Wall", *v);
        }
        let mut rng = thread_rng();
        for v in layout.1.iter() {
            if !rng.gen_bool(0.5) { continue };
            let _ = spawn_with_position(world, "Closed_Door", *v);
        }

        // remove walls
        tile_pool.retain(|v| !layout.0.contains(v));
        // remvove doors and adjacent
        tile_pool.retain(|v| !layout.1.iter().any(|d| d.manhattan(*v) <= 1));

        if self.level < LEVEL_COUNT {
            let _ = spawn_with_position(world, "Stair", get_random_tile(&mut tile_pool, None).unwrap());
        }

        self.player_spawn = get_random_tile(&mut tile_pool, None).unwrap();

        if self.level == LEVEL_COUNT {
            let v = get_random_tile(&mut tile_pool, Some((self.player_spawn, 6))).unwrap();
            let _ = spawn_with_position(world, "Second_Book_of_Poetics", v);
        }

        let pieces = if let Some(data) = world.get_resource::<GameData>() {
            get_board_pieces(self.level, &data) 
        } else { return };

        for name in pieces {
            let Some(v) = get_random_tile(&mut tile_pool, Some((self.player_spawn, 3))) else { continue };
            let _ = spawn_with_position(world, &name, v);
        }
        
        self.tiles.extend(create_bounds(world));

    }
    pub fn is_exit(&self) -> bool {
        self.exit
    }
}

fn create_bounds(world: &mut World) -> HashMap<Vector2i, Entity> {
    let mut entities = HashMap::new();
    for x in -1..=BOARD_SIZE as i32 {
        for y in -1..=BOARD_SIZE as i32 {
            if x >=0 && y >= 0 && x < BOARD_SIZE as i32 && y < BOARD_SIZE as i32 { continue }
            let v = Vector2i::new(x, y);
            let entity = spawn_with_position(world, "Tile", v).unwrap();
            let _ = spawn_with_position(world, "Wall", v).unwrap();
            entities.insert(v, entity);
        }
    }
    entities
}

// fn spawn_npcs(
//     world: &mut World,
//     tile_pool: &mut HashSet<Vector2i>,
//     level: u32
// ) {
//     let npc_pool = if let Some(data) = world.get_resource::<GameData>() {
//         get_entity_pool(&data, &data.npcs, level)
//     } else { return };

//     let mut rng = thread_rng();
//     for _ in 0..rng.gen_range(2..=4) {
//         let npc = &npc_pool.choose_weighted(&mut rng, |a| a.0).unwrap().1;
//         let Some(v) = get_random_tile(tile_pool) else { continue };
//         let _ = spawn_with_position(world, npc, v);
//     }
// }

// fn spawn_items(
//     world: &mut World,
//     tile_pool: &mut HashSet<Vector2i>,
//     level: u32
// ) {
//     let item_pool = if let Some(data) = world.get_resource::<GameData>() {
//         get_entity_pool(&data, &data.items, level)
//     } else { return };

//     let mut rng = thread_rng();
//     for _ in 0..rng.gen_range(1..=3) {
//         let item = &item_pool.choose_weighted(&mut rng, |a| a.0).unwrap().1;
//         let Some(v) = get_random_tile(tile_pool) else { continue };
//         let _ = spawn_with_position(world, item, v);
//     }
// }

// fn spawn_fixtures(
//     world: &mut World,
//     tile_pool: &mut HashSet<Vector2i>,
//     level: u32
// ) {
//     let mut rng = thread_rng();
//     if level % 2 != 0 { return };
//     let fixture_pool = if let Some(data) = world.get_resource::<GameData>() {
//         get_entity_pool(&data, &data.fixtures, level)
//     } else { return };

//     for _ in 0..1 {
//         let fixture = &fixture_pool.choose_weighted(&mut rng, |a| a.0).unwrap().1;
//         let Some(v) = get_random_tile(tile_pool) else { continue };
//         let _ = spawn_with_position(world, fixture, v);
//     }
// }

pub fn update_visibility(world: &mut World) {
    if let Some(position) = world.query::<Player>().with::<Position>().build().single::<Position>() {
        let Some(mut board) = world.get_resource_mut::<Board>() else { return };
        let blockers = world.query::<ViewBlocker>().with::<Position>().build().iter::<Position>()
            .map(|p| p.0)
            .collect::<HashSet<_>>();
        let currently_visible = visible_tiles(
            position.0,
            &HashSet::from_iter(board.tiles.keys().map(|&v| v)),
            &blockers,
            VIEW_RANGE
        );
        board.visible.extend(currently_visible);
    }
}

struct Room {
    pub a: Vector2i,
    pub b: Vector2i,
    pub doors: Vec<Vector2i>
}
impl Room {
    pub fn tiles(&self) -> HashSet<Vector2i> {
        tile_range(self.a, self.b)
    }
}

fn get_bsp_layout() -> (HashSet<Vector2i>, HashSet<Vector2i>) {
    // return (walls, doors)
    'outer: loop {
        let base = Room {
            a: Vector2i::ZERO,
            b: Vector2i::new(BOARD_SIZE as i32 - 1, BOARD_SIZE as i32 - 1),
            doors: Vec::new()
        };
        let mut wall_tiles = base.tiles();
        let rooms = divide_room(base);
        if rooms.len() < 3 { continue; }
        let mut doors = HashSet::new();
        for r in rooms.iter() {
            doors.extend(&r.doors);
            for v in r.tiles() {
                wall_tiles.remove(&v);
            }
            for v in r.doors.iter() {
                wall_tiles.remove(&v);
            }
        }

        // extra door validation for safety
        for door in doors.iter() {
            let n = ORTHO_DIRECTIONS.iter()
                .filter(|dir| wall_tiles.contains(&(*door + **dir)))
                .count();
            if n > 2 { continue 'outer }
        }

        return (wall_tiles, doors);
    }
}

fn divide_room(r: Room) -> Vec<Room> {
    let dx = r.b.x - r.a.x;
    let dy = r.b.y - r.a.y;
    if  dx < 4 && dy < 4 { return vec![r] }
    let vertical = dx < dy;
    let mut rng = thread_rng();

    let split_val = if vertical { rng.gen_range(r.a.y + 2..r.b.y -1 ) }
        else { rng.gen_range(r.a.x + 2..r.b.x -1 ) };

    // check existing door collision
    if vertical && r.doors.iter().any(|&v| v.x == split_val) { return vec![r] }
        else {
            if r.doors.iter().any(|&v| v.y == split_val) { return vec![r] }
        }

    let corner_a = if vertical { Vector2i::new(r.b.x, split_val - 1) } else { Vector2i::new(split_val - 1, r.b.y) };
    let corner_b = if vertical { Vector2i::new(r.a.x, split_val + 1) } else { Vector2i::new(split_val + 1, r.a.y) };

    let mut doors = r.doors.clone();
    let door = get_bsp_door(vertical, split_val, r.a, r.b);
    
    // consider extra door for large rooms
    if dx.max(dy) > 5 && rng.gen_bool(0.75) {
        let extra_door = get_bsp_door(vertical, split_val, r.a, r.b);
        if extra_door.manhattan(door) > 1 { doors.push(extra_door) };
    }
    doors.push(door);

    let room_a = Room { a: r.a, b: corner_a, doors: doors.clone() };
    let room_b = Room { a: corner_b, b: r.b, doors };
    let mut res = divide_room(room_a);
    res.extend(divide_room(room_b));
    res
}

fn get_bsp_door(vertical: bool, split_val: i32, a: Vector2i, b: Vector2i) -> Vector2i {
    let mut rng = thread_rng();
    if vertical { Vector2i::new(rng.gen_range(a.x..=b.x), split_val) }
        else { Vector2i::new(split_val, rng.gen_range(a.y..=b.y))}
}

fn tile_range(a: Vector2i, b: Vector2i) -> HashSet<Vector2i> {
    (a.x..=b.x).map(
            |x| (a.y..=b.y).map(move |y| Vector2i::new(x, y))
        )
        .flatten()
        .collect()
}

fn get_random_tile(pool: &mut HashSet<Vector2i>, dist: Option<(Vector2i, u32)>) -> Option<Vector2i> {
    let mut rng = thread_rng();

    let v = match dist {
        Some((p, d)) => {
            *pool.iter()
                .filter(|a| a.manhattan(p) >= d as i32)
                .choose(&mut rng)?
        },
        None => *pool.iter().choose(&mut rng)?
    };
    // let v = *pool.iter().choose(&mut rng)?;
    pool.remove(&v);
    Some(v)
}

// pub fn get_free_tile(world: &World) -> Option<Vector2i> {
//     let mut rng = thread_rng();
//     let board = world.get_resource::<Board>()?;
//     let tiles = board.tiles.keys()
//         .filter(|&&v| !get_entities_at_position(world, v)
//             .iter()
//             .any(|&e| world.get_component::<Tile>(e).is_none())
//         );
//     tiles.choose(&mut rng).map(|&v| v)
// }

fn get_target_score(level: u32) -> i32 {
    (level as f32 * 1.5) as i32
}

fn get_entity_pool<'a>(data: &'a GameData, base: &'a Vec<String>, level: u32) -> Vec<(f32, String)> {
    base.iter()
        .filter_map(|name| data.entities.get(name).map(|d| (name, d)))
        .filter(|(_, d)| 
            d.min_level <= level
            && (d.max_level == 0 || d.max_level >= level)
        )
        .map(|(name, d)| (d.spawn_chance.unwrap_or(1.), name.to_string()))
        .collect()
}

fn get_board_pieces(level: u32, data: &GameData) -> Vec<String> {
    let target_score = get_target_score(level);
    let mut rng = thread_rng();
    // TODO generete smarter?
    let item_count: usize = rng.gen_range(1..=3);

    let (mut items, mut npcs, mut fixtures) = match data.levels.get(&level) {
        Some(l) => (l.required_items.clone(), l.required_npcs.clone(), l.required_fixtures.clone()),
        None => (Vec::new(), Vec::new(), Vec::new())
    };
    let item_pool = get_entity_pool(&data, &data.items, level);
    for _ in 0..item_count.saturating_sub(items.len()) {
        items.push(item_pool.choose_weighted(&mut rng, |a| a.0).unwrap().1.clone());
    }

    let mut npc_score: i32 = npcs.iter()
        .map(|n| data.entities[n].score)
        .sum();
    let npc_pool = get_entity_pool(&data, &data.npcs, level);
    while npc_score < target_score {
        let npc = npc_pool.choose_weighted(&mut rng, |a| a.0).unwrap().1.clone();
        npc_score += data.entities[&npc].score;
        npcs.push(npc);
    };

    // unneccessary clone / alloc
    let mut output = items.clone();
    output.extend(npcs);

    // TODO redo
    if fixtures.len() == 0 && level % 2 != 1 {
        let pool = get_entity_pool(&data, &data.fixtures, level);
        fixtures.push(pool.choose_weighted(&mut rng, |a| a.0).unwrap().1.clone())
    }

    // traps
    // let trap_pool = get_entity_pool(&data, &data.traps, level);
    // let trap_count = rng.gen_range(0..6);
    // for _ in 0..trap_count {
    //     output.push(trap_pool.choose_weighted(&mut rng, |a| a.0).unwrap().1.clone());
    // }

    output.extend(fixtures);
    output
}
