use std::borrow::Cow;
use rogalik::{
    engine::{Color, GraphicsContext, Params2d},
    math::vectors::Vector2f
};


pub enum SpanItem<'a> {
    Sprite(&'a str, u32),
    Text(Cow<'a, str>),
    Spacer(f32)
}

pub struct Span<'a> {
    text_color: Color,
    sprite_color: Color,
    pub size: f32,
    items: Vec<SpanItem<'a>>,
}
impl<'a> Span<'a> {
    pub fn new() -> Self {
        Span {
            size: 1.,
            sprite_color: Color(255, 255, 255, 255),
            text_color: Color(255, 255, 255, 255),
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
    pub fn with_text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }
    pub fn with_sprite_color(mut self, color: Color) -> Self {
        self.sprite_color = color;
        self
    }
    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
    pub fn width(&self, context: &crate::Context_) -> f32 {
        let mut width = 0.;
        for item in self.items.iter() {
            match item {
                SpanItem::Text(text) => width += context.graphics.text_dimensions("default", text, self.size as f32).x,
                &SpanItem::Sprite(_, _) => width += self.size as f32,
                SpanItem::Spacer(w) => width += w * self.size as f32
            }
        }
        width
    }

    pub fn draw(&self, origin: Vector2f, context: &mut crate::Context_) {
        let mut offset = 0.;

        for item in self.items.iter() {
            match item {
                SpanItem::Text(text) => {
                    let _ = context.graphics.draw_text(
                        "default", 
                        text,
                        origin + Vector2f::new(offset, -(self.size as f32)), 
                        self.size as f32, 
                        Params2d { color: self.text_color, ..Default::default() }
                    );
                    offset += context.graphics.text_dimensions("default", text, self.size as f32).x;
                },
                &SpanItem::Sprite(atlas, index) => {
                    let _ = context.graphics.draw_atlas_sprite(
                        atlas,
                        index as usize,
                        origin + Vector2f::new(offset, -(self.size as f32)),
                        Vector2f::new(self.size as f32, self.size as f32),
                        Params2d { color: self.sprite_color, ..Default::default() }
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
