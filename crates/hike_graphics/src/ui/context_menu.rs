use rogalik::storage::World;

use hike_game::{
    actions::PickItem,
    components::{Item, Player, Position},
    get_player_position,
    set_player_action
};

use super::{InputState, ButtonState, GraphicsBackend, SpriteColor};
use super::buttons::Button;
use super::span::Span;

pub fn handle_menu(
    world: &mut World,
    backend: &dyn GraphicsBackend,
    state: &InputState
) -> bool {
    // true if clicked
    let Some(position) = get_player_position(world) else { return false };

    let item = world.query::<Item>().with::<Position>().iter()
        .filter(|i| i.get::<Position>().unwrap().0 == position)
        .map(|i| i.entity)
        .next();

    if let Some(entity) = item {
        let viewport_size = backend.viewport_size();
        let span = Span::new().with_text_borrowed("PICK");
        let button = Button::new(
                10.,
                85.,
                200.,
                50.
            )
            .with_color(SpriteColor(100, 100, 100, 255))
            .with_span(span);
        button.draw(backend);
        if button.clicked(state) || state.action == ButtonState::Pressed {
            set_player_action(world, Box::new(PickItem { entity }));
            return true;
        }
    }

    false
}
