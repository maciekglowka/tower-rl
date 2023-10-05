use rogalik::{
    engine::{Color, GraphicsContext},
    math::vectors::Vector2f
};

use super::{ButtonState, InputState};
use super::span::Span;
use super::panels::Panel;
use super::super::globals::BUTTON_COLOR_SELECTED;

pub struct Button<'a> {
    origin: Vector2f,
    w: f32,
    h: f32,
    color: Color,
    span: Option<Span<'a>>
}
impl<'a> Button<'a> {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Button { 
            origin: Vector2f::new(x, y),
            w,
            h,
            color: Color(255, 255, 255, 255),
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
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    pub fn draw(&self, context: &mut crate::Context_) {
        let panel = Panel::new(self.origin, self.w, self.h)
            .with_color(self.color)
            .with_border_color(BUTTON_COLOR_SELECTED);
        panel.draw(context);
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