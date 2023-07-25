use rand::prelude::*;
use std::collections::VecDeque;

use rogalik::math::vectors::{Vector2I, ORTHO_DIRECTIONS};

use crate::globals::WIND_QUEUE;

pub struct Wind {
    pub queue: VecDeque<Vector2I>
}
impl Wind {
    pub fn new() -> Wind {
        let mut wind = Wind { queue: VecDeque::new() };
        wind.init_queue();
        wind
    }
    fn init_queue(&mut self) {
        for _ in 0..WIND_QUEUE {
            self.queue.push_back(self.get_next_wind());
        }
    }
    fn get_next_wind(&self) -> Vector2I {
        let mut rng = thread_rng();
        *ORTHO_DIRECTIONS.choose(&mut rng).unwrap()
    }
    pub fn pop_wind(&mut self) -> Vector2I {
        // we assume that the queue never get's empty
        // so it is safe to unwrap
        self.queue.push_back(self.get_next_wind());
        self.queue.pop_front().unwrap()
    }
    pub fn current(&self) -> Vector2I {
        self.queue[0]
    }
}