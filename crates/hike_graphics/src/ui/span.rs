use std::borrow::Cow;
use rogalik::math::vectors::Vector2F;

use super::{GraphicsBackend, SpriteColor};

pub enum SpanItem<'a> {
    Sprite(&'a str, u32),
    Text(Cow<'a, str>),
    Spacer(f32)
}

pub struct Span<'a> {
    text_color: SpriteColor,
    sprite_color: SpriteColor,
    pub size: u32,
    items: Vec<SpanItem<'a>>,
}
impl<'a> Span<'a> {
    pub fn new() -> Self {
        Span {
            size: 32,
            sprite_color: SpriteColor(255, 255, 255, 255),
            text_color: SpriteColor(255, 255, 255, 255),
            items: Vec::new(),
        }
    }
    pub fn with_text_borrowed(mut self, text: &'a str) -> Self {
        self.items.push(
            SpanItem::Text(Cow::Borrowed(text))
        );
        self
    }
    pub fn with_text_owned(mut self, text: String) -> Self {
        self.items.push(
            SpanItem::Text(Cow::Owned(text))
        );
        self
    }
    pub fn with_sprite(mut self, atlas: &'a str, index: u32) -> Self {
        self.items.push(
            SpanItem::Sprite(atlas, index)
        );
        self
    }
    pub fn with_spacer(mut self, width: f32) -> Self {
        self.items.push(
            SpanItem::Spacer(width)
        );
        self
    }
    pub fn with_text_color(mut self, color: SpriteColor) -> Self {
        self.text_color = color;
        self
    }
    pub fn with_sprite_color(mut self, color: SpriteColor) -> Self {
        self.sprite_color = color;
        self
    }
    pub fn with_size(mut self, size: u32) -> Self {
        self.size = size;
        self
    }
    pub fn width(&self, backend: &dyn GraphicsBackend) -> f32 {
        let mut width = 0.;
        for item in self.items.iter() {
            match item {
                SpanItem::Text(text) => width += backend.text_size("default", text, self.size).x,
                &SpanItem::Sprite(_, _) => width += self.size as f32,
                SpanItem::Spacer(w) => width += w * self.size as f32
            }
        }
        width
    }

    pub fn draw(&self, origin: Vector2F, backend: &dyn GraphicsBackend) {
        let mut offset = 0.;
        let text_v_offset = -0.5 * (self.size as f32 - backend.text_size("default", "A", self.size).y);
        for item in self.items.iter() {
            match item {
                SpanItem::Text(text) => {
                    backend.draw_ui_text(
                        "default", 
                        text,
                        origin + Vector2F::new(offset, text_v_offset), 
                        self.size, 
                        self.text_color
                    );
                    offset += backend.text_size("default", text, self.size).x;
                },
                &SpanItem::Sprite(atlas, index) => {
                    backend.draw_ui_sprite(
                        atlas,
                        index,
                        origin + Vector2F::new(offset, -(self.size as f32)),
                        Vector2F::new(self.size as f32, self.size as f32),
                        self.sprite_color
                    );
                    offset += self.size as f32;
                },
                SpanItem::Spacer(w) => {
                    offset += self.size as f32 * w
                }
            }
        }
    }
}
