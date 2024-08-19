// #![windows_subsystem = "windows"]
use rogalik::{
    engine::{Context, Game, GraphicsContext, EngineBuilder, ResourceId},
    events::{EventBus, SubscriberHandle},
    math::vectors::{Vector2f, Vector2i},
    persist,
    storage::World,
    wgpu::WgpuContext
};
use std::collections::HashMap;

#[cfg(target_os = "android")]
use rogalik::engine::AndroidApp;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

mod assets;
mod input;
mod serialize;

pub type Context_ = Context<WgpuContext>;
const SETTINGS_NAME: &str = "monk_settings";
const SAVE_NAME: &str = "monk_save";

#[derive(Default)]
enum GamePhase {
    #[default]
    Title,
    Crash,
    GameStart,
    GameResume,
    Game,
    GameRestart,
    GameEnd
}

struct Events {
    game_events: EventBus<hike_game::GameEvent>,
    ui_events: EventBus<hike_graphics::UiEvent>,
}
impl Events {
    pub fn new() -> Self {
        Self {
            game_events: EventBus::new(),
            ui_events: EventBus::new(),
        }
    }
}

pub struct GameState {
    audio: hike_audio::AudioContext,
    phase: GamePhase,
    can_resume: bool,
    data: hike_data::GameData,
    camera_main: ResourceId,
    events: Events,
    ev_game: SubscriberHandle<hike_game::GameEvent>,
    ev_ui: SubscriberHandle<hike_graphics::UiEvent>,
    graphics_ready: bool,
    graphics_state: hike_graphics::GraphicsState,
    input_state: hike_graphics::game_ui::InputState,
    settings: hike_data::Settings,
    touch_state: HashMap<u64, input::Touch>,
    world: World
}
impl Game<WgpuContext> for GameState {
    fn setup(&mut self, context: &mut Context<WgpuContext>) {
        if let Ok(settings) = persist::load(SETTINGS_NAME, context.os_path.as_deref()) {
            self.settings = settings;
        }
        self.can_resume = persist::load_raw(SAVE_NAME, context.os_path.as_deref()).is_ok();

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
        match self.phase {
            GamePhase::Title => {
                let input_state = input::get_input_state(
                    self.camera_main,
                    &mut self.touch_state,
                    &self.settings,
                    context
                );

                match hike_graphics::title_ui::update_title_ui(context, &input_state, self.can_resume) {
                    hike_graphics::title_ui::TitleMenuAction::Resume => self.phase = GamePhase::GameResume,
                    hike_graphics::title_ui::TitleMenuAction::Start => self.phase = GamePhase::GameStart,
                    hike_graphics::title_ui::TitleMenuAction::None => ()
                }
            },
            GamePhase::Crash => {
                let input_state = input::get_input_state(
                    self.camera_main,
                    &mut self.touch_state,
                    &self.settings,
                    context
                );

                if hike_graphics::crash_ui::update_crash_ui(context, &input_state) {
                    self.phase = GamePhase::Title;
                }
            },
            GamePhase::Game => {
                game_update(self, context);
                for ev in self.ev_game.read().iter().flatten() {
                    match *ev {
                        hike_game::GameEvent::TurnEnd => {
                            if let Ok(save) = self.world.serialize() {
                                let _ = persist::store_raw(SAVE_NAME, &save, context.os_path.as_deref());
                            }
                        },
                        hike_game::GameEvent::Defeat | hike_game::GameEvent::Win => {
                            let _ = persist::remove(SAVE_NAME, context.os_path.as_deref());
                            self.phase = GamePhase::GameEnd
                        },
                        _ => ()
                    }
                }
            },
            GamePhase::GameEnd => {
                game_ui_update(self, context);
                for ev in self.ev_ui.read().iter().flatten() {
                    match ev {
                        hike_graphics::UiEvent::Restart => self.phase = GamePhase::GameRestart
                    }
                }
            },
            GamePhase::GameStart => {
                hike_game::init(&mut self.world, &mut self.events.game_events, self.data.clone());
                self.phase = GamePhase::Game;
            },
            GamePhase::GameResume => {
                self.can_resume = false;
                if let Ok(saved_state) = persist::load_raw(SAVE_NAME, context.os_path.as_deref()) {
                    hike_game::restore(&mut self.world, self.data.clone(), saved_state);
                    self.graphics_state.restore(&self.world);
                    self.phase = GamePhase::Game;
                } else {
                    self.phase = GamePhase::Crash
                }
            },
            GamePhase::GameRestart => {
                (self.world, self.events, self.graphics_state, self.audio) = get_initial_elements();
                self.ev_ui = self.events.ui_events.subscribe();
                self.ev_game = self.events.game_events.subscribe();
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
    #[cfg(target_arch="wasm32")]
    fn resize(&mut self, context: &mut rogalik::engine::Context<WgpuContext>) {
        let scale = 16. * (context.get_physical_size().x as u32 / 10 / 16) as f32;
        if let Some(camera) = context.graphics.get_camera_mut(self.camera_main) {
            camera.set_scale(scale);
        }
    }
    fn resume(&mut self, context: &mut rogalik::engine::Context<WgpuContext>) {
        self.audio = hike_audio::get_audio_context(&mut self.events.game_events);
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
    let engine = EngineBuilder::new()
        .with_title("Tower RL".to_string())
        .with_logical_size(600., 760.)
        .build(game_state());
    engine.run();
}

#[cfg(target_arch="wasm32")]
#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub fn wasm_main() {
    let engine = EngineBuilder::new()
        .build_wasm(game_state());
    engine.run();
}

fn game_state() -> GameState {
    let (world, mut events, graphics_state, audio) = get_initial_elements();
    let ev_ui = events.ui_events.subscribe();
    let ev_game = events.game_events.subscribe();

    GameState {
        audio,
        phase: GamePhase::default(),
        camera_main: ResourceId::default(),
        can_resume: false,
        data: assets::load_game_data(),
        events,
        ev_ui,
        ev_game,
        graphics_ready: false,
        graphics_state,
        input_state: hike_graphics::game_ui::InputState::default(),
        settings: hike_data::Settings::default(),
        touch_state: HashMap::new(),
        world
    }
}

fn get_initial_elements() -> (World, Events, hike_graphics::GraphicsState, hike_audio::AudioContext) {
    let mut world = World::new();
    serialize::register_serialized(&mut world);
    let mut events = Events::new();
    let mut graphics_state = hike_graphics::GraphicsState::new(
        &mut world,
        &mut events.game_events
    );
    graphics_state.ui_state.build_version = env!("CARGO_PKG_VERSION").to_string();
    let audio = hike_audio::get_audio_context(&mut events.game_events);
    (world, events, graphics_state, audio)
}

fn game_update(state: &mut GameState, context: &mut Context_) {
    if state.graphics_ready {
        let _ = hike_game::game_update(&mut state.world, &mut state.events.game_events);
    }

    state.graphics_ready = hike_graphics::graphics_update(&state.world, &mut state.graphics_state, context);
    // state.input_state = input::get_input_state(state.camera_main, &mut state.touch_state, &state.settings, context);
    hike_graphics::game_ui::draw_world_ui(&state.world, context, &mut state.graphics_state);
    // hike_graphics::game_ui::ui_update(
    //     &mut state.world,
    //     &mut state.input_state,
    //     &mut state.graphics_state.ui_state,
    //     &mut state.events.ui_events,
    //     context,
    //     &mut state.settings
    // );
    game_ui_update(state, context);
    hike_audio::handle_game_audio(&mut state.audio, &state.world);
    if state.settings.dirty {
        state.settings.dirty = false;
        let _ = persist::store(SETTINGS_NAME, &state.settings, context.os_path.as_deref());
    }
}

fn game_ui_update(state: &mut GameState, context: &mut Context_) {
    state.input_state = input::get_input_state(state.camera_main, &mut state.touch_state, &state.settings, context);
    hike_graphics::game_ui::ui_update(
        &mut state.world,
        &mut state.input_state,
        &mut state.graphics_state.ui_state,
        &mut state.events.ui_events,
        context,
        &mut state.settings
    );
}
