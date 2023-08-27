use std::borrow::Cow;
use rogalik::math::vectors::Vector2F;

use super::{GraphicsBackend, SpriteColor};

pub enum SpanItem<'a> {
    Sprite(&'a str, u32),
    Text(Cow<'a, str>)
}

pub struct Span<'a> {
    text_color: SpriteColor,
    sprite_color: SpriteColor,
    pub size: u32,
    items: Vec<SpanItem<'a>>,
    spacing: f32
}
impl<'a> Span<'a> {
    pub fn new() -> Self {
        Span {
            size: 32,
            sprite_color: SpriteColor(255, 255, 255, 255),
            text_color: SpriteColor(255, 255, 255, 255),
            items: Vec::new(),
            spacing: 8.
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
    pub fn with_text_color(mut self, color: SpriteColor) -> Self {
        self.text_color = color;
        self
    }
    pub fn with_sprite_color(mut self, color: SpriteColor) -> Self {
        self.sprite_color = color;
        self
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
                    offset += backend.text_size("default", text, self.size).x + self.spacing;
                },
                &SpanItem::Sprite(atlas, index) => {
                    backend.draw_ui_sprite(
                        atlas,
                        index,
                        origin + Vector2F::new(offset, -(self.size as f32)),
                        Vector2F::new(self.size as f32, self.size as f32),
                        self.sprite_color
                    );
                    offset += self.size as f32 + self.spacing;
                }
            }
        }
    }
}
