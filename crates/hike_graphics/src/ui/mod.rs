use rogalik::math::vectors::{Vector2I, Vector2F};
use rogalik::storage::{ComponentSet, Entity, World, WorldEvent};

use super::{GraphicsState, GraphicsBackend, SpriteColor};

mod buttons;
mod context_menu;
mod input;
mod inventory;
mod modal;
mod span;
mod status;

#[derive(Default)]
pub struct InputState {
    pub mouse_world_position: Vector2F,
    pub mouse_screen_position: Vector2F,
    pub mouse_button_left: ButtonState,
    pub direction: InputDirection,
    pub shift: ButtonState,
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

pub fn ui_update(
    world: &mut World,
    input_state: InputState,
    backend: &dyn GraphicsBackend
) {
    status::draw_status(world, backend);
    let mut ui_click = false;
    if let Some(clicked) = inventory::handle_inventory(world, backend, &input_state) {
        inventory::click_item(clicked, world);
        ui_click = true
    }
    if context_menu::handle_menu(world, backend, &input_state) {
        ui_click = true
    }

    if ui_click { return };
    input::handle_dir_input(world, &input_state);
    // cards::handle_shift_input(world, &input_state);
}