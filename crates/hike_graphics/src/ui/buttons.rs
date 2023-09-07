use rogalik::math::vectors::Vector2F;

use super::{GraphicsBackend, ButtonState, InputState, SpriteColor};
use super::span::Span;
use super::panels::Panel;
use super::super::globals::BUTTON_COLOR_SELECTED;

pub struct Button<'a> {
    origin: Vector2F,
    w: f32,
    h: f32,
    color: SpriteColor,
    span: Option<Span<'a>>
}
impl<'a> Button<'a> {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Button { 
            origin: Vector2F::new(x, y),
            w,
            h,
            color: SpriteColor(255, 255, 255, 255),
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
    pub fn with_color(mut self, color: SpriteColor) -> Self {
        self.color = color;
        self
    }
    pub fn draw(&self, backend: &dyn GraphicsBackend) {
        let panel = Panel::new(self.origin, self.w, self.h)
            .with_color(self.color)
            .with_border_color(BUTTON_COLOR_SELECTED);
        panel.draw(backend);
        if let Some(span) = &self.span {
            let span_offset = Vector2F::new(
                0.5 * (self.w - span.width(backend)),
                self.h - (self.h - span.size as f32) / 2.
            );
            span.draw(self.origin + span_offset, backend);
        }
    }
    pub fn clicked(&self, state: &InputState) -> bool {
        if let ButtonState::Released = state.mouse_button_left { 
            let v = state.mouse_screen_position;
            return v.x >= self.origin.x && v.y >= self.origin.y &&
            v.x <= self.origin.x + self.w && v.y <= self.origin.y + self.h
        }
        false
    }
}