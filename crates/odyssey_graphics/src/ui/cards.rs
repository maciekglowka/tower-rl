use rogalik::storage::{Component, World};

use odyssey_game::components::{Actor, PlayerCharacter};

use super::{InputState, GraphicsBackend, SpriteColor, ButtonState};
use super::buttons::Button;

pub fn draw_cards(world: &World, backend: &dyn GraphicsBackend, state: &InputState) -> Option<usize> {
    let query = world.query::<PlayerCharacter>().with::<Actor>();
    let item = query.iter().next()?;
    let abilities = &item.get::<Actor>().unwrap().abilities;
    let active = item.get::<PlayerCharacter>()?.active_ability;

    let viewport_size = backend.viewport_size();

    let mut clicked = None;
    for (i, ability) in abilities.iter().enumerate() {
        let mut desc = ability.as_str().to_owned();
        // if let Some(cooldown) = ability.cooldown {
        //     desc += &format!(" ({})", cooldown.current);
        // }
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
        .active_ability = index;
}

pub fn handle_shift_input(world: &World, state: &InputState) {
    if state.shift == ButtonState::Pressed {
        let query = world.query::<PlayerCharacter>().with::<Actor>();
        let Some(item) = query.iter().next() else { return };
        let count = &item.get::<Actor>().unwrap().abilities.len();
        let active = item.get::<PlayerCharacter>().unwrap().active_ability;
        click_card((active + 1) % count, world);
    }
}