use rogalik::{
    engine::{
        GraphicsContext, ResourceId,
        input::{MouseButton, KeyCode, TouchPhase},
        Instant
    },
    math::vectors::Vector2f
};
use std::collections::HashMap;

use hike_graphics::game_ui::{ButtonState, InputState, InputDirection};

use super::Context_;

pub struct Touch {
    pub start: Vector2f,
    pub time: Instant,
    pub dir: Option<InputDirection>,
    pub move_attempt: bool
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

    let action_right = key_state(context, KeyCode::KeyE);
    let action_left = key_state(context, KeyCode::KeyQ);

    let mut direction = handle_touches(context, touch_state, settings);
    let touch = direction != InputDirection::None;

    if context.input.is_key_pressed(KeyCode::KeyW) 
        || context.input.is_key_pressed(KeyCode::ArrowUp) { direction = InputDirection::Up }
    if context.input.is_key_pressed(KeyCode::KeyS)
        || context.input.is_key_pressed(KeyCode::ArrowDown){ direction = InputDirection::Down }
    if context.input.is_key_pressed(KeyCode::KeyA)
        || context.input.is_key_pressed(KeyCode::ArrowLeft){ direction = InputDirection::Left }
    if context.input.is_key_pressed(KeyCode::KeyD)
        || context.input.is_key_pressed(KeyCode::ArrowRight){ direction = InputDirection::Right }

    if context.input.is_key_pressed(KeyCode::Space) { direction = InputDirection::Still }

    let digits = [
        key_state(context, KeyCode::Digit0),
        key_state(context, KeyCode::Digit1),
        key_state(context, KeyCode::Digit2),
        key_state(context, KeyCode::Digit3),
        key_state(context, KeyCode::Digit4),
        key_state(context, KeyCode::Digit5),
        key_state(context, KeyCode::Digit6),
        key_state(context, KeyCode::Digit7),
        key_state(context, KeyCode::Digit8),
        key_state(context, KeyCode::Digit9),
    ];
    let item_action = [
        key_state(context, KeyCode::KeyZ),
        key_state(context, KeyCode::KeyX),
        key_state(context, KeyCode::KeyC),
        key_state(context, KeyCode::KeyV),
    ];

    let m = context.input.get_mouse_physical_position();
    let mut w = Vector2f::ZERO;
    if let Some(camera) = context.graphics.get_camera(camera) {
        w = camera.camera_to_world(m);
    }

    InputState {
        mouse_screen_position: m,
        mouse_world_position: w,
        mouse_button_left: left,
        touch,
        direction,
        action_left,
        action_right,
        digits,
        item_action
    }
}

fn key_state(context: &Context_, code: KeyCode) -> ButtonState {
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
                *id, Touch { 
                    start: touch.position,
                    time: Instant::init(),
                    dir: None,
                    move_attempt: false
                });
            },
            TouchPhase::Moved => {
                if let Some(existing) = touch_state.get_mut(id) {
                    existing.move_attempt = true;
                    if existing.dir.is_none() {
                        let dx = touch.position.x - existing.start.x;
                        let dy = touch.position.y - existing.start.y;
                        let d = Vector2f::new(dx, dy);

                        // let screen_x = context.get_physical_size().x;
                        let thresh = (0.1 / settings.swipe_sensitivity.pow(2) as f32)
                            * context.get_physical_size().x;
                        // let thresh = screen_x / (0.005 * settings.swipe_sensitivity.pow(2) as f32 * screen_x);
                        if d.len() < thresh { return InputDirection::None }
                        if dx.abs() / dy.abs() < 2.5 && dy.abs() / dx.abs() < 2.5 {
                            return InputDirection::None
                        }

                        let dir = if dx.abs() > dy.abs() {
                            if dx < 0. { InputDirection::Left } else { InputDirection::Right }
                        } else {
                            if dy < 0. { InputDirection::Down } else { InputDirection::Up }
                        };
    
                        // touch_state.insert(*id, Touch { 
                        //     start: touch.position, time: Instant::init(), dir: Some(dir) 
                        // });
                        existing.time = Instant::init();
                        existing.dir = Some(dir);
                        return dir;
                    }

                }
            },
            TouchPhase::Ended => {
                if let Some(existing) = touch_state.remove(id) {
                    if !existing.move_attempt { return InputDirection::Still }
                    // if existing.dir.is_some() { return InputDirection::None }
                }
                    // match existing.dir {
                    //     Some(InputDirection::None) => return InputDirection::Still,
                    //     // Some(a) => return a,
                    // }
                // return InputDirection::Still
                // return InputDirection::None;
            },
            _ => ()
        }
        if let Some(existing) = touch_state.get_mut(id) {
            if let Some(dir) = existing.dir {
                if existing.time.elapsed() > 0.1 * settings.swipe_repeat_delay as f32 {
                    // touch_state.insert(*id, Touch { 
                    //     start: touch.position, time: Instant::init(), dir: Some(dir) 
                    // });
                    existing.time = Instant::init();
                    return dir;
                }
            }
        }
    }
    InputDirection::None
}