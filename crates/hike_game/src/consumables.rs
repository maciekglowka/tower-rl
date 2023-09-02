use rogalik::storage::{Entity, World};

use crate::actions::{Action, Heal, Repair};
use crate::components::{Consumable, ConsumableKind, Durability, Player};

pub fn get_consume_action(
    entity: Entity,
    consumer: Entity,
    world: &World
) -> Option<Box<dyn Action>> {
    let consumable = world.get_component::<Consumable>(entity)?;

    match consumable.kind {
        ConsumableKind::Heal => Some(Box::new(
            Heal { entity: consumer, value: consumable.value }
        )),
        // ConsumableKind::Repair => get_repair(world, consumable.value),
    }
}


fn get_repair(world: &World, value: u32) -> Option<Box<dyn Action>> {
    let query = world.query::<Player>();
    let player_item = query.iter().next()?;
    let player = player_item.get::<Player>()?;

    // if no item is active -> no repair
    let selected = player.items[player.active_item]?;
    if world.get_component::<Durability>(selected).is_none() { return None };
    Some(Box::new(Repair { entity: selected, value }))
}
