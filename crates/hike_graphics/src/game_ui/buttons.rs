use rogalik::{
    engine::{Color, GraphicsContext, Params2d},
    math::vectors::Vector2f
};

use super::{ButtonState, InputState};
use super::span::Span;
// use super::panels::Panel;
use super::super::globals::{UI_BUTTON_Z, UI_GAP, UI_BUTTON_HEIGHT, UI_BUTTON_TEXT_SIZE};

pub struct Button<'a> {
    origin: Vector2f,
    w: f32,
    h: f32,
    sprite: Option<(&'a str, usize)>,
    span: Option<Span<'a>>
}
impl<'a> Button<'a> {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Button { 
            origin: Vector2f::new(x, y),
            w,
            h,
            sprite: None,
            span: None
        }
    }
    pub fn with_span(
        mut self,
        span: Span<'a>
    ) -> Self {
        self.span = Some(span);
        self
    }
    pub fn with_sprite(mut self, atlas: &'a str, index: usize) -> Self {
        self.sprite = Some((atlas, index));
        self
    }
    pub fn draw(&self, context: &mut crate::Context_) {
        if let Some((atlas, index)) = self.sprite {
            context.graphics.draw_atlas_sprite(
                atlas,
                index,
                self.origin,
                UI_BUTTON_Z,
                Vector2f::new(self.w, self.h),
                Params2d { slice: Some((4, Vector2f::new(1., 1.))), ..Default::default() }
            );
        }
        if let Some(span) = &self.span {
            let span_offset = Vector2f::new(
                0.5 * (self.w - span.width(context)),
                self.h - (self.h - span.size as f32) / 2.
            );
            span.draw(self.origin + span_offset, context);
        }
    }
    pub fn clicked(&self, state: &InputState) -> bool {
        if let ButtonState::Released = state.mouse_button_left { 
            let v = state.mouse_world_position;
            return v.x >= self.origin.x && v.y >= self.origin.y &&
            v.x <= self.origin.x + self.w && v.y <= self.origin.y + self.h
        }
        false
    }
}

pub struct InputNumber<'a> {
    origin: Vector2f,
    w: f32,
    value: u32,
    single_width: f32,
    min: u32,
    max: u32,
    label: Option<&'a str>
}
impl<'a> InputNumber<'a> {
    pub fn new(x: f32, y: f32, w: f32, value: u32) -> Self {
        Self { 
            origin: Vector2f::new(x, y),
            w,
            value,
            single_width: w / 4.,
            min: 1,
            max: 10,
            label: None
        }
    }
    pub fn with_label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }
    pub fn with_min_max(mut self, min: u32, max: u32) -> Self {
        self.min = min;
        self.max = max;
        self
    }
    pub fn draw(&self, context: &mut crate::Context_) {
        // let y = self.origin.y - self.h + 2. * UI_GAP + UI_BUTTON_TEXT_SIZE;
        // let single_width = self.w / 4.;
        let value_width = self.w - 2. * (UI_GAP + self.single_width);

        let value = Button::new(
                self.origin.x + 0.5 * (self.w - value_width),
                self.origin.y,
                value_width,
                UI_BUTTON_HEIGHT
            )
            .with_sprite("ui", 1)
            .with_span(
                Span::new()
                    .with_size(UI_BUTTON_TEXT_SIZE)
                    .with_text_owned("|".repeat(self.value as usize))
            );
        value.draw(context);

        let down = self.get_down_button()
            .with_sprite("ui", 0)
            .with_span(
                Span::new()
                    .with_size(UI_BUTTON_TEXT_SIZE)
                    .with_text_borrowed("-")
            );
        down.draw(context);

        let up = self.get_up_button()
            .with_sprite("ui", 0)
            .with_span(
                Span::new()
                    .with_size(UI_BUTTON_TEXT_SIZE)
                    .with_text_borrowed("+")
            );
        up.draw(context);

        if let Some(label) = self.label {
            let span = Span::new()
                .with_text_borrowed(label)
                .with_size(UI_BUTTON_TEXT_SIZE);
            span.draw(
                Vector2f::new(
                    self.origin.x,
                    self.origin.y + 3. * UI_GAP + UI_BUTTON_HEIGHT
                ),
                context
            );

        }
    }
    fn get_down_button(&self) -> Button {
        Button::new(
            self.origin.x,
            self.origin.y,
            self.single_width,
            UI_BUTTON_HEIGHT
        )
    }
    fn get_up_button(&self) -> Button {
        Button::new(
            self.origin.x + self.w - self.single_width,
            self.origin.y,
            self.single_width,
            UI_BUTTON_HEIGHT
        )
    }
    pub fn new_value(&self, state: &InputState) -> u32 {
        if self.get_up_button().clicked(state) {
            return (self.value + 1).min(self.max);
        }
        if self.get_down_button().clicked(state) {
            return (self.value - 1).max(self.min);
        }
        self.value
    }
}
