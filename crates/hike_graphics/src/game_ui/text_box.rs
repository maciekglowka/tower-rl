use std::borrow::Cow;
use rogalik::{
    engine::{Color, GraphicsContext, Params2d},
    math::vectors::Vector2f
};

use crate::globals::UI_TEXT_Z;

pub struct TextBox<'a> {
    text: Cow<'a, str>,
    size: f32,
    color: Color
}
impl<'a> TextBox<'a> {
    pub fn new() -> Self {
        TextBox {
            size: 1.,
            color: Color(255, 255, 255, 255),
            text: Cow::Borrowed(""),
        }
    }
    pub fn with_text_borrowed(mut self, text: &'a str) -> Self {
        self.text = Cow::Borrowed(text);
        self
    }
    pub fn with_text_owned(mut self, text: String) -> Self {
        self.text = Cow::Owned(text);
        self
    }
    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    pub fn draw(&self, origin: Vector2f, width: f32, context: &mut crate::Context_) {
        let paragraphs = self.text.split('\n');
        let mut v_offset = 0.;
        let line_height = 1.1 * self.size;
        let space = context.graphics.text_dimensions("default", " ", self.size).x;

        for paragraph in paragraphs {
            let mut line_width = 0.;
            let words = paragraph.split(" ");
            for word in words {
                let w = context.graphics.text_dimensions("default", word, self.size as f32).x;
                if line_width + w > width {
                    line_width = 0.;
                    v_offset += line_height;
                }

                let _ = context.graphics.draw_text(
                    "default", 
                    word,
                    origin + Vector2f::new(line_width, -(self.size as f32) - v_offset), 
                    UI_TEXT_Z,
                    self.size as f32, 
                    Params2d { color: self.color, ..Default::default() }
                );
                line_width += w + space;
            }
            v_offset += line_height;
        }
    }
}