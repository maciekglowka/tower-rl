use rogalik::math::vectors::Vector2F;
use rogalik::storage::World;

use hike_game::{Board, ContentKind, globals::BOARD_SIZE};

use super::{GraphicsBackend, SpriteColor};
use super::super::TILE_SIZE;

pub fn draw_next_content(
    world: &World,
    backend: &dyn GraphicsBackend
) {
    let Some(board) = world.get_resource::<Board>() else { return };

    let centre = backend.viewport_size() * 0.5;

    for (k, v) in board.next.iter() {
        let dir = Vector2F::new(k.y as f32, k.x as f32);
        draw_next_content_line(
            v,
            centre + k.as_f32() * 0.5 * (BOARD_SIZE + 1) as f32 * TILE_SIZE,
            dir,
            backend
        );
    }
}

fn draw_next_content_line(
    content: &Vec<ContentKind>,
    origin: Vector2F,
    dir: Vector2F,
    backend: &dyn GraphicsBackend
) {
    for (i, kind) in content.iter().enumerate() {
        let index = match kind {
            ContentKind::Item => 4,
            ContentKind::Unit => 1
        };

        backend.draw_ui_sprite(
            "ascii",
            index,
            origin + dir * i as f32 * 40. - Vector2F::new(16., 16.),
            Vector2F::new(32., 32.),
            SpriteColor(0, 0, 0, 255)
        );
    }
}