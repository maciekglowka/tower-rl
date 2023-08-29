use rogalik::math::vectors::Vector2I;

use hike_game::set_player_action_from_dir;

use super::{InputState, InputDirection};

pub fn handle_dir_input(
    world: &mut rogalik::storage::World,
    state: &InputState
) {
    let dir = match state.direction {
        InputDirection::Up => Vector2I::UP,
        InputDirection::Down => Vector2I::DOWN,
        InputDirection::Left => Vector2I::LEFT,
        InputDirection::Right => Vector2I::RIGHT,
        _ => return
    };
    set_player_action_from_dir(world, dir);
}