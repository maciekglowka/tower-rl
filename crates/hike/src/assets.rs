use rogalik::engine::GraphicsContext;
use rogalik::math::vectors::Vector2f;
use hike_data::GameData;

use super::{Context_, GameState};

pub fn load_assets(state: &mut GameState, context: &mut Context_) {
    load_textures(state, context);
}

pub fn load_game_data() -> GameData {
    let mut game_data = hike_data::GameData::new();

    let fixtures = game_data.add_entities_from_str(
        include_str!("../../../assets/data/fixtures.yaml").to_string()
    );
    // let traps = game_data.add_entities_from_str(
    //     include_str!("../../../assets/data/traps.yaml").to_string()
    // );
    let npcs = game_data.add_entities_from_str(
        include_str!("../../../assets/data/npcs.yaml").to_string()
    );
    let _ = game_data.add_entities_from_str(
        include_str!("../../../assets/data/player.yaml").to_string()
    );
    let _ = game_data.add_entities_from_str(
        include_str!("../../../assets/data/board_elements.yaml").to_string()
    );
    let mut items = game_data.add_entities_from_str(
        include_str!("../../../assets/data/items.yaml").to_string()
    );
    let discoverables = game_data.add_entities_from_str(
        include_str!("../../../assets/data/discoverables.yaml").to_string()
    );
    let weapons = game_data.add_entities_from_str(
        include_str!("../../../assets/data/weapons.yaml").to_string()
    );

    items.extend(discoverables.clone());

    game_data.npcs = npcs;
    game_data.discoverables = discoverables;
    game_data.assign_discoverables();

    game_data.items = items;
    game_data.fixtures = fixtures;
    game_data.weapons = weapons;
    // game_data.traps = traps;
    
    game_data.add_level_data_from_str(
        include_str!("../../../assets/data/levels.yaml").to_string()
    );
    game_data
}

fn load_textures(state: &mut GameState, context: &mut Context_) {
    context.graphics.load_sprite_atlas(
        "ascii", include_bytes!("../../../assets/sprites/ascii.png"), 16, 16, None
    );
    context.graphics.load_sprite_atlas(
        "tiles", include_bytes!("../../../assets/sprites/tiles.png"), 8, 8, None
    );
    context.graphics.load_sprite_atlas(
        "items", include_bytes!("../../../assets/sprites/items.png"), 8, 8, None
    );
    context.graphics.load_sprite_atlas(
        "icons", include_bytes!("../../../assets/sprites/icons.png"), 8, 8, None
    );
    context.graphics.load_sprite_atlas(
        "fog", include_bytes!("../../../assets/sprites/fog.png"), 2, 2, None
    );
    context.graphics.load_sprite_atlas(
        "ui", include_bytes!("../../../assets/ui/sprites.png"), 4, 4, None
    );
    context.graphics.load_sprite_atlas(
        "units", include_bytes!("../../../assets/sprites/units.png"), 8, 8, None
    );

    context.graphics.load_font(
        "default", include_bytes!("../../../assets/ui/pico_ascii.png"), 16, 16, Some((12., 0.))
    );

}