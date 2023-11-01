use rogalik::{
    engine::{Color, GraphicsContext, Params2d},
    events::EventBus,
    math::vectors::Vector2f,
    storage::World
};

use hike_data::GameData;
use hike_game::{Board, GameStats};

use crate::UiEvent;
use super::super::globals::{
    UI_BUTTON_HEIGHT, UI_GAP, UI_BUTTON_TEXT_SIZE,
    UI_BG_Z
};
use super::{UiState, UiMode, InputState, get_viewport_bounds};
use super::buttons::Button;
use super::span::Span;

pub fn handle_menu(
    context: &mut crate::Context_,
    input_state: &InputState,
    ui_state: &mut UiState,
    events: &mut EventBus<UiEvent>,
    world: &World
) {
    let bounds = get_viewport_bounds(context);
    let gap = 2. * UI_GAP;
    let width = bounds.1.x - bounds.0.x - 2. * gap;
    let height = bounds.1.y - bounds.0.y - 2. * gap - UI_GAP - UI_BUTTON_HEIGHT;

    let origin = bounds.0 + Vector2f::new(gap, gap + UI_GAP + UI_BUTTON_HEIGHT);

    let _ = context.graphics.draw_atlas_sprite(
        "ui",
        0,
        origin,
        UI_BG_Z,
        Vector2f::new(width, height),
        Params2d { slice: Some((4, Vector2f::new(1., 1.))), ..Default::default() }
    );

    draw_centered_span(
        context,
        bounds,
        bounds.1.y - gap * 2.,
        Span::new()
            .with_text_borrowed("GAME OVER")
            .with_size(1.5 * UI_BUTTON_TEXT_SIZE)
    );

    if let Some(board) = world.get_resource::<Board>() {
        draw_centered_span(
            context,
            bounds,
            bounds.1.y - gap * 2. - 2.* UI_BUTTON_TEXT_SIZE,
            Span::new()
                .with_text_owned(format!("You've reached level {} of the tower", board.level))
                .with_size(UI_BUTTON_TEXT_SIZE)
        );
    }

    draw_centered_span(
        context,
        bounds,
        bounds.1.y - gap * 2. - 4. * UI_BUTTON_TEXT_SIZE,
        Span::new()
            .with_text_borrowed("Kills:")
            .with_size(UI_BUTTON_TEXT_SIZE)
    );

    draw_kill_spans(
        context,
        bounds,
        world,
        bounds.1.y - gap * 2. - 6. * UI_BUTTON_TEXT_SIZE
    );

    let button = Button::new(
            origin.x,
            origin.y - UI_GAP - UI_BUTTON_HEIGHT,
            width,
            UI_BUTTON_HEIGHT
        )
        .with_sprite("ui", 0)
        .with_span(
            Span::new().with_text_borrowed("Restart")
                .with_size(UI_BUTTON_TEXT_SIZE)
        );
    button.draw(context);
    if button.clicked(input_state) {
        events.publish(UiEvent::Restart);
    }
}

fn draw_centered_span(
    context: &mut crate::Context_,
    bounds: (Vector2f, Vector2f),
    y: f32,
    span: Span
) {
    let x = bounds.0.x + 0.5 * (bounds.1.x - bounds.0.x) - 0.5 * span.width(context);
    span.draw(Vector2f::new(x, y), context);
}

fn draw_kill_spans(
    context: &mut crate::Context_,
    bounds: (Vector2f, Vector2f),
    world: &World,
    mut y: f32
) {
    if let Some(stats) = world.get_resource::<GameStats>() {
        if let Some(data) = world.get_resource::<GameData>() {
            for (k, v) in stats.kills.iter() {
                if let Some(entry) = data.entities.get(k) {
                    let span = Span::new()
                        .with_sprite(&entry.sprite.atlas_name, entry.sprite.index)
                        .with_sprite_color(entry.sprite.color)
                        .with_spacer(0.25 * UI_BUTTON_TEXT_SIZE)
                        .with_text_owned(format!("{} x{}", k.replace("_", " "), v))
                        .with_size(UI_BUTTON_TEXT_SIZE);
                    draw_centered_span(context, bounds, y, span);
                    y -= 1.25 * UI_BUTTON_TEXT_SIZE;
                }
            }
        }
    }
}