use rogalik::storage::{Entity, World};

use odyssey_data::{CardData, CardKind};

use crate::abilities;
use crate::components::{Card, Cooldown};

pub fn spawn_card(world: &mut World, data: &CardData) -> Entity {
    let entity = world.spawn_entity();

    let _ = world.insert_component(entity, Card(
        get_ability_by_kind(data.kind)
    ));
    if let Some(cooldown) = data.cooldown {
        let _ = world.insert_component(entity, Cooldown{
            base: cooldown, current: 0
        });
    }
    entity
}

fn get_ability_by_kind(kind: CardKind) -> Box<dyn abilities::Ability> {
    match kind {
        CardKind::Abordage(damage) => Box::new(abilities::Abordage { damage }),
        CardKind::Buoy(health) => Box::new(abilities::Buoy{ health }),
        CardKind::Cannons(dist, damage) => Box::new(abilities::Cannons { dist, damage }),
        CardKind::Sailing => Box::new(abilities::Sailing),
        CardKind::Swimming(dist) => Box::new(abilities::Swimming { dist }),
    }
}