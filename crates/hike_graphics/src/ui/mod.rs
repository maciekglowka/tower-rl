use rogalik::engine::{Color, GraphicsContext};
use rogalik::math::vectors::{Vector2i, Vector2f};
use rogalik::storage::{ComponentSet, Entity, World, WorldEvent};
// use std::collections::HashMap;

use super::GraphicsState;
use super::globals::{TILE_SIZE, BOARD_V_OFFSET, UI_TOP_OFFSET};

pub mod bubbles;
mod buttons;
mod context_menu;
mod help;
mod input;
mod inventory;
mod modal;
mod overlays;
mod span;
mod status;
mod text_box;
mod utils;

#[derive(Clone, Copy, Default)]
pub struct InputState {
    pub mouse_world_position: Vector2f,
    pub mouse_screen_position: Vector2f,
    pub mouse_button_left: ButtonState,
    pub direction: InputDirection,
    pub action_left: ButtonState,
    pub action_right: ButtonState,
    pub pause: ButtonState,
    pub digits: [ButtonState; 10],
    pub item_action: [ButtonState; 4] // ZXCV
}

#[derive(Default)]
pub struct UiState {
    pub direction_buffer: Option<InputDirection>,
    mode: UiMode,
    bubbles: Vec<bubbles::Bubble>
}

#[derive(Default)]
pub enum UiMode {
    #[default]
    Game,
    HelpMenu
}

#[derive(Clone, Copy, Default, PartialEq)]
pub enum InputDirection {
    #[default]
    None,
    Up,
    Down,
    Left,
    Right,
    Still
}

#[derive(Clone, Copy, Default, PartialEq)]
pub enum ButtonState {
    #[default]
    Up,
    Down,
    Pressed,
    Released
}

pub fn draw_world_ui(
    world: &World,
    context: &mut crate::Context_,
    state: &mut GraphicsState
) {
    overlays::draw_overlays(world, context, state);
    bubbles::handle_bubbles(world, state, context);
}

pub fn ui_update(
    world: &mut World,
    input_state: &mut InputState,
    ui_state: &mut UiState,
    context: &mut crate::Context_,
) {
    match ui_state.mode {
        UiMode::Game => update_game_ui(world, input_state, ui_state, context),
        UiMode::HelpMenu => help::handle_help_menu(context, input_state, ui_state)
    }
}

fn update_game_ui(
    world: &mut World,
    input_state: &mut InputState,
    ui_state: &mut UiState,
    context: &mut crate::Context_,
) {
    status::draw_status(world, context);
    let mut ui_click = false;

    inventory::handle_inventory(world, context, input_state);

    if context_menu::handle_menu(world, context, input_state) {
        ui_click = true
    }
    if help::handle_help_button(context, input_state, ui_state) {
        ui_click = true
    }
    if ui_click { return };
    input::handle_dir_input(world, input_state, ui_state);
}

fn get_viewport_bounds(context: &crate::Context_) -> (Vector2f, Vector2f) {
    let scale = context.graphics.get_current_camera().get_scale();
    let half_size = 0.5 * context.get_physical_size() / scale;
    let centre = (hike_game::globals::BOARD_SIZE as f32) * TILE_SIZE / 2.;
    (
        Vector2f::new(centre - half_size.x, centre - half_size.y + BOARD_V_OFFSET),
        Vector2f::new(centre + half_size.x, centre + half_size.y + BOARD_V_OFFSET - UI_TOP_OFFSET)
    )
}
