use rogalik::storage::{Component, World};

use odyssey_game::components::{Actor, Card, Cooldown, PlayerCharacter};

use super::{InputState, GraphicsBackend, SpriteColor};
use super::buttons::Button;

pub fn draw_cards(world: &World, backend: &dyn GraphicsBackend, state: &InputState) -> Option<usize> {
    let query = world.query::<PlayerCharacter>().with::<Actor>();
    let item = query.iter().next()?;
    let cards = &item.get::<Actor>().unwrap().cards;
    let active = item.get::<PlayerCharacter>()?.active_card;

    let viewport_size = backend.viewport_size();

    let mut clicked = None;
    for (i, entity) in cards.iter().enumerate() {
        // let desc = world.get_entity_components(*card)
        //     .iter()
        //     .map(|c| c.as_str())
        //     .collect::<Vec<_>>();
        let mut desc = if let Some(card) = world.get_component::<Card>(*entity) {
            card.as_str()
        } else {
            String::new()
        };
        if let Some(cooldown) = world.get_component::<Cooldown>(*entity) {
            desc += &format!(" ({})", cooldown.current);
        }

        // let desc = desc.join(", ");
        let color = if i == active {
            SpriteColor(255, 255, 255, 255)
        } else {
            SpriteColor(128, 128, 128, 255)
        };
        if Button::new(
                32.,
                viewport_size.y - 48. * (i as f32 + 1.),
                250.,
                32.
            )
            .with_text(&desc, SpriteColor(0, 0, 0, 255), 32)
            .with_color(color)
            .draw(backend)
            .clicked(state) {
                clicked = Some(i)
            }
    }
    clicked
}

pub fn click_card(index: usize, world: &World) {
    world.query::<PlayerCharacter>().iter()
        .next()
        .unwrap()
        .get_mut::<PlayerCharacter>()
        .unwrap()
        .active_card = index;
}