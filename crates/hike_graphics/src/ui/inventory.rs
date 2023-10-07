use rogalik::{
    engine::{Color, GraphicsContext},
    math::vectors::Vector2f,
    storage::{Entity, World}
};

use hike_game::actions::UseCollectable;
use hike_game::components::{Name, Player};
use hike_game::globals::{MAX_COLLECTABLES, MAX_WEAPONS};
use hike_game::set_player_action;

use super::{InputState, ButtonState, get_viewport_bounds};
use super::buttons::Button;
use super::span::Span;
use super::super::globals::{UI_BUTTON_HEIGHT, UI_GAP, UI_BUTTON_TEXT_SIZE, BUTTON_COLOR, BUTTON_COLOR_SELECTED};
use super::utils::get_item_span;

pub fn handle_inventory(
    world: &World,
    context: &mut crate::Context_,
    state: &InputState
) {
    let mut click = (None, None);

    {
        let query = world.query::<Player>().build();
        let Some(player) = query.single::<Player>() else { return };
        click.0 = handle_inventory_buttons(
            world,
            0,
            &player.weapons.to_vec(),
            Some(player.active_weapon),
            context,
            state
        );
        click.1 = handle_inventory_buttons(
            world,
            1,
            &(0..MAX_COLLECTABLES).map(|i| player.collectables.get(i).map(|a| *a)).collect(),
            None,
            context,
            state
        );
    }

    if state.digits[1] == ButtonState::Pressed { click.0 = Some(0) };
    if state.digits[2] == ButtonState::Pressed { click.0 = Some(1) };
    if state.digits[3] == ButtonState::Pressed { click.0 = Some(2) };

    if let Some(click) = click.0 {
        click_weapon(click, world);
    }
    if let Some(click) = click.1 {
        click_item(click, world);
    }
}

fn handle_inventory_buttons(
    world: &World,
    row: u32,
    entities: &Vec<Option<Entity>>,
    active: Option<usize>,
    context: &mut crate::Context_,
    state: &InputState,
) -> Option<usize> {
    // return item index if clicked
    let bounds = get_viewport_bounds(context);
    let mut clicked = None;
    let width = (bounds.1.x - bounds.0.x - UI_GAP) / (entities.len() as f32) - UI_GAP;

    for (i, entity) in entities.iter().enumerate() {
        let color = if Some(i) == active {
            BUTTON_COLOR_SELECTED
        } else {
            BUTTON_COLOR
        };

        let offset = UI_GAP + i as f32 * (UI_GAP + width);

        let mut button = Button::new(
                bounds.0.x + offset,
                bounds.0.y + UI_GAP + row as f32 * (UI_GAP + UI_BUTTON_HEIGHT),
                width,
                UI_BUTTON_HEIGHT
            )
            .with_color(color);

        if let Some(entity) = entity {
            let mut span = get_item_span(*entity, world);
            span = span.with_size(UI_BUTTON_TEXT_SIZE);
    
            button = button.with_span(span);
        }

        button.draw(context);
        if button.clicked(state) {
            clicked = Some(i)
        }
    }

    clicked
}

fn click_weapon(index: usize, world: &World) {
    world.query::<Player>().build()
        .single_mut::<Player>()
        .unwrap()
        .active_weapon = index;
}

fn click_item(index: usize, world: &World) {
    let entity = if let Some(player) = world.query::<Player>().build().single::<Player>() {
        player.collectables.get(index).map(|&e| e)
    } else { return };
    if let Some(entity) = entity {
        set_player_action(world, Box::new(UseCollectable { entity }));
    }
}

// pub fn handle_shift_input(world: &World, state: &InputState) {
//     if state.shift == ButtonState::Pressed {
//         if let Some(player) = world.query::<Player>().build().single::<Player>() {
//             click_item((player.active_weapon + 1) % MAX_WEAPONS, world);
//         }
//     }
// }
