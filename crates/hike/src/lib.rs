use rogalik::{
    engine::{Context, Game, GraphicsContext, EngineBuilder, ResourceId},
    math::vectors::{Vector2f, Vector2i},
    storage::World,
    wgpu::WgpuContext
};
use std::{
    collections::HashMap,
};

#[cfg(target_os = "android")]
use rogalik::engine::AndroidApp;

mod assets;
mod input;

pub type Context_ = Context<WgpuContext>;

pub struct GameState {
    camera_main: ResourceId,
    events: hike_game::GameEvents,
    graphics_ready: bool,
    graphics_state: hike_graphics::GraphicsState,
    touch_state: HashMap<u64, Vector2f>,
    world: World
}
impl Game<WgpuContext> for GameState {
    fn setup(&mut self, context: &mut Context<WgpuContext>) {
        assets::load_assets(self, context);
        context.graphics.set_clear_color(hike_graphics::globals::BACKGROUND_COLOR);

        let board_centre = hike_graphics::tile_to_world(
            rogalik::math::vectors::vector2::Vector2i::new(
                hike_game::globals::BOARD_SIZE as i32 / 2,
                hike_game::globals::BOARD_SIZE as i32 / 2
            )
        );

        self.camera_main = context.graphics.create_camera(
            48., board_centre + Vector2f::new(0., hike_graphics::globals::BOARD_V_OFFSET)
        );
        context.graphics.set_camera(self.camera_main);
    
        hike_game::init(&mut self.world, &mut self.events);
        self.touch_state = HashMap::new();
    }
    fn update(&mut self, context: &mut rogalik::engine::Context<WgpuContext>) {
        // println!("{}", 1. / context.time.get_delta());
        if self.graphics_ready {
            hike_game::game_update(&mut self.world, &mut self.events);
        }

        self.graphics_ready = hike_graphics::graphics_update(&self.world, &mut self.graphics_state, context);
        hike_graphics::ui::draw_world_ui(&self.world, context, &self.graphics_state);
        hike_graphics::ui::ui_update(
            &mut self.world,
            input::get_input_state(self.camera_main, &mut self.touch_state, context),
            context
        );
    }
    #[cfg(target_os = "android")]
    fn resize(&mut self, context: &mut rogalik::engine::Context<WgpuContext>) {
        let scale = 16. * (context.get_physical_size().x as u32 / 10 / 16) as f32;
        if let Some(camera) = context.graphics.get_camera_mut(self.camera_main) {
            camera.set_scale(scale);
        }
    }
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: AndroidApp) {
    let engine = EngineBuilder::new()
        .build_android(game_state(), app);
    engine.run();
}

fn main() {
    std::env::set_var("WINIT_UNIX_BACKEND", "x11");
    run();
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub fn run() {
    let engine = EngineBuilder::new()
        .with_title("Tower RL".to_string())
        .with_logical_size(600., 800.)
        .build(game_state());
    engine.run();
}

fn game_state() -> GameState {
    let mut world = World::new();
    let mut events = hike_game::GameEvents::new();
    let graphics_state = hike_graphics::GraphicsState::new(
        &mut world,
        &mut events
    );

    GameState {
        camera_main: ResourceId::default(),
        events,
        graphics_ready: false,
        graphics_state,
        touch_state: HashMap::new(),
        world
    }
}
