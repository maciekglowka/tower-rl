pub mod globals;
mod graphics;
pub mod game_ui;

use rogalik::engine::{Color, Context, ResourceId};
use rogalik::events::{EventBus, SubscriberHandle};
use rogalik::storage::{World, WorldEvent};
use rogalik::math::vectors::{Vector2f, Vector2i};
use rogalik::wgpu::WgpuContext;

use hike_game::GameEvent;

use globals::{TILE_SIZE, PERSP_RATIO};

type Context_ = Context<WgpuContext>;

pub use graphics::{
    graphics_update,
    utils::move_towards
};

#[derive(Clone, Copy)]
pub enum UiEvent {
    Restart
}


pub struct GraphicsState {
    sprites: Vec<graphics::renderers::SpriteRenderer>,
    ev_world: SubscriberHandle<WorldEvent>,
    ev_game: SubscriberHandle<GameEvent>,
    pub animation_timer: ResourceId,
    pub ui_state: game_ui::UiState,
}
impl GraphicsState {
    pub fn new(world: &mut World, events: &mut EventBus<GameEvent>) -> Self {
        GraphicsState {
            sprites: Vec::new(),
            ev_world: world.events.subscribe(),
            ev_game: events.subscribe(),
            animation_timer: ResourceId::default(),
            ui_state: game_ui::UiState::default()
        }
    }
    pub fn sort_sprites(&mut self) {
        self.sprites.sort_by(|a, b| a.z_index.cmp(&b.z_index));
    }
}

// #[derive(Clone, Copy)]
// pub struct SpriteColor(pub u8, pub u8, pub u8, pub u8);

pub fn world_to_tile(
    v: Vector2f,
) -> Vector2i {
    Vector2i::new (
        (v.x / TILE_SIZE).floor() as i32,
        (v.y / TILE_SIZE / PERSP_RATIO).floor() as i32,
    )
}

pub fn tile_to_world(
    v: Vector2i
) -> Vector2f {
    Vector2f::new(
        v.x as f32 * TILE_SIZE,
        v.y as f32 * TILE_SIZE * PERSP_RATIO
    )
}
