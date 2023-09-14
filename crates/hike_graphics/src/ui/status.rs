use rogalik::{
    math::vectors::Vector2F,
    storage::World
};

use hike_game::{
    Board,
    components::{Health, Player, Poisoned},
    get_entities_at_position, get_player_position
};

use super::{GraphicsBackend, SpriteColor};
use super::super::globals::{UI_GAP, UI_TEXT_GAP, UI_STATUS_TEXT_SIZE};

pub fn draw_status(world: &World, backend: &dyn GraphicsBackend, scale: f32) {
    let query = world.query::<Player>().with::<Health>();
    let Some(item) = query.iter().next() else { return };
    let Some(board) = world.get_resource::<Board>() else { return };

    let health = item.get::<Health>().unwrap();
    let player = item.get::<Player>().unwrap();

    let mut text = format!("HP: {}/{} Gold: {}, L: {}", health.0.current, health.0.max, player.gold, board.level);
    if let Some(poisoned) = world.get_component::<Poisoned>(item.entity) {
        text += &format!(" Poisoned({})", poisoned.0);
    }

    backend.draw_ui_text(
        "default",
        &text,
        Vector2F::new(scale * UI_GAP, scale * (UI_TEXT_GAP + UI_STATUS_TEXT_SIZE as f32)),
        (UI_STATUS_TEXT_SIZE as f32 * scale) as u32,
        SpriteColor(150, 128, 128, 255)
    );
}
