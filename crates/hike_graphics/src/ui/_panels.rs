use rogalik::{
    engine::{Color, GraphicsContext, Params2d},
    math::vectors::Vector2f
};

use super::super::globals::PANEL_BORDER;

pub struct Panel {
    origin: Vector2f,
    w: f32,
    h: f32,
    color: Color,
    border_color: Option<Color>
}
impl Panel {
    pub fn new(origin: Vector2f, w: f32, h: f32) -> Self {
        Panel {
            origin,
            w,
            h,
            color: Color(255, 255, 255, 255),
            border_color: None
        }
    }
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    pub fn with_border_color(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }
    pub fn draw(&self, context: &mut crate::Context_) {
        let base_color = match self.border_color {
            Some(c) => c,
            None => self.color
        };
        context.graphics.draw_atlas_sprite(
            "ascii",
            219,
            self.origin,
            Vector2f::new(self.w, self.h),
            Params2d { color: base_color, ..Default::default() }
        );
        if let Some(_) = self.border_color {
            context.graphics.draw_atlas_sprite(
                "ascii",
                219,
                self.origin + Vector2f::new(PANEL_BORDER, PANEL_BORDER),
                Vector2f::new(
                    self.w - 2.0 * PANEL_BORDER,
                    self.h - 2.0 * PANEL_BORDER
                ),
                Params2d { color: self.color, ..Default::default() }
            ); 
        }
    }
}