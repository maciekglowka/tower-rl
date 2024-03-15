use rogalik::engine::{GraphicsContext, Params2d};
use rogalik::math::vectors::{Vector2i, Vector2f};

use crate::game_ui::{
    InputState, get_viewport_bounds,
    buttons::Button,
    span::Span
};
use crate::globals::{
    UI_GAP, UI_BUTTON_HEIGHT, UI_BUTTON_TEXT_SIZE
};

pub enum TitleMenuAction {
    None,
    Start,
    Resume
}

pub fn update_title_ui(
    context: &mut crate::Context_,
    input_state: &InputState,
    resume: bool
) -> TitleMenuAction {
    let bounds = get_viewport_bounds(context);

    let w = 0.75 * (bounds.1.x - bounds.0.x);
    let h = 1.5 * w;
    let c = bounds.0 + 0.5 * (bounds.1 - bounds.0);

    let _ = context.graphics.draw_atlas_sprite(
        "title",
        0,
        c - Vector2f::new(0.5 * w, 0.5 * h),
        0,
        Vector2f::new(w, h),
        Params2d::default()
    );

    let width = w;
    let origin = Vector2f::new(c.x - 0.5 * w, bounds.0.y + 3. * UI_BUTTON_HEIGHT);

    let start_button = Button::new(
            origin.x,
            origin.y,
            width,
            UI_BUTTON_HEIGHT
        )
        .with_sprite("ui", 0)
        .with_span(
            Span::new().with_text_borrowed("Start")
                .with_size(UI_BUTTON_TEXT_SIZE)
        );
    start_button.draw(context);
    if start_button.clicked(input_state) {
        return TitleMenuAction::Start;
    }

    if resume {
        let resume_button = Button::new(
                origin.x,
                origin.y - UI_GAP - UI_BUTTON_HEIGHT,
                width,
                UI_BUTTON_HEIGHT
            )
            .with_sprite("ui", 0)
            .with_span(
                Span::new().with_text_borrowed("Resume")
                    .with_size(UI_BUTTON_TEXT_SIZE)
            );
        resume_button.draw(context);
        if resume_button.clicked(input_state) {
            return TitleMenuAction::Resume;
        }
    }

    TitleMenuAction::None
}