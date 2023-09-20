use rogalik::{
    math::vectors::Vector2F,
    storage::World
};

use hike_game::{
    Board,
    components::{Health, Player, Poisoned, Immune, Dexterity},
    get_entities_at_position, get_player_position
};

use super::{GraphicsBackend, SpriteColor};
use super::super::globals::{UI_GAP, UI_TEXT_GAP, UI_STATUS_TEXT_SIZE};

pub fn draw_status(world: &World, backend: &dyn GraphicsBackend, scale: f32) {
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
    if let Some(dexterity) = world.get_component::<Dexterity>(query.single_entity().unwrap()) {
        text += &format!(" Dexterity({})", dexterity.0);
    }

    backend.draw_ui_text(
        "default",
        &text,
        Vector2F::new(scale * UI_GAP, scale * (UI_TEXT_GAP + UI_STATUS_TEXT_SIZE as f32)),
        (UI_STATUS_TEXT_SIZE as f32 * scale) as u32,
        SpriteColor(150, 128, 128, 255)
    );
}
