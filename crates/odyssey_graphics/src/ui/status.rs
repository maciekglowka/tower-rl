use rogalik::{
    math::vectors::Vector2F,
    storage::World
};

use odyssey_game::components::{Actor, Health, PlayerCharacter};

use super::{GraphicsBackend, SpriteColor};

pub fn draw_status(world: &World, backend: &dyn GraphicsBackend) {
    let query = world.query::<PlayerCharacter>().with::<Health>().with::<Actor>();
    let Some(item) = query.iter().next() else { return };

    let health = item.get::<Health>().unwrap();
    let actor = item.get::<Actor>().unwrap();

    backend.draw_ui_text(
        "default",
        &format!("HP: {} AP: {}/{}", health.0, actor.ap.current, actor.ap.max),
        Vector2F::new(10., 42.),
        32,
        SpriteColor(255, 255, 255, 255)
    );
}