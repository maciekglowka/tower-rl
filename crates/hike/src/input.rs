use macroquad::prelude::*;
use rogalik::math::vectors::Vector2F;
use std::collections::HashMap;

use hike_graphics::ui::{ButtonState, InputState, InputDirection};

fn get_mouse_screen_position() -> Vector2F {
    let v = mouse_position();
    Vector2F::new(v.0, v.1)
}

fn get_mouse_world_position(camera: &Camera2D) -> Vector2F {
    let mouse = mouse_position();
    let v = camera.screen_to_world(Vec2::new(mouse.0, mouse.1));
    Vector2F::new(v.x, v.y)
}

pub fn get_input_state(camera: &Camera2D, touch_state: &mut HashMap<u64, Vec2>) -> InputState {
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

    let shift = if is_key_pressed(KeyCode::O) { ButtonState::Pressed } else { ButtonState::Up };
    let action = if is_key_pressed(KeyCode::Space) { ButtonState::Pressed } else { ButtonState::Up };
    let pause = if is_key_pressed(KeyCode::P) { ButtonState::Pressed } else { ButtonState::Up };

    let mut direction = handle_touches(touch_state);
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
        shift,
        action,
        pause
    }
}

fn handle_touches(touch_state: &mut HashMap<u64, Vec2>) -> InputDirection {
    let touches = touches();
    if let Some(touch) = touches.iter().next() {
        match touch.phase {
            TouchPhase::Started => { touch_state.insert(touch.id, touch.position); },
            TouchPhase::Moved => {
                if let Some(start) = touch_state.get(&touch.id) {
                    let dx = touch.position.x - start.x;
                    let dy = touch.position.y - start.y;
                    let thresh = 0.05 * screen_width();
                    let mut dir = InputDirection::None;
                    if dx > thresh { dir = InputDirection::Right }
                    if dx < -thresh { dir = InputDirection::Left }
                    if dy > thresh { dir = InputDirection::Down }
                    if dy < -thresh { dir = InputDirection::Up }
                    if dir != InputDirection::None {
                        touch_state.insert(touch.id, touch.position);
                        return dir
                    }
                }
            },
            TouchPhase::Ended => {
                touch_state.remove(&touch.id);
            }
            _ => ()
        }
    }
    InputDirection::None
}