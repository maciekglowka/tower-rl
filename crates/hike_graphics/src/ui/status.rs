use rogalik::{
    engine::{Color, GraphicsContext, Params2d},
    math::vectors::Vector2f,
    storage::World
};

use hike_game::{
    Board,
    components::{Health, Player, Poisoned, Immune},
    get_entities_at_position, get_player_position
};

use super::super::globals::{UI_GAP, UI_TEXT_GAP, UI_STATUS_TEXT_SIZE};
use super::get_viewport_bounds;

pub fn draw_status(world: &World, context: &mut crate::Context_) {
    let query = world.query::<Player>().with::<Health>().build();
    let Some(board) = world.get_resource::<Board>() else { return };

    let Some(health) = query.single::<Health>() else { return };
    let player = query.single::<Player>().unwrap();

    let mut text = format!("HP: {}/{} Gold: {}, Level: {}", health.0.current, health.0.max, player.gold, board.level);
    if let Some(poisoned) = world.get_component::<Poisoned>(query.single_entity().unwrap()) {
        text += &format!(" Poisoned({})", poisoned.0);
    }
    if let Some(immune) = world.get_component::<Immune>(query.single_entity().unwrap()) {
        text += &format!(" Immune({})", immune.0);
    }
    let bounds = get_viewport_bounds(context);

    context.graphics.draw_text(
        "default",
        &text,
        Vector2f::new(
            bounds.0.x + UI_GAP,
            bounds.1.y - UI_GAP - UI_STATUS_TEXT_SIZE,
        ),
        UI_STATUS_TEXT_SIZE,
        Params2d { color: Color(150, 128, 128, 255), ..Default::default() }
    );
}
