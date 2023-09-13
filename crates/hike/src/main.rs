use macroquad::prelude::*;
use std::{
    collections::HashMap,
    time::{Duration, Instant}
};

mod input;

#[cfg(not(target_os = "android"))]
fn window_conf() -> Conf {
    Conf { 
        window_title: "Micro Hike".into(),
        window_width: 600,
        window_height: 800,
        ..Default::default()
    }
}

#[cfg(target_os = "android")]
fn window_conf() -> Conf {
    Conf { 
        window_title: "Micro Hike".into(),
        window_width: 1080,
        window_height: 2340,
        ..Default::default()
    }
}

#[cfg(not(target_os = "android"))]
const UI_SCALE: f32 = 1.0;

#[cfg(target_os = "android")]
const UI_SCALE: f32 = 2.0;

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");
    let fixture_data_str = load_string("data/fixtures.yaml").await.expect("Could not load data!");
    let npc_data_str = load_string("data/npcs.yaml").await.expect("Could not load data!");
    let player_data_str = load_string("data/player.yaml").await.expect("Could not load data!");
    let board_data_str = load_string("data/board_elements.yaml").await.expect("Could not load data!");
    let item_data_str = load_string("data/items.yaml").await.expect("Could not load data!");

    let level_data_str = load_string("data/levels.yaml").await.expect("Could not load data!");

    let mut game_data = hike_data::GameData::new();
    let fixtures = game_data.add_entities_from_str(fixture_data_str);
    let npcs = game_data.add_entities_from_str(npc_data_str);
    let _ = game_data.add_entities_from_str(player_data_str);
    let _ = game_data.add_entities_from_str(board_data_str);
    let items = game_data.add_entities_from_str(item_data_str);
    game_data.npcs = npcs;
    game_data.items = items;
    game_data.fixtures = fixtures;
    
    game_data.add_level_data_from_str(level_data_str);

    let mut backend = macroquad_sprites::MacroquadBackend::new();

    for (name, cols, rows) in [("ascii", 16, 16), ("tiles", 4, 4), ("items", 4, 4)] {
        backend.load_atlas(
                name,
                &format!("sprites/{}.png", name),
                cols,
                rows,
                None
            ).await
            .expect("Could not load sprites!");
    }

    backend.load_font("default",  "ui/04B_03.ttf").await
        .expect("Could not find fonts!");

    let board_centre = 0.5 * hike_graphics::globals::TILE_SIZE * hike_game::globals::BOARD_SIZE as f32;
    let camera_target = Vec2::new(
        board_centre,
        board_centre // + 0.5 * hike_graphics::globals::TILE_SIZE
    );

    let mut main_camera = Camera2D {
        zoom: Vec2::new(2. / screen_width(), 2. / screen_height()),
        target: camera_target,
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
    let mut touch_state = HashMap::new();

    loop {
        let frame_start = Instant::now();
        update_camera(&world, &mut main_camera);

        if graphics_ready {
            hike_game::game_update(&mut world, &mut manager);
        }
        // clear_background(Color::from_rgba(124, 182, 219, 255));
        clear_background(Color::from_rgba(31, 15, 28, 255));
        // clear_background(BLACK);
        set_camera(&main_camera);
        backend.set_bounds(&main_camera);

        graphics_ready = hike_graphics::graphics_update(&world, &mut graphics_state, &backend);
        hike_graphics::ui::draw_world_ui(&world, &backend, &graphics_state);

        set_default_camera();
        hike_graphics::ui::ui_update(
            &mut world,
            input::get_input_state(&main_camera, &mut touch_state),
            &backend,
            UI_SCALE
        );
        next_frame().await;

        // temp to save some cpu cycles
        std::thread::sleep(Duration::from_millis(16).saturating_sub(frame_start.elapsed()));
    }
}

fn update_camera(
    world: &rogalik::storage::World,
    camera: &mut Camera2D
) {
    // let Some(board) = world.get_resource::<hike_game::Board>() else { return };
    // let offset = 0.5 * hike_game::globals::BOARD_SIZE as f32;
    // let target = rogalik::math::vectors::Vector2F::new(
    //     board.origin.x as f32 + offset,
    //     board.origin.y as f32 + offset
    // ) * hike_graphics::globals::TILE_SIZE;
    // let v = hike_graphics::move_towards(
    //     rogalik::math::vectors::Vector2F::new(camera.target.x, camera.target.y),
    //     target,
    //     2.5
    // );
    // camera.target = Vec2::new(v.x, v.y);
}