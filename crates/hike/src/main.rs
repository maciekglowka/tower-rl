use rogalik::{
    engine::{Context, Game, GraphicsContext, EngineBuilder, ResourceId},
    math::vectors::{Vector2f, Vector2i},
    storage::World,
    wgpu::WgpuContext
};
use std::{
    collections::HashMap,
    time::{Duration, Instant}
};

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
            64., board_centre
        );
        context.graphics.set_camera(self.camera_main);
    
        hike_game::init(&mut self.world, &mut self.events);
        self.touch_state = HashMap::new();
    }
    fn update(&mut self, context: &mut rogalik::engine::Context<WgpuContext>) {
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
}

fn main() {
    std::env::set_var("WINIT_UNIX_BACKEND", "x11");
    run();
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
fn run() {
    let mut world = World::new();
    let mut events = hike_game::GameEvents::new();
    let graphics_state = hike_graphics::GraphicsState::new(
        &mut world,
        &mut events
    );

    let game_state = GameState {
        camera_main: ResourceId::default(),
        events,
        graphics_ready: false,
        graphics_state,
        touch_state: HashMap::new(),
        world
    };
    let engine = EngineBuilder::new()
        .with_title("Tower RL".to_string())
        .with_logical_size(600., 800.)
        .build(game_state);
    engine.run();
}

// // #[macroquad::main(window_conf)]
// async fn _main() {
//     // set_pc_assets_folder("assets");
//     // let fixture_data_str = load_string("data/fixtures.yaml").await.expect("Could not load data!");
//     // let npc_data_str = load_string("data/npcs.yaml").await.expect("Could not load data!");
//     // let player_data_str = load_string("data/player.yaml").await.expect("Could not load data!");
//     // let board_data_str = load_string("data/board_elements.yaml").await.expect("Could not load data!");
//     // let item_data_str = load_string("data/items.yaml").await.expect("Could not load data!");

//     // let level_data_str = load_string("data/levels.yaml").await.expect("Could not load data!");

//     // let mut game_data = hike_data::GameData::new();
//     // let fixtures = game_data.add_entities_from_str(fixture_data_str);
//     // let npcs = game_data.add_entities_from_str(npc_data_str);
//     // let _ = game_data.add_entities_from_str(player_data_str);
//     // let _ = game_data.add_entities_from_str(board_data_str);
//     // let items = game_data.add_entities_from_str(item_data_str);
//     // game_data.npcs = npcs;
//     // game_data.items = items;
//     // game_data.fixtures = fixtures;
    
//     // game_data.add_level_data_from_str(level_data_str);

//     // let mut backend = macroquad_sprites::MacroquadBackend::new();

//     // for (name, cols, rows) in [
//     //     ("ascii", 16, 16), ("tiles", 4, 4), ("items", 4, 4), ("icons", 4, 4), ("fog", 1, 1)
//     //     ] {
//     //     backend.load_atlas(
//     //             name,
//     //             &format!("sprites/{}.png", name),
//     //             cols,
//     //             rows,
//     //             None
//     //         ).await
//     //         .expect("Could not load sprites!");
//     // }

//     // backend.load_font("default",  "ui/04B_03.ttf").await
//     //     .expect("Could not find fonts!");

    

//     let board_centre = hike_graphics::tile_to_world(
//         rogalik::math::vectors::vector2::Vector2i::new(
//             hike_game::globals::BOARD_SIZE as i32 / 2,
//             hike_game::globals::BOARD_SIZE as i32 / 2
//         )
//     );
//     let camera_target = Vec2::new(
//         board_centre.x,
//         board_centre.y
//     );

//     let mut main_camera = Camera2D {
//         zoom: Vec2::new(2. / screen_width(), 2. / screen_height()),
//         target: camera_target,
//         ..Default::default()
//     };

//     let mut events = hike_game::GameEvents::new();
//     let mut graphics_state = hike_graphics::GraphicsState::new(
//         &mut world,
//         &mut events
//     );
//     hike_game::init(&mut world, &mut events);

//     let mut graphics_ready = true;
//     let mut touch_state = HashMap::new();
//     let bg_color = Color::from_rgba(
//         hike_graphics::globals::BACKGROUND_COLOR.0,
//         hike_graphics::globals::BACKGROUND_COLOR.1,
//         hike_graphics::globals::BACKGROUND_COLOR.2,
//         hike_graphics::globals::BACKGROUND_COLOR.3,
//     );

//     loop {
//         let frame_start = Instant::now();
//         update_camera(&world, &mut main_camera);

//         if graphics_ready {
//             hike_game::game_update(&mut world, &mut events);
//         }
//         // clear_background(Color::from_rgba(124, 182, 219, 255));
//         clear_background(bg_color);
//         // clear_background(BLACK);
//         set_camera(&main_camera);
//         backend.set_bounds(&main_camera);

//         graphics_ready = hike_graphics::graphics_update(&world, &mut graphics_state, &backend);
//         hike_graphics::ui::draw_world_ui(&world, &backend, &graphics_state);

//         set_default_camera();
//         hike_graphics::ui::ui_update(
//             &mut world,
//             input::get_input_state(&main_camera, &mut touch_state),
//             &backend,
//             UI_SCALE
//         );
//         next_frame().await;

//         // temp to save some cpu cycles
//         std::thread::sleep(Duration::from_millis(16).saturating_sub(frame_start.elapsed()));
//     }
// }
