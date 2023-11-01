use rogalik::{
    engine::{Color, GraphicsContext, Params2d},
    math::vectors::Vector2f
};

use super::{ButtonState, InputState};
use super::span::Span;
// use super::panels::Panel;
use super::super::globals::UI_BUTTON_Z;

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