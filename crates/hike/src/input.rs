use rogalik::{
    engine::{
        GraphicsContext, ResourceId,
        input::{MouseButton, VirtualKeyCode, TouchPhase}
    },
    math::vectors::Vector2f
};
use std::collections::HashMap;

use hike_graphics::game_ui::{ButtonState, InputState, InputDirection};

use super::Context_;

// fn get_mouse_screen_position() -> Vector2f {
//     let v = mouse_position();
//     Vector2f::new(v.0, v.1)
// }

// fn get_mouse_world_position(camera: &Camera2D) -> Vector2f {
//     let mouse = mouse_position();
//     let v = camera.screen_to_world(Vec2::new(mouse.0, mouse.1));
//     Vector2f::new(v.x, v.y)
// }

pub fn get_input_state(
    camera: ResourceId,
    touch_state: &mut HashMap<u64, Vector2f>,
    context: &Context_
) -> InputState {
    let mut left = ButtonState::Up;
    if context.input.is_mouse_button_down(MouseButton::Left) {
        left = ButtonState::Down
    }
    if context.input.is_mouse_button_released(MouseButton::Left) {
        left = ButtonState::Released
    }
    if context.input.is_mouse_button_pressed(MouseButton::Left) {
        left = ButtonState::Pressed
    }

    let action_right = key_state(context, VirtualKeyCode::E);
    let action_left = key_state(context, VirtualKeyCode::Q);
    let pause = key_state(context, VirtualKeyCode::Space);

    let mut direction = handle_touches(context, touch_state);
    if context.input.is_key_pressed(VirtualKeyCode::W) { direction = InputDirection::Up }
    if context.input.is_key_pressed(VirtualKeyCode::S) { direction = InputDirection::Down }
    if context.input.is_key_pressed(VirtualKeyCode::A) { direction = InputDirection::Left }
    if context.input.is_key_pressed(VirtualKeyCode::D) { direction = InputDirection::Right }
    if context.input.is_key_pressed(VirtualKeyCode::Space) { direction = InputDirection::Still }

    let digits = [
        key_state(context, VirtualKeyCode::Key0),
        key_state(context, VirtualKeyCode::Key1),
        key_state(context, VirtualKeyCode::Key2),
        key_state(context, VirtualKeyCode::Key3),
        key_state(context, VirtualKeyCode::Key4),
        key_state(context, VirtualKeyCode::Key5),
        key_state(context, VirtualKeyCode::Key6),
        key_state(context, VirtualKeyCode::Key7),
        key_state(context, VirtualKeyCode::Key8),
        key_state(context, VirtualKeyCode::Key9),
    ];
    let item_action = [
        key_state(context, VirtualKeyCode::Z),
        key_state(context, VirtualKeyCode::X),
        key_state(context, VirtualKeyCode::C),
        key_state(context, VirtualKeyCode::V),
    ];

    let mut m = context.input.get_mouse_physical_position();
    let mut w = Vector2f::ZERO;
    if let Some(camera) = context.graphics.get_camera(camera) {
        w = camera.camera_to_world(m);
    }

    InputState {
        mouse_screen_position: m,
        mouse_world_position: w,
        mouse_button_left: left,
        direction,
        action_left,
        action_right,
        pause,
        digits,
        item_action
    }
}

fn key_state(context: &Context_, code: VirtualKeyCode) -> ButtonState {
    if context.input.is_key_pressed(code) { ButtonState::Pressed } else { ButtonState::Up }
}

fn handle_touches(context: &Context_, touch_state: &mut HashMap<u64, Vector2f>) -> InputDirection {
    for (id, touch) in context.input.get_touches().iter() {
        match touch.phase {
            TouchPhase::Started => { touch_state.insert(*id, touch.position); },
            TouchPhase::Moved => {
                if let Some(start) = touch_state.get(&id) {
                    let dx = touch.position.x - start.x;
                    let dy = touch.position.y - start.y;
                    let thresh = 0.05 * context.get_physical_size().x;
                    let mut dir = InputDirection::None;
                    if dx > thresh { dir = InputDirection::Right }
                    if dx < -thresh { dir = InputDirection::Left }
                    if dy > thresh { dir = InputDirection::Up }
                    if dy < -thresh { dir = InputDirection::Down }
                    if dir != InputDirection::None {
                        // touch_state.insert(*id, touch.position);
                        touch_state.remove(id);
                        return dir
                    }
                }
            },
            TouchPhase::Ended => {
                touch_state.remove(&id);
            }
            _ => ()
        }
    }
    InputDirection::None
}