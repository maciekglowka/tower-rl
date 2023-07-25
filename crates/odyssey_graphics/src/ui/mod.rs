use rogalik::math::vectors::{Vector2I, Vector2F};
use rogalik::storage::{ComponentSet, Entity, World, WorldEvent};

use odyssey_game::{
    components::{Actor, PlayerCharacter},
    Wind
};

use super::{GraphicsState, GraphicsBackend, SpriteColor};

mod buttons;
mod cards;
mod input;

#[derive(Default)]
pub struct InputState {
    pub mouse_world_position: Vector2F,
    pub mouse_screen_position: Vector2F,
    pub mouse_button_left: ButtonState
}

#[derive(Default, PartialEq)]
pub enum ButtonState {
    #[default]
    Up,
    Down,
    Pressed,
    Released
}

pub fn ui_update(
    world: &mut World,
    input_state: InputState,
    backend: &dyn GraphicsBackend
) {
    draw_wind_queue(world, backend);
    if let Some(clicked_card) = cards::draw_cards(world, backend, &input_state) {
        cards::click_card(clicked_card, world);
    }
    else {
        input::handle_tile_input(world, &input_state);
    }
}


fn draw_wind_queue(world: &World, backend: &dyn GraphicsBackend) {
    let Some(wind) = world.get_resource::<Wind>() else { return };
    for (i, dir) in wind.queue.iter().enumerate() {
        let index = match *dir {
            Vector2I::DOWN => 31,
            Vector2I::UP => 30,
            Vector2I::LEFT => 17,
            Vector2I::RIGHT => 16,
            _ => continue
        };
        backend.draw_ui_sprite(
            "ascii",
            index,
            Vector2F::new(32. * i as f32, 0.),
            Vector2F::new(32., 32.),
            SpriteColor(255, 255, 255, 255)
        );
    }
}
