use core::sync::atomic::Ordering::Relaxed;
use rogalik::{
    engine::{Color, GraphicsContext, Params2d},
    math::vectors::Vector2f,
};

use super::super::globals::{
    UI_BUTTON_HEIGHT, UI_GAP, UI_BUTTON_TEXT_SIZE, UI_BOTTOM_PANEL_HEIGHT,
};
use super::{UiState, UiMode, InputState, get_viewport_bounds};
use super::buttons::Button;
use super::context_menu::CONTEXT_VISIBLE;
use super::span::Span;

pub fn handle_help_menu(
    context: &mut crate::Context_,
    input_state: &InputState,
    ui_state: &mut UiState
) {
    let bounds = get_viewport_bounds(context);
    let width = bounds.1.x - bounds.0.x - 2. * UI_GAP;
    let height = bounds.1.y - bounds.0.y - 3. * UI_GAP - UI_BUTTON_HEIGHT;
    let origin = bounds.0 + Vector2f::new(UI_GAP, 2. * UI_GAP + UI_BUTTON_HEIGHT);

    let button_width = (width - 3. * UI_GAP) / 4.;

    let _ = context.graphics.draw_atlas_sprite(
        "ui",
        0,
        origin,
        Vector2f::new(width, height),
        Params2d { slice: Some((4, Vector2f::new(1., 1.))), ..Default::default() }
    );

    if draw_menu_button(origin, button_width, 3, context, input_state) {
        ui_state.mode = UiMode::Game;
    }
}

fn draw_menu_button(
    origin: Vector2f,
    width: f32,
    idx: usize,
    context: &mut crate::Context_,
    input_state: &InputState
) -> bool {
    let v = origin + Vector2f::new(
        idx as f32 * (UI_GAP + width),
        - UI_GAP - UI_BUTTON_HEIGHT
    );
    let button = Button::new(
        v.x,
        v.y,
        width,
        UI_BUTTON_HEIGHT
    )
        .with_sprite("ui", 0)
        .with_span(Span::new().with_text_borrowed("?").with_size(UI_BUTTON_TEXT_SIZE));
    button.draw(context);
    button.clicked(input_state)
}

pub fn handle_help_button(
    context: &mut crate::Context_,
    input_state: &InputState,
    ui_state: &mut UiState
) -> bool {
    if draw_help_button(context, input_state) {
        ui_state.mode = UiMode::HelpMenu;
        return true
    }
    false
}

fn draw_help_button(context: &mut crate::Context_, state: &InputState) -> bool {
    let bounds = get_viewport_bounds(context);
    let y = if CONTEXT_VISIBLE.load(Relaxed) {
        bounds.0.y + UI_BOTTOM_PANEL_HEIGHT + 2. * UI_GAP + UI_BUTTON_HEIGHT
    } else {
        bounds.0.y + UI_BOTTOM_PANEL_HEIGHT + UI_GAP
    };

    let button = Button::new(
        bounds.1.x - UI_GAP - UI_BUTTON_HEIGHT,
        y,
        UI_BUTTON_HEIGHT,
        UI_BUTTON_HEIGHT
    )
        .with_sprite("ui", 0)
        .with_span(Span::new().with_text_borrowed("?").with_size(UI_BUTTON_TEXT_SIZE));
    button.draw(context);
    button.clicked(state)
}