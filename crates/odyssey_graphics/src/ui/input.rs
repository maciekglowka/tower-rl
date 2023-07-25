use odyssey_game::actions::SelectedAction;

use crate::world_to_tile;
use super::{ButtonState, InputState};

pub fn handle_tile_input(
    world: &rogalik::storage::World,
    state: &InputState
) {
    if state.mouse_button_left != ButtonState::Released { return }

    let query = world.query::<odyssey_game::components::PlayerCharacter>();
    let Some(item) = query.iter().next() else { return };
    let tile = world_to_tile(state.mouse_world_position);

    let Some(mut actor) = item.get_mut::<odyssey_game::components::Actor>() else { return };
    let active = item.get::<odyssey_game::components::PlayerCharacter>().unwrap().active_card;
    let card_entity = actor.cards[active];
    let Some(card) = world.get_component::<odyssey_game::components::Card>(card_entity) else { return };

    if let Some(cooldown) = world.get_component::<odyssey_game::components::Cooldown>(card_entity) {
        if cooldown.current > 0 { return }
    }

    if let Some(action) = card.0.get_possible_actions(item.entity, world).remove(&tile) {
        actor.action = Some(SelectedAction { action, card: Some(card_entity) });
    }
}