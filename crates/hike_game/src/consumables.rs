use rogalik::storage::{Entity, World};

use crate::actions::{Action, Heal, PickGold, MakeImmune};
use crate::components::{Consumable, ConsumableKind};

pub fn get_consume_action(
    entity: Entity,
    consumer: Entity,
    world: &World
) -> Option<Box<dyn Action>> {
    let consumable = world.get_component::<Consumable>(entity)?;

    match consumable.kind {
        ConsumableKind::Gold => Some(Box::new(
            PickGold { value: consumable.value }
        )),
        ConsumableKind::Heal => Some(Box::new(
            Heal { entity: consumer, value: consumable.value }
        )),
        ConsumableKind::Immunity => Some(Box::new(
            MakeImmune { entity: consumer, value: consumable.value }
        ))
    }
}
