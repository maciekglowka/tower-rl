use rogalik::{
    engine::GraphicsContext,
    storage::World
};

use hike_game::components::Position;

use super::{Context_, GraphicsState};

pub mod renderers;
pub mod utils;

pub fn graphics_update(
    world: &World,
    state: &mut GraphicsState,
    context: &mut Context_
) -> bool {
    renderers::handle_world_events(world, state);
    renderers::handle_action_events(world, state);
    let ready = renderers::update_sprites(world, state);

    renderers::draw_sprites(world, state, context);
    renderers::draw_fog(world, state, context);
    ready
}