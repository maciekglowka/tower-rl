use rogalik::math::vectors::Vector2i;

use hike_game::{
    actions::Pause,
    set_player_action,
    set_player_action_from_dir,
};
use super::get_viewport_bounds;
use super::context_menu::CONTEXT_VISIBLE;
use crate::globals::{UI_BOTTOM_SAFE_AREA, UI_BUTTON_HEIGHT};
    

use super::{UiState, InputState, InputDirection, ButtonState};

pub fn handle_dir_input(
    world: &mut rogalik::storage::World,
    input_state: &mut InputState,
    ui_state: &mut UiState,
    context: &mut crate::Context_,
) {
    // if input_state.pause == ButtonState::Pressed && set_player_action(world, Box::new(Pause)) {
        //     ui_state.direction_buffer = None;
        //     return;
        // }
        
    let bounds = get_viewport_bounds(context);
    let mut safe_area = UI_BOTTOM_SAFE_AREA;
    if CONTEXT_VISIBLE.load(core::sync::atomic::Ordering::Relaxed) {
        safe_area += UI_BUTTON_HEIGHT;
    }
    let world_input = !input_state.touch
        || input_state.mouse_world_position.y > bounds.0.y + safe_area;

    if input_state.direction != InputDirection::None {
        ui_state.direction_buffer = Some((input_state.direction, world_input));
    }

    // if input_state.direction != InputDirection::None && (world_input || !input_state.touch) {
    //     ui_state.direction_buffer = Some(input_state.direction);
    // }

    if let Some((buffer, is_world)) = ui_state.direction_buffer {
        if !is_world { return };
        let dir = match buffer {
            InputDirection::Up => Some(Vector2i::UP),
            InputDirection::Down => Some(Vector2i::DOWN),
            InputDirection::Left => Some(Vector2i::LEFT),
            InputDirection::Right => Some(Vector2i::RIGHT),
            _ => None
        };
        if let Some(dir) = dir {
            if set_player_action_from_dir(world, dir) {
                ui_state.direction_buffer = None;
                return
            }
        }
        if buffer == InputDirection::Still && set_player_action(world, Box::new(Pause)) {
            ui_state.direction_buffer = None;
        }
    }
    // if input_state.mouse_button_left == ButtonState::Released && world_input {
    //     set_player_action(world, Box::new(Pause));
    // }
}