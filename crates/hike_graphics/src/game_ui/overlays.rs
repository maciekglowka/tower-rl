use rogalik::{
    engine::{Color, GraphicsContext, Params2d},
    math::vectors::Vector2f,
    storage::World
};

use hike_game::{
    Board,
    components::{Health, Player},
};

use crate::GraphicsState;
use crate::globals::{TILE_SIZE, UI_OVERLAY_Z, UI_OVERLAY_FONT_SIZE, HEALTH_COLOR};
use crate::world_to_tile;
use crate::graphics::renderers::get_entity_sprite;

pub fn draw_overlays(
    world: &World,
    context: &mut crate::Context_,
    state: &GraphicsState
) {
    let query = world.query::<Health>().build();
    let Some(board) = world.get_resource::<Board>() else { return };        

    for (health, &entity) in query.iter::<Health>().zip(query.entities()) {
        if world.get_component::<Player>(entity).is_some() { continue };

        let text = format!("{}", health.0.current);
        let size = context.graphics.text_dimensions("default", &text, UI_OVERLAY_FONT_SIZE);

        let Some(base) = get_entity_sprite(entity, state) else { continue };
        let tile = world_to_tile(base.v);
        if !board.visible.contains(&tile) { continue; }

        let _ = context.graphics.draw_text(
            "default",
            &text,
            base.v + Vector2f::new(TILE_SIZE - size.x, 0.),
            UI_OVERLAY_Z,
            UI_OVERLAY_FONT_SIZE,
            Params2d { color: HEALTH_COLOR, ..Default::default() }
        );
    }
}