use rogalik::{
    math::vectors::Vector2F,
    storage::World
};

use hike_game::{
    Board,
    components::{Health, Player},
};

use crate::GraphicsState;
use crate::globals::TILE_SIZE;
use super::{GraphicsBackend, SpriteColor};
use crate::world_to_tile;
use crate::graphics::renderers::get_entity_sprite;

const FONT_SIZE: u32 = 24;

pub fn draw_overlays(
    world: &World,
    backend: &dyn GraphicsBackend,
    state: &GraphicsState
) {
    let query = world.query::<Health>();
    let Some(board) = world.get_resource::<Board>() else { return };        

    for item in query.iter() {
        if world.get_component::<Player>(item.entity).is_some() { continue };
        let health = item.get::<Health>().unwrap();

        let text = format!("{}", health.0.current);
        let size = backend.text_size("default", &text, FONT_SIZE);

        let Some(base) = get_entity_sprite(item.entity, state) else { continue };
        let tile = world_to_tile(base.v);
        if !board.visible.contains(&tile) { continue; }

        backend.draw_world_sprite(
            "ascii",
            219,
            base.v + Vector2F::new(TILE_SIZE, TILE_SIZE) - Vector2F::new(size.x, size.y),
            Vector2F::new(size.x, size.y),
            SpriteColor(195, 234, 254, 255)
        );
        backend.draw_world_text(
            "default",
            &text,
            base.v + Vector2F::new(TILE_SIZE, TILE_SIZE) - Vector2F::new(size.x, 0.),
            FONT_SIZE,
            SpriteColor(180, 0, 50, 255)
        );
    }
}