use rogalik::math::vectors::Vector2i;

use hike_game::{
    actions::Pause,
    globals::BOARD_SIZE,
    get_player_position,
    set_player_action,
    set_player_action_from_dir,
};
use super::get_viewport_bounds;
use super::super::world_to_tile;
use crate::globals::UI_BOTTOM_SAFE_AREA;
    

use super::{UiState, InputState, InputDirection, ButtonState};

pub fn handle_dir_input(
    world: &mut rogalik::storage::World,
    input_state: &mut InputState,
    ui_state: &mut UiState,
    context: &mut crate::Context_,
) {
    if input_state.pause == ButtonState::Pressed {
        if set_player_action(world, Box::new(Pause)) {
            ui_state.direction_buffer = None;
            return;
        }
    }
    let bounds = get_viewport_bounds(context);
    let world_input = input_state.mouse_world_position.y > bounds.0.y + UI_BOTTOM_SAFE_AREA;

    if input_state.direction != InputDirection::None && world_input {
        ui_state.direction_buffer = Some(input_state.direction);
    }

    if let Some(buffer) = ui_state.direction_buffer {
        let dir = match buffer {
            InputDirection::Up => Vector2i::UP,
            InputDirection::Down => Vector2i::DOWN,
            InputDirection::Left => Vector2i::LEFT,
            InputDirection::Right => Vector2i::RIGHT,
            _ => return
        };
        if set_player_action_from_dir(world, dir) {
            ui_state.direction_buffer = None;
            return
        }
    }

    if input_state.mouse_button_left == ButtonState::Released && world_input {
        // let t = world_to_tile(input_state.mouse_world_position);
        // if t.x >= 0 && t.y >= 0 && t.x < BOARD_SIZE as i32 && t.y < BOARD_SIZE as i32 {
        set_player_action(world, Box::new(Pause));
        ui_state.direction_buffer = None;
        return;
    }
}