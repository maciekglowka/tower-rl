use rogalik::storage::World;

use hike_game::components::Position;

use super::GraphicsState;

pub mod renderers;
pub mod utils;

use super::GraphicsBackend;

pub fn graphics_update(
    world: &World,
    state: &mut GraphicsState,
    backend: &dyn GraphicsBackend
) -> bool {
    renderers::handle_world_events(world, state);
    renderers::handle_action_events(world, state);
    let ready = renderers::update_sprites(world, state);

    renderers::draw_sprites(world, state, backend);
    renderers::draw_fog(world, backend);
    ready
}