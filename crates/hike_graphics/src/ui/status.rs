use rogalik::{
    math::vectors::Vector2F,
    storage::World
};

use hike_game::{
    components::{Health, Player, Poisoned},
    get_entities_at_position, get_player_position
};

use super::{GraphicsBackend, SpriteColor};

pub fn draw_status(world: &World, backend: &dyn GraphicsBackend) {
    let query = world.query::<Player>().with::<Health>();
    let Some(item) = query.iter().next() else { return };

    let health = item.get::<Health>().unwrap();
    let player = item.get::<Player>().unwrap();

    let mut text = format!("HP: {}/{} Gold: {}", health.0.current, health.0.max, player.gold);
    if world.get_component::<Poisoned>(item.entity).is_some() {
        text += " Poisoned";
    }

    backend.draw_ui_text(
        "default",
        &text,
        Vector2F::new(10., 42.),
        32,
        SpriteColor(150, 128, 128, 255)
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
