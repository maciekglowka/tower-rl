use core::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use rogalik::{
    engine::{Color, GraphicsContext, Params2d},
    math::vectors::Vector2f,
};

use super::super::globals::{
    UI_BUTTON_HEIGHT, UI_GAP, UI_BUTTON_TEXT_SIZE, UI_BOTTOM_PANEL_HEIGHT,
    UI_BG_Z
};
use super::{UiState, UiMode, InputState, get_viewport_bounds};
use super::buttons::Button;
use super::context_menu::CONTEXT_VISIBLE;
use super::span::Span;
use super::utils;

static TAB_IDX: AtomicUsize = AtomicUsize::new(0);

pub fn handle_help_menu(
    context: &mut crate::Context_,
    input_state: &InputState,
    ui_state: &mut UiState
) {
    let bounds = get_viewport_bounds(context);
    let width = bounds.1.x - bounds.0.x - 2. * UI_GAP;
    let height = bounds.1.y - bounds.0.y - 3. * UI_GAP - UI_BUTTON_HEIGHT;
    let origin = bounds.0 + Vector2f::new(UI_GAP, 2. * UI_GAP + UI_BUTTON_HEIGHT);

    let button_count = 5;
    let button_width = (width - (button_count - 1) as f32 * UI_GAP) / button_count as f32;

    let _ = context.graphics.draw_atlas_sprite(
        "ui",
        0,
        origin,
        UI_BG_Z,
        Vector2f::new(width, height),
        Params2d { slice: Some((4, Vector2f::new(1., 1.))), ..Default::default() }
    );

    draw_help_tab(origin, width, context);

    if draw_menu_button(origin, button_width, 0, "Symbol", context, input_state) {
        TAB_IDX.store(0, Relaxed);
    }
    if draw_menu_button(origin, button_width, 1, "Item", context, input_state) {
        TAB_IDX.store(1, Relaxed);
    }
    if draw_menu_button(origin, button_width, 2, "Weapon", context, input_state) {
        TAB_IDX.store(2, Relaxed);
    }
    if draw_menu_button(origin, button_width, 3, "Ctrl", context, input_state) {
        TAB_IDX.store(3, Relaxed);
    }
    if draw_menu_button(origin, button_width, 4, "Close", context, input_state) {
        ui_state.mode = UiMode::Game;
    }
}

fn draw_menu_button(
    origin: Vector2f,
    width: f32,
    idx: usize,
    label: &str,
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
        .with_span(Span::new().with_text_borrowed(label).with_size(UI_BUTTON_TEXT_SIZE));
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

fn draw_help_tab(
    origin: Vector2f,
    width: f32,
    context: &mut crate::Context_
) {
    match TAB_IDX.load(Relaxed) {
        0 => draw_symbols_tab(origin, width, context),
        _ => ()
    }
}

fn draw_symbols_tab(
    origin: Vector2f,
    width: f32,
    context: &mut crate::Context_
) {
    let data = [
        (utils::ICON_HIT, "Hit"),
        (utils::ICON_POISON, "Poison"),
        (utils::ICON_DURABILITY, "Durability"),
        (utils::ICON_STUN, "Stun"),
        (utils::ICON_SWING, "Swing (hit frond and sides"),
        (utils::ICON_LUNGE, "Lunge (hit 2 tiles in front"),
        (utils::ICON_PUSH, "Push"),
        (utils::ICON_GOLD, "Gold"),
        (utils::ICON_HEAL, "Heal / Health"),
        (utils::ICON_IMMUNITY, "Immunity"),
        (utils::ICON_HEAL_POISON, "Heal Poison"),
        (utils::ICON_REGENERATION, "Regenerate"),
        (utils::ICON_TELEPORT, "Teleport"),
        (utils::ICON_LEVEL, "Level"),
    ];
    let mut v = origin + Vector2f::new(UI_GAP, 4. * UI_GAP);
    for d in data {
        let span = Span::new()
            .with_size(UI_BUTTON_TEXT_SIZE)
            .with_sprite("icons", d.0)
            .with_spacer(0.5)
            .with_text_borrowed(d.1);
        span.draw(v, context);
        v += Vector2f::new(0., UI_BUTTON_TEXT_SIZE + UI_GAP);
    }
}