use rogalik::engine::{Color, GraphicsContext};
use rogalik::math::vectors::{Vector2i, Vector2f};
use rogalik::storage::{ComponentSet, Entity, World, WorldEvent};

use super::GraphicsState;
use super::globals::TILE_SIZE;

mod buttons;
mod context_menu;
mod input;
mod inventory;
mod modal;
mod overlays;
mod panels;
mod span;
mod status;
mod utils;

#[derive(Default)]
pub struct InputState {
    pub mouse_world_position: Vector2f,
    pub mouse_screen_position: Vector2f,
    pub mouse_button_left: ButtonState,
    pub direction: InputDirection,
    pub shift: ButtonState,
    pub action: ButtonState,
    pub pause: ButtonState,
    pub digits: [ButtonState; 10]
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
    context: &mut crate::Context_,
    state: &GraphicsState
) {
    overlays::draw_overlays(world, context, state);
}

pub fn ui_update(
    world: &mut World,
    input_state: InputState,
    context: &mut crate::Context_,
) {
    status::draw_status(world, context);
    let mut ui_click = false;
    // if let Some(clicked) = inventory::handle_inventory_buttons(world, context, &input_state) {
    //     inventory::click_weapon(clicked, world);
    //     ui_click = true
    // }
    inventory::handle_inventory(world, context, &input_state);
    if context_menu::handle_menu(world, context, &input_state) {
        ui_click = true
    }

    if ui_click { return };
    // inventory::handle_shift_input(world, &input_state);
    input::handle_dir_input(world, &input_state);
}

fn get_viewport_bounds(context: &crate::Context_) -> (Vector2f, Vector2f) {
    let scale = context.graphics.get_current_camera().get_scale();
    let half_size = 0.5 * context.get_physical_size() / scale;
    let centre = (hike_game::globals::BOARD_SIZE as f32) * TILE_SIZE / 2.;
    (
        Vector2f::new(centre - half_size.x, centre - half_size.y),
        Vector2f::new(centre + half_size.x, centre + half_size.y)
    )
}