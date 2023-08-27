use rogalik::storage::{Component, World};

use hike_data::GameData;
use hike_game::components::{Durability, Name, Player};
use hike_game::globals::INVENTORY_SIZE;

use super::{InputState, GraphicsBackend, SpriteColor};
use super::buttons::Button;
use super::span::Span;
use super::super::globals::INVENTORY_SINGLE_DIM;

pub fn handle_inventory(
    world: &World,
    backend: &dyn GraphicsBackend,
    state: &InputState
) -> Option<usize> {
    // return item index if clicked
    let query = world.query::<Player>();
    let player_item = query.iter().next()?;
    let player = player_item.get::<Player>()?;

    let viewport_size = backend.viewport_size();

    let mut clicked = None;
    let gap = (viewport_size.x - INVENTORY_SIZE as f32 * INVENTORY_SINGLE_DIM) / (INVENTORY_SIZE + 1) as f32;

    for i in 0..INVENTORY_SIZE {
        let color = if i == player.active_item {
            SpriteColor(255, 255, 255, 255)
        } else {
            SpriteColor(128, 128, 128, 255)
        };

        let offset = gap * (i + 1) as f32 + INVENTORY_SINGLE_DIM * i as f32;

        let mut button = Button::new(
                offset,
                viewport_size.y - 1.5 * INVENTORY_SINGLE_DIM as f32,
                INVENTORY_SINGLE_DIM,
                INVENTORY_SINGLE_DIM
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
                        .with_sprite_color(data.sprite.color)
                        .with_text_color(SpriteColor(0, 0, 0, 255));

                    if let Some(durability) = world.get_component::<Durability>(entity) {
                        span = span.with_text_owned(
                            format!("({})", durability.value)
                        );
                    };

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
    // if state.shift == ButtonState::Pressed {
    //     let query = world.query::<PlayerCharacter>().with::<Actor>();
    //     let Some(item) = query.iter().next() else { return };
    //     let count = &item.get::<Actor>().unwrap().abilities.len();
    //     let active = item.get::<PlayerCharacter>().unwrap().active_ability;
    //     click_card((active + 1) % count, world);
    // }
}