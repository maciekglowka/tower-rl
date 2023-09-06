use rogalik::{
    math::vectors::Vector2F,
    storage::{Entity, World}
};
use core::sync::atomic::{AtomicUsize, Ordering::Relaxed};

use hike_game::{
    actions::{Action, Consume, Interact, AddToInventory},
    components::{Consumable, Interactive, Item, Offensive, Name, Position},
    get_player_position,
    get_player_entity,
    set_player_action,
    get_entities_at_position
};

use super::super::globals::{UI_BUTTON_HEIGHT, UI_GAP, UI_BUTTON_TEXT_SIZE};
use super::{InputState, ButtonState, GraphicsBackend, SpriteColor};
use super::buttons::Button;
use super::span::Span;

static CONTEXT_IDX: AtomicUsize = AtomicUsize::new(0);

pub fn handle_menu(
    world: &mut World,
    backend: &dyn GraphicsBackend,
    state: &InputState
) -> bool {
    // true if clicked
    let Some(position) = get_player_position(world) else { return false };
    let player = get_player_entity(world).unwrap();

    let mut entities = get_entities_at_position(world, position)
        .iter()
        .filter(|&&e| world.get_component::<Interactive>(e).is_some() ||
            world.get_component::<Item>(e).is_some()
        )
        .map(|&e| e)
        .collect::<Vec<_>>();

    if entities.len() == 0 { return false };

    entities.sort_by_key(|a| (a.version, a.id));

    let cur_idx = CONTEXT_IDX.load(Relaxed);
    CONTEXT_IDX.store(cur_idx % entities.len(), Relaxed);

    let viewport_size = backend.viewport_size();
    let y = viewport_size.y - 2.0 * (UI_BUTTON_HEIGHT + UI_GAP);
    let width = match entities.len() {
        0 => return false,
        1 => viewport_size.x - 2.0 * UI_GAP,
        _ => (viewport_size.x - 3.0 * UI_GAP) / 2.0
    };

    if entities.len() > 1 {
        // draw `next` button
        let button = Button::new(
                2.0 * UI_GAP + width,
                y,
                width,
                UI_BUTTON_HEIGHT
            )
            .with_color(SpriteColor(100, 100, 100, 255))
            .with_span(Span::new().with_text_borrowed("[MORE]").with_size(UI_BUTTON_TEXT_SIZE));
        button.draw(backend);
        if button.clicked(state) {
            CONTEXT_IDX.store(cur_idx + 1, Relaxed);
            return true;
        }
    };

    
    for (i, entity) in entities.iter().enumerate() {
        if i != cur_idx { continue };

        let (text, action) = match entity {
            e if world.get_component::<Interactive>(*e).is_some() =>
                ("USE", Box::new(Interact { entity: *e}) as Box<dyn Action>),
            e if world.get_component::<Consumable>(*e).is_some() =>
                ("USE", Box::new(Consume { entity: *e, consumer: player }) as Box<dyn Action>),
            e if world.get_component::<Item>(*e).is_some() && world.get_component::<Offensive>(*e).is_some() =>
                ("PICK", Box::new(AddToInventory { entity: *e }) as Box<dyn Action>),
            _ => continue
        };

        if let Some(s) = get_item_desc(world, *entity) {
            backend.draw_ui_text(
                "default",
                &format!("{}", s),
                Vector2F::new(10., 74.),
                32,
                SpriteColor(0, 0, 0, 255)
            );
        }

        let span = Span::new().with_text_borrowed(text).with_size(UI_BUTTON_TEXT_SIZE);
        let button = Button::new(
                UI_GAP,
                y,
                width,
                UI_BUTTON_HEIGHT
            )
            .with_color(SpriteColor(100, 100, 100, 255))
            .with_span(span);
        button.draw(backend);
        if button.clicked(state) || state.action == ButtonState::Pressed {
            set_player_action(world, action);
            return true;
        }
    }

    false
}


fn get_item_desc(world: &World, entity: Entity)-> Option<String> {
    if world.get_component::<Item>(entity).is_none()
        && world.get_component::<Interactive>(entity).is_none() { return None };

    let name = world.get_component::<Name>(entity)?.0.clone();
    let s = world.get_entity_components(entity).iter()
        .map(|c| c.as_str())
        .filter(|s| s.len() > 0)
        .collect::<Vec<_>>()
        .join(" ");
    return Some(format!("{}: {}", name, s));
}