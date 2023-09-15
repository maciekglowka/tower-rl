use rogalik::{
    math::vectors::Vector2F,
    storage::{Entity, World}
};
use core::sync::atomic::{AtomicUsize, Ordering::Relaxed};

use hike_game::{
    actions::{Action, Consume, Interact, AddToInventory, Pause},
    components::{Consumable, Interactive, Item, Name, Hit, Poison, Stun},
    get_player_position,
    get_player_entity,
    set_player_action,
    get_entities_at_position
};

use super::super::globals::{
    UI_BUTTON_HEIGHT, UI_GAP, UI_TEXT_GAP, UI_BUTTON_TEXT_SIZE, BUTTON_COLOR, UI_STATUS_TEXT_SIZE
};
use super::{InputState, ButtonState, GraphicsBackend, SpriteColor};
use super::buttons::Button;
use super::span::Span;

static CONTEXT_IDX: AtomicUsize = AtomicUsize::new(0);

pub fn handle_menu(
    world: &mut World,
    backend: &dyn GraphicsBackend,
    state: &InputState,
    scale: f32
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

    if entities.len() == 0 { 
        return false 
    };

    entities.sort_by_key(|a| (a.version, a.id));

    let cur_idx = CONTEXT_IDX.load(Relaxed);
    CONTEXT_IDX.store(cur_idx % entities.len(), Relaxed);

    let viewport_size = backend.viewport_size();
    let gap = scale * UI_GAP;
    let height = scale * UI_BUTTON_HEIGHT;
    let y = viewport_size.y - 2.0 * (height + gap);
    let width = match entities.len() {
        0 => return false,
        1 => viewport_size.x - 2.0 * gap,
        _ => (viewport_size.x - 3.0 * gap) / 2.0
    };

    if entities.len() > 1 {
        // draw `next` button
        let button = Button::new(
                2.0 * gap + width,
                y,
                width,
                height
            )
            .with_color(BUTTON_COLOR)
            .with_span(Span::new().with_text_borrowed("[MORE]").with_size((scale * UI_BUTTON_TEXT_SIZE as f32) as u32));
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
            e if world.get_component::<Item>(*e).is_some() =>
                ("PICK", Box::new(AddToInventory { entity: *e }) as Box<dyn Action>),
            _ => continue
        };

        draw_item_desc(world, *entity, backend, scale);

        let span = Span::new().with_text_borrowed(text).with_size((scale * UI_BUTTON_TEXT_SIZE as f32) as u32);
        let button = Button::new(
                gap,
                y,
                width,
                height
            )
            .with_color(BUTTON_COLOR)
            .with_span(span);
        button.draw(backend);
        if button.clicked(state) || state.action == ButtonState::Pressed {
            set_player_action(world, action);
            return true;
        }
    }

    false
}


fn get_item_desc(world: &World, entity: Entity) -> Option<Vec<String>> {
    if world.get_component::<Item>(entity).is_none()
        && world.get_component::<Interactive>(entity).is_none() { return None };

    let mut v = vec![world.get_component::<Name>(entity)?.0.clone().replace("_", " ")];
    v.extend(world.get_entity_components(entity).iter()
            .map(|c| c.as_str())
            .filter(|s| s.len() > 0)
        );
        // .collect::<Vec<_>>();
        // .join(" ");
    // return Some(format!("{}: {}", name, s));
    Some(v)
}

fn draw_item_desc(world: &World, entity: Entity, backend: &dyn GraphicsBackend, scale: f32) {
    if let Some(parts) = get_item_desc(world, entity) {
        let gap = scale * UI_GAP;
        let text_gap = scale * UI_TEXT_GAP;
        let font_size = (scale * UI_STATUS_TEXT_SIZE as f32) as u32;
        // let parts = s.split(" ");
        let space = backend.text_size("default", " ", font_size).x;

        let mut y = 2.0 * (font_size as f32 + text_gap);
        let mut x = gap;

        for (i, part) in parts.iter().enumerate() {
            let width = backend.text_size("default", &part, font_size).x;
            if x + width > backend.viewport_size().x - 2.0 * gap {
                x = gap;
                y += font_size as f32 + text_gap;
            }
            backend.draw_ui_text(
                "default",
                &format!("{} ", part),
                Vector2F::new(x, y),
                font_size,
                if i == 0 { SpriteColor(150, 128, 128, 255) } else { SpriteColor(98, 81, 81, 255) }
            );
            x += width + space;
        }
    }
}