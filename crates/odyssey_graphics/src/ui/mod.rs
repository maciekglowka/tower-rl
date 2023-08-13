use rogalik::math::vectors::{Vector2I, Vector2F};
use rogalik::storage::{ComponentSet, Entity, World, WorldEvent};

use odyssey_game::{
    components::{Actor, PlayerCharacter},
};

use super::{GraphicsState, GraphicsBackend, SpriteColor};

mod buttons;
mod cards;
mod input;
mod status;

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
    status::draw_status(world, backend);
    if let Some(clicked_card) = cards::draw_cards(world, backend, &input_state) {
        cards::click_card(clicked_card, world);
    }
    else {
        input::handle_tile_input(world, &input_state);
    }
}
