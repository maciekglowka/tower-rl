use rogalik::storage::{Entity, World};

use hike_game::components::{Hit, Durability, Poison, Stun, Swing};

pub fn get_entity_icons(entity: Entity, world: &World) -> Vec<(u32, Option<u32>)> {
    // surely can be done better?
    let mut output = Vec::new();

    if let Some(hit) = world.get_component::<Hit>(entity) {
        output.push((0, Some(hit.0)));
    }
    if let Some(stun) = world.get_component::<Stun>(entity) {
        output.push((3, Some(stun.0)));
    }
    if let Some(poison) = world.get_component::<Poison>(entity) {
        output.push((1, Some(poison.0)));
    }
    if let Some(durability) = world.get_component::<Durability>(entity) {
        output.push((2, Some(durability.0)));
    }
    if let Some(_swing) = world.get_component::<Swing>(entity) {
        output.push((4, None));
    }
    output
}