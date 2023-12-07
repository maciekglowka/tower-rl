use core::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use rogalik::{
    engine::{Color, GraphicsContext, Params2d},
    math::vectors::Vector2f,
};

use hike_data::Settings;

use super::super::globals::{
    UI_BUTTON_HEIGHT, UI_GAP, UI_BUTTON_TEXT_SIZE, UI_BOTTOM_PANEL_HEIGHT,
    UI_BG_Z, UI_STATUS_TEXT_SIZE
};
use super::{UiState, UiMode, InputState, get_viewport_bounds};
use super::buttons::Button;
use super::context_menu::CONTEXT_VISIBLE;
use super::span::Span;
use super::text_box::TextBox;
use super::utils;

static TAB_IDX: AtomicUsize = AtomicUsize::new(0);

pub fn handle_help_menu(
    context: &mut crate::Context_,
    input_state: &InputState,
    ui_state: &mut UiState,
    settings: &mut Settings
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

    draw_build_version(
        Vector2f::new(
            bounds.1.x - 2.* UI_GAP,
            origin.y + 4. * UI_GAP
        ),
        context,
        &ui_state.build_version
    );


    draw_help_tab(
        Vector2f::new(bounds.0.x + 2. * UI_GAP, bounds.1.y - 3. * UI_GAP),
        width - 2. * UI_GAP,
        height - 2. * UI_GAP,
        context,
        settings,
        input_state
    );

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
    let sprite_idx = if TAB_IDX.load(Relaxed) == idx { 1 } else { 0 };
    let button = Button::new(
        v.x,
        v.y,
        width,
        UI_BUTTON_HEIGHT
    )
        .with_sprite("ui", sprite_idx)
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
        .with_span(Span::new().with_text_borrowed("\u{0098}").with_size(UI_BUTTON_TEXT_SIZE));
    button.draw(context);
    button.clicked(state)
}

fn draw_help_tab(
    origin: Vector2f,
    width: f32,
    height: f32,
    context: &mut crate::Context_,
    settings: &mut Settings,
    state: &InputState
) {
    match TAB_IDX.load(Relaxed) {
        0 => draw_symbols_tab(origin, context),
        1 => draw_text_tab(ITEM_TEXT, origin, width, context),
        2 => draw_text_tab(WEAPON_TEXT, origin, width, context),
        3 => {
            draw_text_tab(CTRL_TEXT, origin, width, context);
            draw_ctrl_settings(origin, width, height, context, settings, state)
        },
        _ => ()
    }
}

fn draw_build_version(origin: Vector2f, context: &mut crate::Context_, ver: &str) {
    let w = context.graphics.text_dimensions("default", ver, UI_STATUS_TEXT_SIZE).x;
    let v = origin - Vector2f::new(w, 0.);
    let span = Span::new()
        .with_text_borrowed(ver)
        .with_text_color(Color(126, 104, 104, 255))
        .with_size(UI_STATUS_TEXT_SIZE);
    span.draw(v, context);
}

fn draw_symbols_tab(
    origin: Vector2f,
    context: &mut crate::Context_
) {
    let data = [
        (utils::ICON_HIT, "Hit"),
        (utils::ICON_POISON, "Poison"),
        (utils::ICON_DURABILITY, "Durability"),
        (utils::ICON_STUN, "Stun"),
        (utils::ICON_SWING, "Swing (hit front and sides)"),
        (utils::ICON_LUNGE, "Lunge (hit 2 tiles in front)"),
        (utils::ICON_PUSH, "Push"),
        (utils::ICON_GOLD, "Gold"),
        (utils::ICON_HEAL, "Heal / Health"),
        (utils::ICON_IMMUNITY, "Immunity"),
        (utils::ICON_HEAL_POISON, "Heal Poison"),
        (utils::ICON_REGENERATION, "Regenerate"),
        (utils::ICON_TELEPORT, "Teleport"),
        (utils::ICON_LEVEL, "Level"),
    ];
    let mut v = origin;
    for d in data {
        let span = Span::new()
            .with_size(UI_BUTTON_TEXT_SIZE)
            .with_sprite("icons", d.0)
            .with_spacer(0.5)
            .with_text_borrowed(d.1);
        span.draw(v, context);
        v -= Vector2f::new(0., UI_BUTTON_TEXT_SIZE + UI_GAP);
    }
}

fn draw_ctrl_settings(
    origin: Vector2f,
    width: f32,
    height: f32,
    context: &mut crate::Context_,
    settings: &mut Settings,
    state: &InputState
) {
    let y = origin.y - height + 2. * UI_GAP + UI_STATUS_TEXT_SIZE;

    let span = Span::new()
        .with_text_borrowed("Swipe sensitivity")
        .with_size(UI_STATUS_TEXT_SIZE);
    span.draw(Vector2f::new(origin.x, y + 4. * UI_GAP + UI_BUTTON_HEIGHT), context);

    let single_width = width / 4.;

    let value_width = width - 2. * (UI_GAP + single_width);

    let value = Button::new(
            origin.x + 0.5 * (width - value_width),
            y,
            value_width,
            UI_BUTTON_HEIGHT
        )
        .with_sprite("ui", 1)
        .with_span(
            Span::new()
                .with_size(UI_BUTTON_TEXT_SIZE)
                .with_text_owned("|".repeat(settings.swipe_sensitivity as usize))
        );
    value.draw(context);

    let down = Button::new(
            origin.x,
            y,
            single_width,
            UI_BUTTON_HEIGHT
        )
        .with_sprite("ui", 0)
        .with_span(
            Span::new()
                .with_size(UI_BUTTON_TEXT_SIZE)
                .with_text_borrowed("-")
        );
    down.draw(context);

    let up = Button::new(
            origin.x + width - single_width,
            y,
            single_width,
            UI_BUTTON_HEIGHT
        )
        .with_sprite("ui", 0)
        .with_span(
            Span::new()
                .with_size(UI_BUTTON_TEXT_SIZE)
                .with_text_borrowed("+")
        );
    up.draw(context);

    if down.clicked(state) {
        settings.swipe_sensitivity = (settings.swipe_sensitivity - 1).max(1);
        settings.dirty = true;
    }
    if up.clicked(state) {
        settings.swipe_sensitivity = (settings.swipe_sensitivity + 1).min(10);
        settings.dirty = true;
    }
}

fn draw_text_tab(
    text: &str,
    origin: Vector2f,
    width: f32,
    context: &mut crate::Context_
) {
    let text_box = TextBox::new()
        .with_size(UI_BUTTON_TEXT_SIZE)
        .with_text_borrowed(text);
    text_box.draw(
        origin,
        width,
        context
    );
}

const CTRL_TEXT: &str = 
"MOBILE CONTROLS
---------------
  Swipe: move
  Swipe + hold: move continuously
  Board tap: wait

KEYBOARD CONTROLS
-----------------
  WASD / Arrows: move
  Space: wait
  Q: pick / interact
  E: [more] (if available)
  1234: change weapon slot
  ZXCV: use item
";

const WEAPON_TEXT: &str =
"WEAPONS
-------
The player has 4 weapon slots available. Only one can be active at a time.
Each weapon action (attack, pick, repair etc.) is always performed on the active slot.

Beware: picking new weapon permanently replaces the active one.

Weapons have a durability parameter (marked by a hammer icon) that decreases with every use by one.

Weapon switching does NOT take a turn.
";

const ITEM_TEXT: &str =
"ITEMS
-----
The player can carry up to 4 items at a time. Newly picked item is always placed on the first free slot.
When no slots are available, new items cannot be picked.

Most of the items are randomized per each gameplay and have to be discovered on the first use.

Item use takes a single turn.
";