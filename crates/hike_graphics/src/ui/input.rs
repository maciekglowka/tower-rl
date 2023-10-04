use rogalik::math::vectors::Vector2i;

use hike_game::{
    actions::Pause,
    get_player_position,
    set_player_action,
    set_player_action_from_dir
};
use super::super::world_to_tile;

    

use super::{InputState, InputDirection, ButtonState};

pub fn handle_dir_input(
    world: &mut rogalik::storage::World,
    state: &InputState
) {
    if state.mouse_button_left == ButtonState::Released {
        if let Some(p) = get_player_position(world) {
            if world_to_tile(state.mouse_world_position) == p {
                set_player_action(world, Box::new(Pause));
                return;
            }
        }
    }
    if state.pause == ButtonState::Pressed {
        set_player_action(world, Box::new(Pause));
        return;
    }
    let dir = match state.direction {
        InputDirection::Up => Vector2i::UP,
        InputDirection::Down => Vector2i::DOWN,
        InputDirection::Left => Vector2i::LEFT,
        InputDirection::Right => Vector2i::RIGHT,
        _ => return
    };
    set_player_action_from_dir(world, dir);
}