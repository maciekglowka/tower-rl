use rogalik::{
    engine::{Color, GraphicsContext, Params2d},
    math::vectors::Vector2f,
    storage::World
};

use hike_game::{
    Board,
    components::{Health, Player, Poisoned, Immune, Regeneration, Stunned},
    get_entities_at_position, get_player_position
};

use super::super::globals::{UI_GAP, UI_TEXT_GAP, UI_STATUS_TEXT_SIZE};
use super::get_viewport_bounds;
use super::span::Span;
use crate::game_ui::utils;

pub fn draw_status(world: &World, context: &mut crate::Context_) {
    let query = world.query::<Player>().with::<Health>().build();
    let Some(board) = world.get_resource::<Board>() else { return };

    let Some(health) = query.single::<Health>() else { return };
    let player = query.single::<Player>().unwrap();

    let spacer = 0.5 * UI_GAP;

    let mut span = Span::new()
        .with_size(UI_STATUS_TEXT_SIZE)
        .with_text_color(Color(150, 128, 128, 255))
        .with_sprite("icons", utils::ICON_LEVEL)
        .with_text_owned(format!("{}", board.level))
        .with_spacer(spacer)
        .with_sprite("icons", utils::ICON_HEAL)
        .with_text_owned(format!("{}/{}", health.0.current, health.0.max))
        .with_spacer(spacer)
        .with_sprite("icons", utils::ICON_GOLD)
        .with_text_owned(format!("{}", player.gold));

    if let Some(poisoned) = world.get_component::<Poisoned>(query.single_entity().unwrap()) {
        span = span.with_spacer(spacer)
            .with_sprite("icons", utils::ICON_POISON)
            .with_text_owned(format!("{}", poisoned.0));
        // text += &format!(" Poisoned({})", poisoned.0);
    }
    if let Some(immune) = world.get_component::<Immune>(query.single_entity().unwrap()) {
        span = span.with_spacer(spacer)
            .with_sprite("icons", utils::ICON_IMMUNITY)
            .with_text_owned(format!("{}", immune.0));
    }
    if let Some(regeneration) = world.get_component::<Regeneration>(query.single_entity().unwrap()) {
        span = span.with_spacer(spacer)
            .with_sprite("icons", utils::ICON_REGENERATION)
            .with_text_owned(format!("{}", regeneration.0));
    }
    if let Some(stunned) = world.get_component::<Stunned>(query.single_entity().unwrap()) {
        span = span.with_spacer(spacer)
            .with_sprite("icons", utils::ICON_STUN)
            .with_text_owned(format!("{}", stunned.0));
    }
    let bounds = get_viewport_bounds(context);

    let v = Vector2f::new(
        bounds.0.x + UI_GAP,
        bounds.1.y - UI_GAP,
    );

    span.draw(v, context);
}
