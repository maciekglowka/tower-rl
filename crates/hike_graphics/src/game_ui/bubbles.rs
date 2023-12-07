use rand::prelude::*;
use rogalik::engine::{Color, GraphicsContext, Params2d};
use rogalik::math::vectors::Vector2f;
use rogalik::storage::{Entity, World};

use hike_game::{
    GameEvent,
    components::Position,
};

use super::super::{Context_, tile_to_world};
use super::UiState;
use crate::globals::{
    UI_BUBBLE_Z, UI_BUBBLE_MAX_AGE, UI_BUBBLE_SPEED, 
    UI_OVERLAY_FONT_SIZE, HEALTH_COLOR, POISON_COLOR, IMMUNITY_COLOR
};

pub struct Bubble {
    pub v: Vector2f,
    pub text: String,
    pub color: Color,
    pub age: f32
}

pub fn handle_bubbles(
    world: &World,
    state: &mut UiState,
    context: &mut Context_,
) {
    update_bubbles(state, context.time.get_delta());
    draw_bubbles(state, context);
}

pub fn handle_game_event(
    ev: &GameEvent,
    world: &World,
    state: &mut UiState
) {
    let mut bubble_value = None;
    match ev {
        GameEvent::Health(entity, value) => {
            bubble_value = Some((
                entity,
                format!("{}", value),
                HEALTH_COLOR
            ));
        },
        _ => {}
    }
    if let Some(value) = bubble_value {
        if let Some(position) = world.get_component::<Position>(*value.0) {
            let mut rng = thread_rng();
            let offset = Vector2f::new(
                rng.gen_range(0.0..0.5),
                rng.gen_range(0.5..1.),
            );
            let v = tile_to_world(position.0) + offset;
            let bubble = Bubble {
                v,
                text: value.1,
                color: value.2,
                age: 0.
            };
            state.bubbles.push(bubble);
        }
    }
}

fn update_bubbles(state: &mut UiState, delta: f32) {
    for bubble in state.bubbles.iter_mut() {
        bubble.v.y += UI_BUBBLE_SPEED * delta;
        bubble.age += delta;
    }
    state.bubbles.retain(|b| b.age < UI_BUBBLE_MAX_AGE);
}

fn draw_bubbles(
    state: &UiState,
    context: &mut Context_
) {
    for bubble in state.bubbles.iter() {
        let _ = context.graphics.draw_text(
            "default",
            &bubble.text,
            bubble.v,
            UI_BUBBLE_Z,
            UI_OVERLAY_FONT_SIZE,
            Params2d { color: bubble.color, ..Default::default() }
        );
    }
}
