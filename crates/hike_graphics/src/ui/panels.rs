use rogalik::math::vectors::Vector2F;

use super::{GraphicsBackend, ButtonState, InputState, SpriteColor};
use super::super::globals::PANEL_BORDER;

pub struct Panel {
    origin: Vector2F,
    w: f32,
    h: f32,
    color: SpriteColor,
    border_color: Option<SpriteColor>
}
impl Panel {
    pub fn new(origin: Vector2F, w: f32, h: f32) -> Self {
        Panel {
            origin,
            w,
            h,
            color: SpriteColor(255, 255, 255, 255),
            border_color: None
        }
    }
    pub fn with_color(mut self, color: SpriteColor) -> Self {
        self.color = color;
        self
    }
    pub fn with_border_color(mut self, color: SpriteColor) -> Self {
        self.border_color = Some(color);
        self
    }
    pub fn draw(&self, backend: &dyn GraphicsBackend) {
        let base_color = match self.border_color {
            Some(c) => c,
            None => self.color
        };
        backend.draw_ui_sprite(
            "ascii",
            219,
            self.origin,
            Vector2F::new(self.w, self.h),
            base_color
        );
        if let Some(_) = self.border_color {
            backend.draw_ui_sprite(
                "ascii",
                219,
                self.origin + Vector2F::new(PANEL_BORDER, PANEL_BORDER),
                Vector2F::new(
                    self.w - 2.0 * PANEL_BORDER,
                    self.h - 2.0 * PANEL_BORDER
                ),
                self.color
            ); 
        }
    }
}