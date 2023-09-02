use rogalik::storage::World;

use hike_game::{
    actions::{Action, Consume, Interact, AddToInventory},
    components::{Consumable, Interactive, Item, Offensive, Position},
    get_player_position,
    get_player_entity,
    set_player_action,
    get_entities_at_position
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
    let player = get_player_entity(world).unwrap();

    let entities = get_entities_at_position(world, position);

    // let item = world.query::<Item>().with::<Position>().iter()
    //     .filter(|i| i.get::<Position>().unwrap().0 == position)
    //     .map(|i| i.entity)
    //     .next();
    
    for (i, entity) in entities.iter().enumerate() {
        let (text, action) = match entity {
            e if world.get_component::<Interactive>(*e).is_some() =>
                ("USE", Box::new(Interact { entity: *e}) as Box<dyn Action>),
            e if world.get_component::<Consumable>(*e).is_some() =>
                ("USE", Box::new(Consume { entity: *e, consumer: player }) as Box<dyn Action>),
            e if world.get_component::<Item>(*e).is_some() && world.get_component::<Offensive>(*e).is_some() =>
                ("PICK", Box::new(AddToInventory { entity: *e }) as Box<dyn Action>),
            _ => continue
        };

        let span = Span::new().with_text_borrowed(text);
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
            set_player_action(world, action);
            return true;
        }
    }

    // if let Some(entity) = item {
    //     let text = if world.get_component::<Consumable>(entity).is_some() { "USE" } else { "PICK" };
    //     let span = Span::new().with_text_borrowed(text);
    //     let button = Button::new(
    //             10.,
    //             85.,
    //             200.,
    //             50.
    //         )
    //         .with_color(SpriteColor(100, 100, 100, 255))
    //         .with_span(span);
    //     button.draw(backend);
    //     if button.clicked(state) || state.action == ButtonState::Pressed {
    //         set_player_action(world, Box::new(PickItem { entity }));
    //         return true;
    //     }
    // }

    false
}
