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
use crate::globals::{TILE_SIZE, PERSP_RATIO};
use crate::world_to_tile;
use crate::graphics::renderers::get_entity_sprite;

const FONT_SIZE: f32 = 16.;

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
        let size = context.graphics.text_dimensions("default", &text, FONT_SIZE);

        let Some(base) = get_entity_sprite(entity, state) else { continue };
        let tile = world_to_tile(base.v);
        if !board.visible.contains(&tile) { continue; }

        context.graphics.draw_text(
            "default",
            &text,
            base.v + Vector2f::new(TILE_SIZE, TILE_SIZE * PERSP_RATIO) - Vector2f::new(size.x, 8.),
            FONT_SIZE,
            Params2d { color: Color(195, 234, 254, 255), ..Default::default() }
        );
    }
}