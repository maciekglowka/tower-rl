use rogalik::storage::World;

use hike_game::components::*;
use hike_game::{Board, GameStats};

pub fn register_serialized(world: &mut World) {
    world.register_serializable_resource::<Board>("Board");
    world.register_serializable_resource::<GameStats>("GameStats");

    world.register_serializable_component::<Actor>("Actor");
    world.register_serializable_component::<Budding>("Budding");
    world.register_serializable_component::<Collectable>("Collectable");
    world.register_serializable_component::<Defensive>("Defensive");
    world.register_serializable_component::<Durability>("Durability");
    world.register_serializable_component::<Discoverable>("Discoverable");
    world.register_serializable_component::<Effects>("Effects");
    world.register_serializable_component::<Fixture>("Fixture");
    world.register_serializable_component::<Health>("Health");
    world.register_serializable_component::<Instant>("Instant");
    world.register_serializable_component::<Interactive>("Interactive");
    world.register_serializable_component::<Item>("Item");
    world.register_serializable_component::<Info>("Info");
    world.register_serializable_component::<Loot>("Loot");
    world.register_serializable_component::<Obstacle>("Obstacle");
    world.register_serializable_component::<Offensive>("Offensive");
    world.register_serializable_component::<Ranged>("Ranged");
    world.register_serializable_component::<Summoner>("Summoner");
    world.register_serializable_component::<Tile>("Tile");
    world.register_serializable_component::<Transition>("Transition");
    world.register_serializable_component::<Weapon>("Weapon");
    world.register_serializable_component::<Immaterial>("Immaterial");
    world.register_serializable_component::<Lunge>("Lunge");
    world.register_serializable_component::<Swing>("Swing");
    world.register_serializable_component::<Push>("Push");
    world.register_serializable_component::<Switch>("Switch");
    world.register_serializable_component::<ViewBlocker>("ViewBlocker");
    world.register_serializable_component::<Name>("Name");
    world.register_serializable_component::<Player>("Player");
    world.register_serializable_component::<Immune>("Immune");
    world.register_serializable_component::<Stunned>("Stunned");
    world.register_serializable_component::<Poisoned>("Poisoned");
    world.register_serializable_component::<Projectile>("Projectile");
    world.register_serializable_component::<Position>("Position");
    world.register_serializable_component::<Regeneration>("Regeneration");
}