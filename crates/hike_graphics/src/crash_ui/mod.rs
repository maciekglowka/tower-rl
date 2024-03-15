use rogalik::engine::{GraphicsContext, Params2d};
use rogalik::math::vectors::{Vector2i, Vector2f};

use crate::game_ui::{
    InputState, get_viewport_bounds, ButtonState,
    span::Span
};
use crate::globals::{
    UI_GAP, UI_BUTTON_TEXT_SIZE
};

pub fn update_crash_ui(
    context: &mut crate::Context_,
    input_state: &InputState
) -> bool {
    let bounds = get_viewport_bounds(context);

    let origin = Vector2f::new(
        bounds.0.x + UI_GAP,
        bounds.1.y - 2. * UI_BUTTON_TEXT_SIZE
    );

    let oops = Span::new()
        .with_text_borrowed("Ooops...")
        .with_size(UI_BUTTON_TEXT_SIZE);
    oops.draw(origin, context);

    let msg = Span::new()
        .with_text_borrowed("Something went wrong ;(")
        .with_size(UI_BUTTON_TEXT_SIZE);
    msg.draw(origin - Vector2f::new(0., UI_GAP + UI_BUTTON_TEXT_SIZE), context);

    input_state.mouse_button_left == ButtonState::Released
}