use macroquad::prelude::*;

use rogalik::math::vectors::Vector2F;

use super::errors::AssetError;

#[derive(Debug)]
pub struct SpriteAtlas {
    pub tex: Texture2D,
    columns: u32,
    rows: u32,
    grid_size: Vec2,
    grid_offset: Vec2,
    sprite_size: Vec2,
}
impl SpriteAtlas {
    pub async fn new(
        path: &str,
        columns: u32,
        rows: u32,
        spacing: Option<f32>
    ) -> Result<Self, AssetError> {
        // TODO defer loading?
        let tex = load_texture(path).await
            .map_err(|_| AssetError(format!("Could not load {}", path)))?;
        tex.set_filter(FilterMode::Nearest);

        let grid_size = Vec2::new(
            tex.width() / columns as f32,
            tex.height() / rows as f32
        );
        let sprite_size = match spacing {
            Some(d) => Vec2::new(grid_size.x - d, grid_size.y - d),
            None => grid_size
        };
        let grid_offset = (grid_size - sprite_size) / 2.;

        Ok(SpriteAtlas {
            tex, sprite_size, grid_size, grid_offset, rows, columns
        })
    }
    pub fn get_sprite(&self, idx: u32) -> Rect {
        let row = idx / self.columns;
        let col = idx % self.columns;
        Rect::new(
            col as f32 * self.grid_size.x + self.grid_offset.x,
            row as f32 * self.grid_size.x + self.grid_offset.x,
            self.sprite_size.x,
            self.sprite_size.y
        )
    }
}