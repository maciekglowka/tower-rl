use rogalik::{
    engine::{Color, GraphicsContext, Params2d},
    math::vectors::Vector2f,
    storage::{Entity, World}
};
use core::sync::atomic::{AtomicUsize, Ordering::Relaxed};

use hike_game::{
    actions::{Action, Interact, WieldWeapon, Pause},
    components::{Interactive, Collectable, Item, Name, Weapon},
    get_player_position,
    get_player_entity,
    set_player_action,
    get_entities_at_position
};

use super::super::globals::{
    UI_BUTTON_HEIGHT, UI_GAP, UI_TEXT_GAP, UI_BUTTON_TEXT_SIZE, BUTTON_COLOR, UI_STATUS_TEXT_SIZE
};
use super::{InputState, ButtonState, get_viewport_bounds};
use super::buttons::Button;
use super::span::Span;

static CONTEXT_IDX: AtomicUsize = AtomicUsize::new(0);

pub fn handle_menu(
    world: &mut World,
    context: &mut crate::Context_,
    state: &InputState,
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

    let bounds = get_viewport_bounds(context);
    let y = bounds.0.y + UI_BUTTON_HEIGHT + 2. * UI_GAP;
    let width = match entities.len() {
        0 => return false,
        1 => bounds.1.x - bounds.0.x - 2.0 * UI_GAP,
        _ => (bounds.1.x - bounds.0.x - 3.0 * UI_GAP) / 2.0
    };

    if entities.len() > 1 {
        // draw `next` button
        let button = Button::new(
                bounds.0.x + 2.0 * UI_GAP + width,
                y,
                width,
                UI_BUTTON_HEIGHT
            )
            .with_color(BUTTON_COLOR)
            .with_span(Span::new().with_text_borrowed("[MORE]").with_size(UI_BUTTON_TEXT_SIZE));
        button.draw(context);
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
            // e if world.get_component::<Collectable>(*e).is_some() =>
            //     ("PICK", Box::new(Consume { entity: *e, consumer: player }) as Box<dyn Action>),
            e if world.get_component::<Item>(*e).is_some() =>
                ("WIELD", Box::new(WieldWeapon { entity: *e }) as Box<dyn Action>),
            _ => continue
        };

        draw_item_desc(
            world,
            *entity,
            context,
            Vector2f::new(bounds.0.x + UI_GAP, bounds.1.y - UI_GAP - UI_TEXT_GAP - 2. * UI_STATUS_TEXT_SIZE),
            bounds.1.x - bounds.0.x
        );

        let span = Span::new().with_text_borrowed(text).with_size(UI_BUTTON_TEXT_SIZE);
        let button = Button::new(
                bounds.0.x + UI_GAP,
                y,
                width,
                UI_BUTTON_HEIGHT
            )
            .with_color(BUTTON_COLOR)
            .with_span(span);
        button.draw(context);
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
    Some(v)
}

fn draw_item_desc(world: &World, entity: Entity, context: &mut crate::Context_, v: Vector2f, vw: f32) {
    if let Some(parts) = get_item_desc(world, entity) {
        let space = context.graphics.text_dimensions("default", " ", UI_STATUS_TEXT_SIZE).x;

        let mut y = v.y;
        let mut x = v.x;

        for (i, part) in parts.iter().enumerate() {
            let width = context.graphics.text_dimensions("default", &part, UI_STATUS_TEXT_SIZE).x;
            if x + width > vw - 2.0 * UI_GAP {
                x = v.x;
                y -= UI_STATUS_TEXT_SIZE + UI_TEXT_GAP;
            }
            let color = if i == 0 { Color(150, 128, 128, 255) } else { Color(98, 81, 81, 255) };
            context.graphics.draw_text(
                "default",
                &format!("{} ", part),
                Vector2f::new(x, y),
                UI_STATUS_TEXT_SIZE,
                Params2d { color, ..Default::default() }
            );
            x += width + space;
        }
    }
}