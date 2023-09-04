use rogalik::{
    math::vectors::Vector2F,
    storage::World
};

use hike_game::{
    components::{Health, Player, Position, Name, Item, Interactive, Durability},
    get_entities_at_position, get_player_position
};

use super::{GraphicsBackend, SpriteColor};

pub fn draw_status(world: &World, backend: &dyn GraphicsBackend) {
    let query = world.query::<Player>().with::<Health>();
    let Some(item) = query.iter().next() else { return };

    let health = item.get::<Health>().unwrap();
    let player = item.get::<Player>().unwrap();

    backend.draw_ui_text(
        "default",
        &format!("HP: {}/{} Gold: {}", health.0.current, health.0.max, player.gold),
        Vector2F::new(10., 42.),
        32,
        SpriteColor(0, 0, 0, 255)
    );
    // if let Some(s) = get_item_desc(world) {
    //     backend.draw_ui_text(
    //         "default",
    //         &format!("{}", s),
    //         Vector2F::new(10., 74.),
    //         32,
    //         SpriteColor(0, 0, 0, 255)
    //     );
    // }
}
