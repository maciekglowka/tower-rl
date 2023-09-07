use rogalik::math::vectors::{Vector2I, Vector2F};
use rogalik::storage::{ComponentSet, Entity, World, WorldEvent};

use super::{GraphicsState, GraphicsBackend, SpriteColor};

mod buttons;
mod context_menu;
mod input;
mod inventory;
mod modal;
mod overlays;
mod panels;
mod span;
mod status;

#[derive(Default)]
pub struct InputState {
    pub mouse_world_position: Vector2F,
    pub mouse_screen_position: Vector2F,
    pub mouse_button_left: ButtonState,
    pub direction: InputDirection,
    pub shift: ButtonState,
    pub action: ButtonState,
}

#[derive(Default, PartialEq)]
pub enum InputDirection {
    #[default]
    None,
    Up,
    Down,
    Left,
    Right,
    Still
}

#[derive(Default, PartialEq)]
pub enum ButtonState {
    #[default]
    Up,
    Down,
    Pressed,
    Released
}

pub fn draw_world_ui(
    world: &World,
    backend: &dyn GraphicsBackend,
    state: &GraphicsState
) {
    overlays::draw_overlays(world, backend, state);
}

pub fn ui_update(
    world: &mut World,
    input_state: InputState,
    backend: &dyn GraphicsBackend,
    scale: f32
) {
    status::draw_status(world, backend, scale);
    let mut ui_click = false;
    if let Some(clicked) = inventory::handle_inventory(world, backend, &input_state, scale) {
        inventory::click_item(clicked, world);
        ui_click = true
    }
    if context_menu::handle_menu(world, backend, &input_state, scale) {
        ui_click = true
    }

    if ui_click { return };
    inventory::handle_shift_input(world, &input_state);
    input::handle_dir_input(world, &input_state);
}
