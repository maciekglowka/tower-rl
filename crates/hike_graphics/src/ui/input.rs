use rogalik::math::vectors::Vector2I;

use hike_game::{
    components::{Player, Position},
    get_action_at_dir
};

use super::{InputState, InputDirection};

pub fn handle_dir_input(
    world: &rogalik::storage::World,
    state: &InputState
) {
    // if state.mouse_button_left != ButtonState::Released { return }
    let query = world.query::<Player>().with::<Position>();
    let Some(item) = query.iter().next() else { return };

    let dir = match state.direction {
        InputDirection::Up => Vector2I::UP,
        InputDirection::Down => Vector2I::DOWN,
        InputDirection::Left => Vector2I::LEFT,
        InputDirection::Right => Vector2I::RIGHT,
        _ => return
    };
    item.get_mut::<Player>()
        .unwrap().action = get_action_at_dir(item.entity, world, dir);
}