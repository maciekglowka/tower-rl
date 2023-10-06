use rogalik::{
    engine::{Color, GraphicsContext},
    storage::{Entity, World}
};

use hike_data::{EntityData, GameData};
use hike_game::components::{Name, Player};
use hike_game::globals::{MAX_COLLECTABLES, MAX_WEAPONS};

use super::{InputState, ButtonState, get_viewport_bounds};
use super::buttons::Button;
use super::span::Span;
use super::super::globals::{UI_BUTTON_HEIGHT, UI_GAP, UI_BUTTON_TEXT_SIZE, BUTTON_COLOR, BUTTON_COLOR_SELECTED};
use super::utils::get_entity_icons;

pub fn handle_inventory(
    world: &World,
    context: &mut crate::Context_,
    state: &InputState
) -> Option<usize> {
    // return item index if clicked
    let query = world.query::<Player>().build();
    let player = query.single::<Player>()?;

    let bounds = get_viewport_bounds(context);
    let mut clicked = None;
    let width = (bounds.1.x - bounds.0.x - UI_GAP) / (MAX_WEAPONS as f32) - UI_GAP;

    for i in 0..MAX_WEAPONS {
        let color = if i == player.active_weapon {
            BUTTON_COLOR_SELECTED
        } else {
            BUTTON_COLOR
        };

        let offset = UI_GAP + i as f32 * (UI_GAP + width);

        let mut button = Button::new(
                bounds.0.x + offset,
                bounds.0.y + UI_GAP,
                width,
                UI_BUTTON_HEIGHT
            )
            .with_color(color);

        let game_data = world.get_resource::<GameData>().unwrap();

        if let Some(entity) = player.weapons[i] {
            if let Some(name) = world.get_component::<Name>(entity) {
                if let Some(data) = game_data.entities.get(&name.0) {
                    let mut span = get_item_span(entity, world, data);
                    span = span.with_size(UI_BUTTON_TEXT_SIZE);

                    button = button.with_span(span);

                }
            }
        }
        button.draw(context);
        if button.clicked(state) {
            clicked = Some(i)
        }
    }

    if state.digits[1] == ButtonState::Pressed { clicked = Some(0) };
    if state.digits[2] == ButtonState::Pressed { clicked = Some(1) };
    if state.digits[3] == ButtonState::Pressed { clicked = Some(2) };

    clicked
}

pub fn click_item(index: usize, world: &World) {
    world.query::<Player>().build()
        .single_mut::<Player>()
        .unwrap()
        .active_weapon = index;
}

pub fn handle_shift_input(world: &World, state: &InputState) {
    if state.shift == ButtonState::Pressed {
        if let Some(player) = world.query::<Player>().build().single::<Player>() {
            click_item((player.active_weapon + 1) % MAX_WEAPONS, world);
        }
    }
}

fn get_item_span<'a>(entity: Entity, world: &World, data: &'a EntityData) -> Span<'a> {
    let mut span = Span::new()
        .with_text_color(Color(255, 255, 255, 255));

    let icons = get_entity_icons(entity, world);
    let mut it = icons.iter().peekable();
    while let Some((idx, val)) = it.next() {
        span = span.with_sprite("icons", *idx);
        if let Some(val) = val {
            span = span.with_text_owned(format!("{}", val));
        }
        if it.peek().is_some() {
            // non-last element
            span = span.with_spacer(0.2);
        }
    }
    span
}