pub mod globals;
mod graphics;
pub mod game_ui;
pub mod title_ui;

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
        let ev_world = world.get_resource_mut::<EventBus<WorldEvent>>()
            .expect("Can't subscribe to world events!")
            .subscribe();
        GraphicsState {
            sprites: Vec::new(),
            ev_world,
            ev_game: events.subscribe(),
            animation_timer: ResourceId::default(),
            ui_state: game_ui::UiState::new(events)
        }
    }
    pub fn sort_sprites(&mut self) {
        self.sprites.sort_by(|a, b| a.z_index.cmp(&b.z_index));
    }
    pub fn restore(&mut self, world: &World) {
        graphics::renderers::restore_sprites(world, self);
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
