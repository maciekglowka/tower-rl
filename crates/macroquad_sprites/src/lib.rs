use macroquad::prelude::*;
use rogalik::math::vectors::Vector2F;
use std::collections::HashMap;

use odyssey_graphics::{GraphicsBackend, SpriteColor};

mod assets;
mod errors;

pub struct MacroquadBackend {
    pub atlases: HashMap<String, assets::SpriteAtlas>,
    pub fonts: HashMap<String, Font>,
    world_bounds: (Vec2, Vec2),
    window_size: Vec2
}
impl MacroquadBackend {
    pub fn new() -> Self {
        MacroquadBackend { 
            atlases: HashMap::new(),
            world_bounds: (Vec2::ZERO, Vec2::ZERO),
            fonts: HashMap::new(),
            window_size: Vec2::ZERO
        }
    }
    pub async fn load_font(
        &mut self,
        name: &str,
        path: &str
    ) -> Result<(), errors::AssetError> {
        let font = load_ttf_font(path).await
            .map_err(|_| errors::AssetError(format!("Could not load {}", path)))?;
        self.fonts.insert(name.into(), font);
        Ok(())
    }
    pub async fn load_atlas(
        &mut self,
        name: &str,
        path: &str,
        columns: u32,
        rows: u32,
        spacing: Option<f32>
    ) -> Result<(), errors::AssetError> {
        let atlas = assets::SpriteAtlas::new(
            path,
            columns,
            rows,
            spacing
        ).await?;
        self.atlases.insert(name.into(), atlas);
        Ok(())
    }
    pub fn set_bounds(&mut self, camera: &Camera2D) {
        let bounds_min = camera.screen_to_world(Vec2::new(0., 0.));
        let bounds_max = camera.screen_to_world(Vec2::new(screen_width(), screen_height()));
        self.world_bounds = (bounds_min, bounds_max);
        self.window_size = Vec2::new(screen_width(), screen_height());
    }
    fn draw_sprite(
        &self,
        atlas_name: &str,
        index: u32,
        position: Vector2F,
        size: Vector2F,
        color: SpriteColor
    ) {
        let Some(atlas) = self.atlases.get(atlas_name) else { return };
        let sprite = atlas.get_sprite(index);
        let params = DrawTextureParams {
            dest_size: Some(Vec2::new(size.x, size.y)),
            source: Some(sprite),
            ..Default::default()
        };
        let macroquad_color = macroquad_color_from_sprite(color);
        draw_texture_ex(&atlas.tex, position.x, position.y, macroquad_color, params);
    }
}
impl GraphicsBackend for MacroquadBackend {
    fn viewport_size(&self) -> Vector2F {
        Vector2F::new(self.window_size.x, self.window_size.y)
    }
    fn draw_world_sprite(
        &self,
        atlas_name: &str,
        index: u32,
        position: Vector2F,
        size: Vector2F,
        color: SpriteColor
    ) {
        // draw only visible sprites
        if position.x > self.world_bounds.1.x || position.y > self.world_bounds.1.y { return };
        if position.x + size.x < self.world_bounds.0.x || position.y + size.y < self.world_bounds.0.y { return };
        self.draw_sprite(atlas_name, index, position, size, color);
    }
    fn draw_ui_sprite(
        &self,
        atlas_name: &str,
        index: u32,
        position: Vector2F,
        size: Vector2F,
        color: SpriteColor
    ) {
        self.draw_sprite(atlas_name, index, position, size, color);
    }
    fn draw_ui_text(
        &self,
        font_name: &str,
        text: &str,
        position: Vector2F,
        font_size: u32,
        color: SpriteColor
    ) {
        let macroquad_color = macroquad_color_from_sprite(color);
        let Some(font) = self.fonts.get(font_name) else { return };
        let params = TextParams {
            font_size: font_size as u16,
            color: macroquad_color,
            font: Some(font),
            ..Default::default()
        };
        draw_text_ex(text, position.x, position.y, params);
    }
}

fn macroquad_color_from_sprite(color: SpriteColor) -> Color {
    macroquad::color::Color::from_rgba(
        color.0,
        color.1,
        color.2,
        color.3
    )
}