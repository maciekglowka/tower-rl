use rogalik::{
    engine::{Color, GraphicsContext, Params2d},
    math::vectors::Vector2f,
    storage::{Entity, World}
};
use core::sync::atomic::{AtomicUsize, AtomicBool, Ordering::Relaxed};

use hike_game::{
    actions::{Action, Interact, WieldWeapon, PickCollectable},
    components::{Interactive, Collectable, Item, Name, Weapon},
    get_player_position,
    get_player_entity,
    set_player_action,
    get_entities_at_position
};

use super::super::globals::{
    UI_BUTTON_HEIGHT, UI_GAP, UI_TEXT_GAP, UI_BUTTON_TEXT_SIZE, UI_BOTTOM_PANEL_HEIGHT, UI_STATUS_TEXT_SIZE
};
use super::{InputState, ButtonState, get_viewport_bounds};
use super::buttons::Button;
use super::span::Span;
use super::utils::{get_item_name, get_item_span, get_interactive_span};

static CONTEXT_IDX: AtomicUsize = AtomicUsize::new(0);
pub static CONTEXT_VISIBLE: AtomicBool = AtomicBool::new(false);

pub fn handle_menu(
    world: &mut World,
    context: &mut crate::Context_,
    state: &InputState,
) -> bool {
    // true if clicked
    let Some(position) = get_player_position(world) else { return false };

    let mut entities = get_entities_at_position(world, position)
        .iter()
        .filter(|&&e| world.get_component::<Interactive>(e).is_some() ||
            world.get_component::<Weapon>(e).is_some() ||
            world.get_component::<Collectable>(e).is_some()
        )
        .map(|&e| e)
        .collect::<Vec<_>>();

    if entities.len() == 0 {
        CONTEXT_VISIBLE.store(false, Relaxed);
        return false;
    };
    
    entities.sort_by_key(|a| (a.version, a.id));
    
    let cur_idx = CONTEXT_IDX.load(Relaxed);
    CONTEXT_IDX.store(cur_idx % entities.len(), Relaxed);
    CONTEXT_VISIBLE.store(true, Relaxed);
    
    let bounds = get_viewport_bounds(context);
    let y = bounds.0.y + UI_BOTTOM_PANEL_HEIGHT + UI_GAP;
    let max_x = bounds.1.x - 2. * (UI_BUTTON_HEIGHT + UI_GAP);
    let width = match entities.len() {
        0 => return false,
        1 => max_x - bounds.0.x - 2.0 * UI_GAP,
        _ => (max_x - bounds.0.x - 3.0 * UI_GAP) / 2.0
    };

    if entities.len() > 1 {
        // draw `next` button
        let button = Button::new(
                bounds.0.x + 2.0 * UI_GAP + width,
                y,
                width,
                UI_BUTTON_HEIGHT
            )
            .with_sprite("ui", 0)
            .with_span(Span::new().with_text_borrowed("[MORE]").with_size(UI_BUTTON_TEXT_SIZE));
        button.draw(context);
        if button.clicked(state) || state.action_right == ButtonState::Pressed {
            CONTEXT_IDX.store(cur_idx + 1, Relaxed);
            return true;
        }
    };

    
    for (i, entity) in entities.iter().enumerate() {
        if i != cur_idx { continue };

        let (text, action) = match entity {
            e if world.get_component::<Interactive>(*e).is_some() =>
                ("USE", Box::new(Interact { entity: *e}) as Box<dyn Action>),
            e if world.get_component::<Collectable>(*e).is_some() =>
                ("PICK", Box::new(PickCollectable { entity: *e }) as Box<dyn Action>),
            e if world.get_component::<Weapon>(*e).is_some() =>
                ("WIELD", Box::new(WieldWeapon { entity: *e }) as Box<dyn Action>),
            _ => continue
        };

        draw_item_desc(
            world,
            *entity,
            context,
            Vector2f::new(bounds.0.x + UI_GAP, bounds.1.y - UI_GAP - 2. * (UI_STATUS_TEXT_SIZE + UI_TEXT_GAP))
        );

        let span = Span::new().with_text_borrowed(text).with_size(UI_BUTTON_TEXT_SIZE);
        let button = Button::new(
                bounds.0.x + UI_GAP,
                y,
                width,
                UI_BUTTON_HEIGHT
            )
            .with_sprite("ui", 0)
            .with_span(span);
        button.draw(context);
        if button.clicked(state) || state.action_left == ButtonState::Pressed {
            set_player_action(world, action);
            return true;
        }
    }

    false
}

fn get_desc_spans(world: &World, entity: Entity) -> Option<Vec<Span>> {
    let name = get_item_name(entity, world)?;
    let name_span = Span::new()
        .with_text_owned(name)
        .with_text_color(Color(150, 128, 128, 255))
        .with_size(UI_STATUS_TEXT_SIZE);
    let mut res = vec![name_span];
    
    if world.get_component::<Item>(entity).is_some() {
        res.push(get_item_span(entity, world).with_size(UI_STATUS_TEXT_SIZE));
    }    
    if world.get_component::<Interactive>(entity).is_some() {
        res.push(get_interactive_span(entity, world).with_size(UI_STATUS_TEXT_SIZE));
    }    
    Some(res)
}

fn draw_item_desc(world: &World, entity: Entity, context: &mut crate::Context_, v: Vector2f) {
    if let Some(spans) = get_desc_spans(world, entity) {
        let space = 0.5 * context.graphics.text_dimensions("default", " ", UI_STATUS_TEXT_SIZE).x;
        let mut offset = 0.;
        for span in spans {
            span.draw(
                v + Vector2f::new(offset, UI_STATUS_TEXT_SIZE + UI_GAP), context
            );
            offset += span.width(context) + space;
        }
    }
}
