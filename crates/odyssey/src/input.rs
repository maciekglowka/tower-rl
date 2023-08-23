use macroquad::prelude::*;
use rogalik::math::vectors::Vector2F;

use odyssey_graphics::ui::{ButtonState, InputState, InputDirection};

fn get_mouse_screen_position() -> Vector2F {
    let v = mouse_position();
    Vector2F::new(v.0, v.1)
}

fn get_mouse_world_position(camera: &Camera2D) -> Vector2F {
    let mouse = mouse_position();
    let v = camera.screen_to_world(Vec2::new(mouse.0, mouse.1));
    Vector2F::new(v.x, v.y)
}

pub fn get_input_state(camera: &Camera2D) -> InputState {
    // use event streams ?
    let mut left = ButtonState::Up;
    if is_mouse_button_down(MouseButton::Left) {
        left = ButtonState::Down
    }
    if is_mouse_button_released(MouseButton::Left) {
        left = ButtonState::Released
    }
    if is_mouse_button_pressed(MouseButton::Left) {
        left = ButtonState::Pressed
    }

    let mut shift = ButtonState::Up;
    if is_key_pressed(KeyCode::Enter) {
        shift = ButtonState::Pressed
    }

    let mut direction = InputDirection::None;
    if is_key_pressed(KeyCode::W) { direction = InputDirection::Up }
    if is_key_pressed(KeyCode::S) { direction = InputDirection::Down }
    if is_key_pressed(KeyCode::A) { direction = InputDirection::Left }
    if is_key_pressed(KeyCode::D) { direction = InputDirection::Right }
    if is_key_pressed(KeyCode::Space) { direction = InputDirection::Still }

    InputState {
        mouse_screen_position: get_mouse_screen_position(),
        mouse_world_position: get_mouse_world_position(camera),
        mouse_button_left: left,
        direction,
        shift
    }
}