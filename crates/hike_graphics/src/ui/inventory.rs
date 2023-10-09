use rogalik::{
    engine::{Color, GraphicsContext, Params2d},
    math::vectors::Vector2f,
    storage::{Entity, World}
};

use hike_game::actions::UseCollectable;
use hike_game::components::{Name, Player};
use hike_game::globals::{MAX_COLLECTABLES, MAX_WEAPONS};
use hike_game::set_player_action;

use super::{InputState, ButtonState, get_viewport_bounds};
use super::buttons::Button;
use super::super::globals::{
    UI_BUTTON_HEIGHT, UI_GAP, UI_BUTTON_TEXT_SIZE, BUTTON_COLOR,
    BUTTON_COLOR_SELECTED, UI_BOTTOM_PANEL_HEIGHT
};
use super::utils::get_item_span;

pub fn handle_inventory(
    world: &World,
    context: &mut crate::Context_,
    state: &InputState
) {
    let bounds = get_viewport_bounds(context);

    draw_inventory_panel(bounds.0, bounds.1.x - bounds.0.x, context);
    
    context.graphics.draw_text(
        "default",
        "Weapons",
        bounds.0 + Vector2f::new(UI_GAP, 1.5 * UI_GAP + UI_BUTTON_HEIGHT),
        UI_BUTTON_TEXT_SIZE,
        Params2d { color: BUTTON_COLOR, ..Default::default() }
    );
    context.graphics.draw_text(
        "default",
        "Inventory",
        bounds.0 + Vector2f::new(UI_GAP, 2.5 * UI_GAP + 2. * UI_BUTTON_HEIGHT + UI_BUTTON_TEXT_SIZE),
        UI_BUTTON_TEXT_SIZE,
        Params2d { color: BUTTON_COLOR, ..Default::default() }
    );
    
    let mut click = (None, None);
    {
        let query = world.query::<Player>().build();
        let Some(player) = query.single::<Player>() else { return };
        click.0 = handle_inventory_buttons(
            bounds.0 + Vector2f::new(
                0.,
                UI_GAP
            ),
            bounds.1.x - bounds.0.x,
            &player.weapons.to_vec(),
            Some(player.active_weapon),
            world,
            context,
            state
        );
        click.1 = handle_inventory_buttons(
            bounds.0 + Vector2f::new(
                0.,
                2. * UI_GAP + UI_BUTTON_HEIGHT + UI_BUTTON_TEXT_SIZE
            ),
            bounds.1.x - bounds.0.x,
            &(0..MAX_COLLECTABLES).map(|i| player.collectables.get(i).map(|a| *a)).collect(),
            None,
            world,
            context,
            state
        );
    }

    if state.digits[1] == ButtonState::Pressed { click.0 = Some(0) };
    if state.digits[2] == ButtonState::Pressed { click.0 = Some(1) };
    if state.digits[3] == ButtonState::Pressed { click.0 = Some(2) };
    if state.digits[4] == ButtonState::Pressed { click.0 = Some(3) };

    if let Some(click) = click.0 {
        click_weapon(click, world);
    }
    if let Some(click) = click.1 {
        click_item(click, world);
    }
}

fn handle_inventory_buttons(
    v: Vector2f,
    width: f32,    
    entities: &Vec<Option<Entity>>,
    active: Option<usize>,
    world: &World,
    context: &mut crate::Context_,
    state: &InputState,
) -> Option<usize> {
    // return item index if clicked
    let mut clicked = None;
    let single_width = (width - UI_GAP) / (entities.len() as f32) - UI_GAP;

    for (i, entity) in entities.iter().enumerate() {
        let idx = if Some(i) == active { 1 } else { 0 };
        let offset = UI_GAP + i as f32 * (UI_GAP + single_width);

        let mut button = Button::new(
                v.x + offset,
                v.y,
                single_width,
                UI_BUTTON_HEIGHT
            )
            .with_sprite("ui", idx);

        if let Some(entity) = entity {
            let mut span = get_item_span(*entity, world);
            span = span.with_size(UI_BUTTON_TEXT_SIZE);
    
            button = button.with_span(span);
        }

        button.draw(context);
        if button.clicked(state) {
            clicked = Some(i)
        }
    }
    clicked
}

fn click_weapon(index: usize, world: &World) {
    world.query::<Player>().build()
        .single_mut::<Player>()
        .unwrap()
        .active_weapon = index;
}

fn click_item(index: usize, world: &World) {
    let entity = if let Some(player) = world.query::<Player>().build().single::<Player>() {
        player.collectables.get(index).map(|&e| e)
    } else { return };
    if let Some(entity) = entity {
        set_player_action(world, Box::new(UseCollectable { entity }));
    }
}

fn draw_inventory_panel(
    v: Vector2f,
    width: f32,    
    context: &mut crate::Context_,
) {
    context.graphics.draw_atlas_sprite(
        "ui",
        0,
        v,
        Vector2f::new(width, UI_BOTTOM_PANEL_HEIGHT),
        Params2d { slice: Some((4, Vector2f::new(1., 1.))), ..Default::default() }
    );
}
