use rogalik::{
    math::vectors::Vector2f,
    storage::World
};

use hike_game::{
    components::{Info, Stunned},
    GameEvent,
    get_entities_at_position, get_player_position, get_player_entity
};

use super::super::globals::{UI_GAP, UI_TEXT_GAP, UI_STATUS_TEXT_SIZE};
use super::get_viewport_bounds;
use super::text_box::TextBox;
use super::UiState;

pub fn update_messages(
    world: &World,
    context: &mut crate::Context_,
    ui_state: &mut UiState
) {
    handle_info(world, ui_state);
    handle_stun(world, ui_state);
    draw_messages(context, ui_state);
}

pub fn draw_messages(
    context: &mut crate::Context_,
    ui_state: &UiState
) {
    let Some(message) = &ui_state.message else { return };
    let bounds = get_viewport_bounds(context);
    let v = Vector2f::new(
        bounds.0.x + 2. * UI_GAP,
        bounds.1.y - UI_GAP  - 2. * (UI_STATUS_TEXT_SIZE + UI_TEXT_GAP)
    );
    TextBox::new()
        .with_text_borrowed(message)
        .with_size(UI_STATUS_TEXT_SIZE)
        .draw(
            v,
            bounds.1.x - bounds.0.x - 4. * UI_GAP,
            context
        );
}

pub fn handle_stun(world: &World, ui_state: &mut UiState) {
    if let Some(player) = get_player_entity(world) {
        if world.get_component::<Stunned>(player).is_none() { return }
        ui_state.message = Some("Suddenly, you can't move!".to_string());
    }
}

pub fn handle_info(world: &World, ui_state: &mut UiState) {
    let Some(player_v) = get_player_position(world) else { return };
    let entities = get_entities_at_position(world, player_v);
    let mut infos = entities.iter()
        .filter_map(|&e| world.get_component::<Info>(e))
        .map(|a| a.text.to_string());

    // replace only if Some
    if let Some(text) = infos.next() {
        ui_state.message = Some(text);
    }
    
}

pub fn handle_game_event(
    ev: &GameEvent,
    world: &World,
    state: &mut UiState
) {
    let mut text = None;
    match ev {
        GameEvent::Poison(entity, _) => {
            if Some(*entity) == get_player_entity(world) {
                text = Some("You feel sick!");
            }
        },
        // GameEvent::Heal(entity) => {
        //     if Some(*entity) == get_player_entity(world) {
        //         text = Some("You feel stronger!");
        //     }
        // },        
        GameEvent::HealPoison(entity) => {
            if Some(*entity) == get_player_entity(world) {
                text = Some("Suddenly, your blood seems clear!");
            }
        },        
        GameEvent::Immunity(entity) => {
            if Some(*entity) == get_player_entity(world) {
                text = Some("You feel invincible!");
            }
        },
        GameEvent::Regeneration(entity) => {
            if Some(*entity) == get_player_entity(world) {
                text = Some("Slowly you regain your strength!");
            }
        },
        GameEvent::Travel(entity, _) => {
            if Some(*entity) == get_player_entity(world) {
                state.message = None;
            }
        }
        _ => {}
    }
    // replace only if Some
    if let Some(text) = text {
        state.message = Some(text.to_string());
    }
}
