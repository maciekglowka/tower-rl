use std::collections::{HashMap, HashSet};

use rogalik::math::vectors::Vector2I;
use::rogalik::storage::{Entity, World};

use crate::components::Position;
use crate::globals::BOARD_SIZE;
use crate::utils::{get_entities_at_position, spawn_with_position};

#[derive(Default)]
pub struct Board {
    pub tiles: HashMap<Vector2I, Entity>,
    pub origin: Vector2I
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
        let _ = spawn_with_position(world, "Jellyfish", Vector2I::new(4, 4));
        let _ = spawn_with_position(world, "Jellyfish", Vector2I::new(4, 5));
    }
}

fn spawn_line(world: &mut World, vs: &HashSet<Vector2I>) {
    for v in vs {
        let entity = spawn_with_position(world, "Tile", *v).unwrap();
        world.get_resource_mut::<Board>().unwrap().tiles.insert(*v, entity);

    }
}

fn remove_line(world: &mut World, vs: &HashSet<Vector2I>) {
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
            get_col(origin.x, origin.y),
            get_col(origin.x + BOARD_SIZE as i32, origin.y),
        ),
        a if a == Vector2I::LEFT => (
            get_col(origin.x + BOARD_SIZE as i32 - 1, origin.y),
            get_col(origin.x - 1, origin.y),
        ),
        a if a == Vector2I::DOWN => (
            get_row(origin.x, origin.y),
            get_row(origin.x, origin.y + BOARD_SIZE as i32),
        ),
        a if a == Vector2I::UP => (
            get_row(origin.x, origin.y + BOARD_SIZE as i32 - 1),
            get_row(origin.x, origin.y - 1),
        ),
        _ => panic!("Wrong shift dir!")
    };

    remove_line(world, &removed_vs);
    world.get_resource_mut::<Board>().unwrap().origin += dir;
    spawn_line(world, &new_vs);
}

fn get_row(origin_x: i32, y: i32) -> HashSet<Vector2I> {
    HashSet::from_iter(
        (0..BOARD_SIZE as i32).map(|i| Vector2I::new(origin_x + i, y))
    )
}

fn get_col(x: i32, origin_y: i32) -> HashSet<Vector2I> {
    HashSet::from_iter(
        (0..BOARD_SIZE as i32).map(|i| Vector2I::new(x, origin_y + i))
    )
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
