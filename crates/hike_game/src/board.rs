use std::collections::HashMap;

use rogalik::math::vectors::Vector2I;
use::rogalik::storage::{Entity, World};

use crate::globals::BOARD_SIZE;
use crate::utils::{get_entities_at_position, spawn_with_position};

#[derive(Default)]
pub struct Board {
    pub tiles: HashMap<Vector2I, Entity>
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

        let _ = spawn_with_position(world, "Sword", Vector2I::new(4, 0));
        let _ = spawn_with_position(world, "Jellyfish", Vector2I::new(4, 4));
    }
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
