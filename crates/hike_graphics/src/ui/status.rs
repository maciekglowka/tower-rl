use rogalik::{
    math::vectors::Vector2F,
    storage::World
};

use hike_game::components::{Health, Player};

use super::{GraphicsBackend, SpriteColor};

pub fn draw_status(world: &World, backend: &dyn GraphicsBackend) {
    let query = world.query::<Player>().with::<Health>();
    let Some(item) = query.iter().next() else { return };

    let health = item.get::<Health>().unwrap();

    backend.draw_ui_text(
        "default",
        &format!("HP: {}", health.0),
        Vector2F::new(10., 42.),
        32,
        SpriteColor(255, 255, 255, 255)
    );
}