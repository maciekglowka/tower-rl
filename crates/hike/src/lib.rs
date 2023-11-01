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

#[derive(Default)]
enum GamePhase {
    #[default]
    GameStart,
    Game,
    GameEnd,
}

pub struct GameState {
    phase: GamePhase,
    camera_main: ResourceId,
    events: hike_game::GameEvents,
    graphics_ready: bool,
    graphics_state: hike_graphics::GraphicsState,
    input_state: hike_graphics::game_ui::InputState,
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
    
        self.touch_state = HashMap::new();
        self.graphics_state.animation_timer = context.time.add_timer(hike_graphics::globals::ANIMATION_TICK);
    }
    fn update(&mut self, context: &mut rogalik::engine::Context<WgpuContext>) {
        // println!("{}", 1. / context.time.get_delta());
        match self.phase {
            GamePhase::Game => game_update(self, context),
            GamePhase::GameStart => {
                hike_game::init(&mut self.world, &mut self.events);
                self.phase = GamePhase::Game;
            },
            GamePhase::GameEnd => {
                (self.world, self.events, self.graphics_state) = get_initial_elements();
                self.phase = GamePhase::GameStart;
            }
        }
        // std::thread::sleep(std::time::Duration::from_millis(5));
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
        .with_logical_size(600., 760.)
        .build(game_state());
    engine.run();
}

fn game_state() -> GameState {
    // let mut world = World::new();
    // let mut events = hike_game::GameEvents::new();
    // let graphics_state = hike_graphics::GraphicsState::new(
    //     &mut world,
    //     &mut events
    // );
    let (world, events, graphics_state) = get_initial_elements();

    GameState {
        phase: GamePhase::default(),
        camera_main: ResourceId::default(),
        events,
        graphics_ready: false,
        graphics_state,
        input_state: hike_graphics::game_ui::InputState::default(),
        touch_state: HashMap::new(),
        world
    }
}

fn get_initial_elements() -> (World, hike_game::GameEvents, hike_graphics::GraphicsState) {
    let mut world = World::new();
    let mut events = hike_game::GameEvents::new();
    let graphics_state = hike_graphics::GraphicsState::new(
        &mut world,
        &mut events
    );
    (world, events, graphics_state)
}

fn game_update(state: &mut GameState, context: &mut Context_) {
    if state.graphics_ready {
        let _ = hike_game::game_update(&mut state.world, &mut state.events);
    }

    state.graphics_ready = hike_graphics::graphics_update(&state.world, &mut state.graphics_state, context);
    state.input_state = input::get_input_state(state.camera_main, &mut state.touch_state, context);
    hike_graphics::game_ui::draw_world_ui(&state.world, context, &mut state.graphics_state);
    hike_graphics::game_ui::ui_update(
        &mut state.world,
        &mut state.input_state,
        &mut state.graphics_state.ui_state,
        context
    );
}