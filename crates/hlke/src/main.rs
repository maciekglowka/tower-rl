use macroquad::prelude::*;
use std::time::{Duration, Instant};

mod input;

fn window_conf() -> Conf {
    Conf { 
        window_title: "Micro Hike".into(),
        window_width: 540,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");
    let fixture_data_str = load_string("data/fixtures.yaml").await.expect("Could not load data!");
    let npc_data_str = load_string("data/npcs.yaml").await.expect("Could not load data!");
    let player_data_str = load_string("data/player.yaml").await.expect("Could not load data!");
    let tile_data_str = load_string("data/tiles.yaml").await.expect("Could not load data!");
    let item_data_str = load_string("data/items.yaml").await.expect("Could not load data!");

    let mut game_data = hike_data::GameData::new();
    let fixtures = game_data.add_entities_from_str(fixture_data_str);
    let npcs = game_data.add_entities_from_str(npc_data_str);
    let player = game_data.add_entities_from_str(player_data_str);
    let tiles = game_data.add_entities_from_str(tile_data_str);
    let items = game_data.add_entities_from_str(item_data_str);

    let mut backend = macroquad_sprites::MacroquadBackend::new();

    backend.load_atlas(
            "ascii",
            "sprites/ascii.png",
            16,
            16,
            None
        ).await
        .expect("Could not load sprites!");

    backend.load_font("default",  "ui/04B_03.ttf").await
        .expect("Could not find fonts!");

    let camera_target = 0.5 * hike_graphics::globals::TILE_SIZE * hike_game::globals::BOARD_SIZE as f32;

    let main_camera = Camera2D {
        zoom: Vec2::new(2. / screen_width(), 2. / screen_height()),
        target: Vec2::splat(camera_target),
        ..Default::default()
    };

    let mut world = rogalik::storage::World::new();
    world.insert_resource(game_data);

    let mut manager = hike_game::GameManager::new();
    let mut graphics_state = hike_graphics::GraphicsState::new(
        &mut world,
        &mut manager
    );
    hike_game::init(&mut world, &mut manager);

    let mut graphics_ready = true;

    loop {
        let frame_start = Instant::now();

        if graphics_ready {
            hike_game::game_update(&mut world, &mut manager);
        }
        clear_background(BLACK);
        set_camera(&main_camera);
        backend.set_bounds(&main_camera);

        graphics_ready = hike_graphics::graphics_update(&world, &mut graphics_state, &backend);

        set_default_camera();
        hike_graphics::ui::ui_update(
            &mut world,
            input::get_input_state(&main_camera),
            &backend,
        );
        next_frame().await;

        // temp to save some cpu cycles
        std::thread::sleep(Duration::from_millis(16).saturating_sub(frame_start.elapsed()));
    }
}