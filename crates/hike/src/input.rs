use rogalik::{
    engine::{
        GraphicsContext, ResourceId,
        input::{MouseButton, VirtualKeyCode, TouchPhase},
        Instant
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

pub struct Touch {
    pub start: Vector2f,
    pub time: Instant,
    pub dir: Option<InputDirection>
}

pub fn get_input_state(
    camera: ResourceId,
    touch_state: &mut HashMap<u64, Touch>,
    settings: &hike_data::Settings,
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

    let mut direction = handle_touches(context, touch_state, settings);
    if context.input.is_key_pressed(VirtualKeyCode::W) 
        || context.input.is_key_pressed(VirtualKeyCode::Up) { direction = InputDirection::Up }
    if context.input.is_key_pressed(VirtualKeyCode::S)
        || context.input.is_key_pressed(VirtualKeyCode::Down){ direction = InputDirection::Down }
    if context.input.is_key_pressed(VirtualKeyCode::A)
        || context.input.is_key_pressed(VirtualKeyCode::Left){ direction = InputDirection::Left }
    if context.input.is_key_pressed(VirtualKeyCode::D)
        || context.input.is_key_pressed(VirtualKeyCode::Right){ direction = InputDirection::Right }

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

fn handle_touches(
    context: &Context_,
    touch_state: &mut HashMap<u64, Touch>,
    settings: &hike_data::Settings
) -> InputDirection {
    for (id, touch) in context.input.get_touches().iter() {
        match touch.phase {
            TouchPhase::Started => { touch_state.insert(
                *id, Touch { start: touch.position, time: Instant::init(), dir: None }
            ); },
            TouchPhase::Moved => {
                if let Some(existing) = touch_state.get(&id) {
                    if existing.dir.is_none() {
                        let dx = touch.position.x - existing.start.x;
                        let dy = touch.position.y - existing.start.y;
                        let d = Vector2f::new(dx, dy);
                        let speed = (d.len() / context.get_physical_size().x) / existing.time.elapsed();
                        if speed < 2. / settings.swipe_sensitivity.pow(3) as f32 {
                            return InputDirection::None
                        }
                        if dx.abs() / dy.abs() < 1.5 && dy.abs() / dx.abs() < 1.5 {
                            return InputDirection::None
                        }
    
                        // TEMP
                        let dir = if dx.abs() > dy.abs() {
                            if dx < 0. { InputDirection::Left } else { InputDirection::Right }
                        } else {
                            if dy < 0. { InputDirection::Down } else { InputDirection::Up }
                        };
    
                        touch_state.insert(*id, Touch { start: touch.position, time: Instant::init(), dir: Some(dir) });
                        return dir;
                    }

                    // let thresh = (settings.swipe_sensitivity as f32 / 100.) * context.get_physical_size().x;
                    // let mut dir = InputDirection::None;
                    // if dx > thresh { dir = InputDirection::Right }
                    // if dx < -thresh { dir = InputDirection::Left }
                    // if dy > thresh { dir = InputDirection::Up }
                    // if dy < -thresh { dir = InputDirection::Down }
                    // if dir != InputDirection::None {
                    //     // if settings.continuous_swipe {
                    //     //     touch_state.insert(*id, touch.position);
                    //     // } else {
                    //     // }
                    //     touch_state.remove(id);
                    //     return dir
                    // }
                }
            },
            TouchPhase::Ended => {
                touch_state.remove(&id);
                return InputDirection::None;
            }
            _ => ()
        }
        if let Some(existing) = touch_state.get(&id) {
            if let Some(dir) = existing.dir {
                if existing.time.elapsed() > 0.1 {
                    touch_state.insert(*id, Touch { start: touch.position, time: Instant::init(), dir: Some(dir) });
                    return dir;
                }
            }
        }
    }
    InputDirection::None
}