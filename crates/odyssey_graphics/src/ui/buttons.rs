use rogalik::math::vectors::Vector2F;

use super::{GraphicsBackend, ButtonState, InputState, SpriteColor};

pub struct Button<'a> {
    origin: Vector2F,
    w: f32,
    h: f32,
    color: SpriteColor,
    text: &'a str,
    text_size: u32,
    text_color: SpriteColor
    // command: Option<Box<dyn Command>>
}
impl<'a> Button<'a> {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Button { 
            origin: Vector2F::new(x, y),
            w,
            h,
            color: SpriteColor(255, 255, 255, 255),
            text: "",
            text_size: 0,
            text_color: SpriteColor(255, 255, 255, 255)
        }
    }
    pub fn with_text(
        &mut self,
        text: &'a str,
        color: SpriteColor,
        size: u32
    ) -> &mut Self {
        self.text = text;
        self.text_color = color;
        self.text_size = size;
        self
    }
    pub fn with_color(&mut self, color: SpriteColor) -> &mut Self {
        self.color = color;
        self
    }
    pub fn draw(&mut self, backend: &dyn GraphicsBackend) -> &mut Self{
        backend.draw_ui_sprite(
            "ascii",
            219,
            self.origin,
            Vector2F::new(self.w, self.h),
            self.color
        );
        backend.draw_ui_text(
            "default",
            self.text,
            self.origin + Vector2F::new(4., self.h - 4.),
            self.text_size,
            self.text_color
        );
        self
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