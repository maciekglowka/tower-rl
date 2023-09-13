use rogalik::storage::{Component, World};

use hike_data::GameData;
use hike_game::components::{Durability, Hit, Poison, Stun, Name, Player};
use hike_game::globals::INVENTORY_SIZE;

use super::{InputState, ButtonState, GraphicsBackend, SpriteColor};
use super::buttons::Button;
use super::span::Span;
use super::super::globals::{UI_BUTTON_HEIGHT, UI_GAP, UI_BUTTON_TEXT_SIZE, BUTTON_COLOR, BUTTON_COLOR_SELECTED};

pub fn handle_inventory(
    world: &World,
    backend: &dyn GraphicsBackend,
    state: &InputState,
    scale: f32
) -> Option<usize> {
    // return item index if clicked
    let query = world.query::<Player>();
    let player_item = query.iter().next()?;
    let player = player_item.get::<Player>()?;

    let viewport_size = backend.viewport_size();

    let mut clicked = None;
    let height = UI_BUTTON_HEIGHT * scale;
    let gap = UI_GAP * scale;
    let width = (viewport_size.x - gap) / INVENTORY_SIZE as f32 - gap;

    for i in 0..INVENTORY_SIZE {
        let color = if i == player.active_item {
            BUTTON_COLOR_SELECTED
        } else {
            BUTTON_COLOR
        };

        let offset = gap + i as f32 * (gap + width);

        let mut button = Button::new(
                offset,
                viewport_size.y - gap - height,
                width,
                height
            )
            .with_color(color);

        let game_data = world.get_resource::<GameData>().unwrap();

        if let Some(entity) = player.items[i] {
            if let Some(name) = world.get_component::<Name>(entity) {
                if let Some(data) = game_data.entities.get(&name.0) {
                    let mut span = Span::new()
                        .with_sprite(
                            &data.sprite.atlas_name,
                            data.sprite.index
                        )
                        .with_size((scale * UI_BUTTON_TEXT_SIZE as f32) as u32)
                        .with_sprite_color(data.sprite.color)
                        .with_text_color(SpriteColor(255, 255, 255, 255));

                    // TODO - automate
                    // if let Some(hit) = world.get_component::<Hit>(entity) {
                    //     span = span.with_text_owned(
                    //         format!("H{}", hit.0)
                    //     );
                    // };
                    // if let Some(stun) = world.get_component::<Stun>(entity) {
                    //     span = span.with_text_owned(
                    //         format!("S{}", stun.0)
                    //     );
                    // };
                    // if let Some(poison) = world.get_component::<Poison>(entity) {
                    //     span = span.with_text_owned(
                    //         format!("P{}", poison.0)
                    //     );
                    // };
                    // if let Some(durability) = world.get_component::<Durability>(entity) {
                    //     span = span.with_text_owned(
                    //         format!("D{}", durability.0)
                    //     );
                    // };
                    let attrs = world.get_entity_components(entity).iter()
                        .map(|c| c.as_str())
                        .filter(|s| s.len() > 0)
                        .collect::<Vec<_>>()
                        .join(" ");
                    if attrs.len() > 0 {
                        span = span.with_text_owned(attrs);
                    }

                    button = button.with_span(span);

                }
            }
        }
        button.draw(backend);
        if button.clicked(state) {
                clicked = Some(i)
            }
    }

    clicked
}

pub fn click_item(index: usize, world: &World) {
    world.query::<Player>().iter()
        .next()
        .unwrap()
        .get_mut::<Player>()
        .unwrap()
        .active_item = index;
}

pub fn handle_shift_input(world: &World, state: &InputState) {
    if state.shift == ButtonState::Pressed {
        let query = world.query::<Player>();
        let Some(item) = query.iter().next() else { return };
        let active = item.get::<Player>().unwrap().active_item;
        click_item((active + 1) % INVENTORY_SIZE, world);
    }
}
