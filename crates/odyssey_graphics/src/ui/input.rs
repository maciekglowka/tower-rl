use rogalik::math::vectors::Vector2I;

use odyssey_game::{
    components::{PlayerCharacter, Position, Actor},
    get_ability_actions
};

use crate::world_to_tile;
use super::{ButtonState, InputState, InputDirection};

pub fn handle_tile_input(
    world: &rogalik::storage::World,
    state: &InputState
) {
    // if state.mouse_button_left != ButtonState::Released { return }
    let query = world.query::<PlayerCharacter>().with::<Position>();
    let Some(item) = query.iter().next() else { return };
    let position = item.get::<Position>().unwrap().0;

    let mut tile = None;
    if state.mouse_button_left == ButtonState::Released {
        tile = Some(world_to_tile(state.mouse_world_position));
    }
    match state.direction {
        InputDirection::Up => tile = Some(position + Vector2I::UP),
        InputDirection::Down => tile = Some(position + Vector2I::DOWN),
        InputDirection::Left => tile = Some(position + Vector2I::LEFT),
        InputDirection::Right => tile = Some(position + Vector2I::RIGHT),
        InputDirection::Still => tile = Some(position + Vector2I::ZERO),
        _ => ()
    }

    if tile.is_none() { return }

    // let tile = world_to_tile(state.mouse_world_position);

    let Some(actor) = item.get::<Actor>() else { return };

    // do not borrow PC mutably yet, as it might interfere with ability actions
    let ability = if let Some(player) = item.get::<PlayerCharacter>() {
        actor.abilities[player.active_ability]
    } else { return };

    if let Some(action) = get_ability_actions(item.entity, &ability, world).remove(&tile.unwrap()) {
        item.get_mut::<PlayerCharacter>()
            .unwrap().selected_action = Some(action);
    }
}