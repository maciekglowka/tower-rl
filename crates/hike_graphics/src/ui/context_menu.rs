use rogalik::storage::World;

use hike_game::{
    actions::{Action, PendingActions, PickItem},
    components::{Item, Player, Position}
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
    let query = world.query::<Player>().with::<Position>();
    let Some(player_item) = query.iter().next() else { return false };
    
    let position = player_item.get::<Position>().unwrap().0;

    let item = world.query::<Item>().with::<Position>().iter()
        .filter(|i| i.get::<Position>().unwrap().0 == position)
        .map(|i| i.entity)
        .next();

    if let Some(entity) = item {
        let viewport_size = backend.viewport_size();
        let span = Span::new().with_text_borrowed("PICK");
        let button = Button::new(
                viewport_size.x / 2. - 100.,
                50.,
                200.,
                50.
            )
            .with_color(SpriteColor(100, 100, 100, 255))
            .with_span(span);
        button.draw(backend);
        if button.clicked(state) || state.action == ButtonState::Pressed {
            world.get_resource_mut::<PendingActions>().unwrap().0
                .push_back(Box::new(
                    PickItem { entity }
                ) as Box<dyn Action>);
            return true;
        }
    }

    false
}
