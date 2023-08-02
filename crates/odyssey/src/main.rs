use macroquad::prelude::*;
use std::time::{Duration, Instant};

mod input;

fn window_conf() -> Conf {
    Conf { 
        window_title: "Micro Odyssey".into(),
        window_width: 600,
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

    let mut game_data = odyssey_data::GameData::new();
    let fixtures = game_data.add_entities_from_str(fixture_data_str);
    let npcs = game_data.add_entities_from_str(npc_data_str);
    let player = game_data.add_entities_from_str(player_data_str);
    let tiles = game_data.add_entities_from_str(tile_data_str);

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

    let main_camera = Camera2D {
        zoom: Vec2::new(2. / screen_width(), 2. / screen_height()),
        target: 0.5 * odyssey_graphics::globals::TILE_SIZE * Vec2::splat(8.),
        ..Default::default()
    };

    let mut world = rogalik::storage::World::new();
    world.insert_resource(game_data);

    let mut manager = odyssey_game::GameManager::new();
    let mut graphics_state = odyssey_graphics::GraphicsState::new(
        &mut world,
        &mut manager
    );
    odyssey_game::init(&mut world, &mut manager);

    let mut graphics_ready = true;

    loop {
        let frame_start = Instant::now();

        if graphics_ready {
            odyssey_game::game_step(&mut world, &mut manager);
        }
        clear_background(BLACK);
        set_camera(&main_camera);
        backend.set_bounds(&main_camera);

        graphics_ready = odyssey_graphics::graphics_update(&world, &mut graphics_state, &backend);

        set_default_camera();
        odyssey_graphics::ui::ui_update(
            &mut world,
            input::get_input_state(&main_camera),
            &backend,
        );
        next_frame().await;

        // temp to save some cpu cycles
        std::thread::sleep(Duration::from_millis(16).saturating_sub(frame_start.elapsed()));
    }
}
