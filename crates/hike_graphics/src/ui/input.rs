use rogalik::math::vectors::Vector2i;

use hike_game::{
    actions::Pause,
    globals::BOARD_SIZE,
    get_player_position,
    set_player_action,
    set_player_action_from_dir,
};
use super::super::world_to_tile;

    

use super::{InputState, InputDirection, ButtonState};

pub fn handle_dir_input(
    world: &mut rogalik::storage::World,
    state: &mut InputState
) {
    if state.mouse_button_left == ButtonState::Released {
        let t = world_to_tile(state.mouse_world_position);
        if t.x >= 0 && t.y >= 0 && t.x < BOARD_SIZE as i32 && t.y < BOARD_SIZE as i32 {
            set_player_action(world, Box::new(Pause));
            state.direction_buffer = None;
            return;
        }
    }

    if state.pause == ButtonState::Pressed {
        if set_player_action(world, Box::new(Pause)) {
            state.direction_buffer = None;
            return;
        }
    }

    if let Some(buffer) = state.direction_buffer {
        let dir = match buffer {
            InputDirection::Up => Vector2i::UP,
            InputDirection::Down => Vector2i::DOWN,
            InputDirection::Left => Vector2i::LEFT,
            InputDirection::Right => Vector2i::RIGHT,
            _ => return
        };
        if set_player_action_from_dir(world, dir) {
            state.direction_buffer = None;
            return
        }
    }
    // println!("Setting buffer to {:?}", state.direction);
    if state.direction != InputDirection::None {
        state.direction_buffer = Some(state.direction);
    }
}