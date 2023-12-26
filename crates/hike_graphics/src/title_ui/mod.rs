use rogalik::engine::{GraphicsContext, Params2d};
use rogalik::math::vectors::{Vector2i, Vector2f};

use crate::game_ui::{InputState, get_viewport_bounds};

pub fn update_title_ui(
    context: &mut crate::Context_
) {
    let bounds = get_viewport_bounds(context);

    let w = 0.75 * (bounds.1.x - bounds.0.x);
    let h = 1.5 * w;
    let c = bounds.0 + 0.5 * (bounds.1 - bounds.0);

    let _ = context.graphics.draw_atlas_sprite(
        "title",
        0,
        c - Vector2f::new(0.5 * w, 0.5 * h),
        0,
        Vector2f::new(w, h),
        Params2d::default()
    );
}