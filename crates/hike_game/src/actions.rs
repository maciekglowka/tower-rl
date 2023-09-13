use rogalik::{
    math::vectors::{Vector2I, ORTHO_DIRECTIONS},
    storage::{Entity, World}
};
use rand::prelude::*;
use std::{
    any::{Any, TypeId},
    cell::Ref,
    collections::VecDeque
};

use crate::board::Board;
use crate::components::{
    Durability, Stunned, Health, Interactive, Loot, Item,
    Obstacle, Position, Player, InteractionKind, Name, Poisoned,
    Hit, Poison, Stun, Swing
};
use crate::consumables::get_consume_action;
use crate::events::ActionEvent;
use crate::player::get_player_entity;
use crate::utils::{are_hostile, get_entities_at_position, spawn_with_position};

pub struct PendingActions(pub VecDeque<Box<dyn Action>>);
pub struct ActorQueue(pub VecDeque<Entity>);

pub type ActionResult = Result<Vec<Box<dyn Action>>, ()>;

pub trait Action {
    fn as_any(&self) -> &dyn Any;
    fn execute(&self, world: &mut World) -> ActionResult;
    fn event(&self) -> ActionEvent { ActionEvent::Other }
    fn score(&self, world: &World) -> i32 { 0 }
    fn type_id(&self) -> TypeId where Self: 'static {
        TypeId::of::<Self>()
    }
}

pub fn get_action_at_dir(
    entity: Entity,
    world: &World,
    dir: Vector2I
) -> Option<Box<dyn Action>> {
    let position = world.get_component::<Position>(entity)?;
    let target = position.0 + dir;
    let board = world.get_resource::<Board>()?;
    if !board.tiles.contains_key(&target) { return None };

    let entities = get_entities_at_position(world, target);

    let attackable = entities.iter()
        .any(|&e| world.get_component::<Health>(e).is_some());
    if attackable {
        return Some(Box::new(Attack { entity, target }));
    }

    if let Some(door) = entities.iter()
        .find(|&e| world.get_component::<Name>(*e).unwrap().0 == "Closed_Door") {
            return Some(Box::new(Replace { entity: *door, name: "Open_Door".to_string() }))
        }

    let bumpable = entities.iter()
        .any(|&e| world.get_component::<Obstacle>(e).is_some());
    if bumpable {
        // return Some(Box::new(Bump { entity, target }))
        return None
    }

    // otherwise should be safe to walk into
    Some(Box::new(Walk { entity, target }))
}

pub fn get_npc_action(
    entity: Entity,
    world: &World
) -> Box<dyn Action> {
    let mut possible_actions = ORTHO_DIRECTIONS.iter()
       .filter_map(|dir| get_action_at_dir(entity, world, *dir))
       .collect::<Vec<_>>();

   possible_actions.sort_by(|a, b| a.score(world).cmp(&b.score(world)));
   match possible_actions.pop() {
       Some(a) => a,
       _ => Box::new(Pause)
   }
}

pub struct Walk {
    pub entity: Entity,
    pub target: Vector2I
}
impl Action for Walk {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        let mut position = world.get_component_mut::<Position>(self.entity).ok_or(())?;
        position.0 = self.target;

        Ok(Vec::new())
    }
    fn event(&self) -> ActionEvent {
        ActionEvent::Travel(self.entity, self.target)
    }
    fn score(&self, world: &World) -> i32 {
        let Some(player_position) = world.query::<Player>().with::<Position>()
            .iter()
            .map(|i| i.get::<Position>().unwrap().0)
            .next()
            else { return 0 };

        20 - self.target.manhattan(player_position)
    }
}

pub struct Attack {
    // base attack action used to dispatch specific attack types
    pub entity: Entity,
    pub target: Vector2I
}
impl Attack {
    fn get_offending_entity(&self, world: &World) -> Entity {
        if let Some(player) = world.get_component::<Player>(self.entity) {
            if let Some(item) = player.items[player.active_item] {
                return item
            }
        }
        self.entity
    }
    fn get_attack_targets(&self, entity: Entity, world: &World) -> Vec<Vector2I> {
        if world.get_component::<Swing>(entity).is_none() {
            return vec![self.target]
        }

        let Some(position) = world.get_component::<Position>(self.entity) else { return vec![self.target] };
        let dir = position.0 - self.target;
        ORTHO_DIRECTIONS.iter()
            .filter(|d| **d != dir)
            .map(|d| position.0 + *d)
            .collect::<Vec<_>>()
        // if d.x != 0 {
        //     vec![
        //         self.target,
        //         Vector2I::new(self.target.x, self.target.y + 1),
        //         Vector2I::new(self.target.x, self.target.y - 1),
        //     ]
        // } else {
        //     vec![
        //         self.target,
        //         Vector2I::new(self.target.x + 1, self.target.y),
        //         Vector2I::new(self.target.x - 1, self.target.y),
        //     ] 
        // }
    }
    fn get_attack_actions(&self, entity: Entity, world: &World, v: Vector2I) -> Vec<Box<dyn Action>> {
        let mut actions: Vec<Box<dyn Action>> = Vec::new();

        if let Some(hit) = world.get_component::<Hit>(entity) {
            actions.push(Box::new(HitAction { 
                    entity: self.entity,
                    target: v,
                    value: hit.0
                }
            ));
        }
        if let Some(poison) = world.get_component::<Poison>(entity) {
            actions.push(Box::new(PoisonAction { 
                    entity: self.entity,
                    target: v,
                    value: poison.0
                }
            ));
        }
        if let Some(stun) = world.get_component::<Stun>(entity) {
            actions.push(Box::new(StunAction { 
                    entity: self.entity,
                    target: v,
                    value: stun.0
                }
            ));
        }
        actions
    }
}
impl Action for Attack {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        let offending_entity = self.get_offending_entity(world);
        let mut actions = self.get_attack_targets(offending_entity, world).iter()
            .map(|&v| self.get_attack_actions(offending_entity, world, v))
            .flatten()
            .collect::<Vec<_>>();
        
        if world.get_component::<Item>(offending_entity).is_some() {
            actions.push(Box::new(
                UseItem { entity: offending_entity }
            ));
        }
        Ok(actions)
    }
    fn event(&self) -> ActionEvent {
        ActionEvent::Attack(self.entity, self.target)
    }
    fn score(&self, world: &World) -> i32 {
        if get_entities_at_position(world, self.target).iter().any(
            |e| world.get_component::<Player>(*e).is_some()
        ) {
            200
        } else {
            -50
        }
    }
}

pub struct HitAction {
    pub entity: Entity,
    pub target: Vector2I,
    pub value: u32
}
impl Action for HitAction {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        let actions = get_entities_at_position(world, self.target).iter()
            .filter_map(|e| match world.get_component::<Health>(*e) {
                Some(_) => Some(
                    Box::new(Damage { entity: *e, value: self.value }) as Box<dyn Action>
                ),
                None => None
            })
            .collect::<Vec<_>>();
        Ok(actions)
    }
    // no score - should be a resulting action only
}

pub struct StunAction {
    pub entity: Entity,
    pub target: Vector2I,
    pub value: u32
}
impl Action for StunAction {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        for entity in get_entities_at_position(world, self.target) {
            if world.get_component::<Health>(entity).is_none() { continue }
            if let Some(mut stunned)  = world.get_component_mut::<Stunned>(entity) {
                stunned.0 += self.value;
                continue
            };
            let _ = world.insert_component(entity, Stunned(self.value));
        }
        Ok(Vec::new())
    }
}

pub struct PoisonAction {
    pub entity: Entity,
    pub target: Vector2I,
    pub value: u32
}
impl Action for PoisonAction {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        for entity in get_entities_at_position(world, self.target) {
            if world.get_component::<Health>(entity).is_none() { continue }
            if let Some(mut poisoned)  = world.get_component_mut::<Poisoned>(entity) {
                poisoned.0 += self.value;
                continue
            };
            let _ = world.insert_component(entity, Poisoned(self.value));
        }
        Ok(Vec::new())
    }
}

pub struct Bump {
    pub entity: Entity,
    pub target: Vector2I
}
impl Action for Bump {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        Ok(Vec::new())
    }
    fn event(&self) -> ActionEvent {
        ActionEvent::Bump(self.entity, self.target)
    }
}

pub struct UseItem {
    pub entity: Entity
}
impl Action for UseItem {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        if let Some(mut durability) = world.get_component_mut::<Durability>(self.entity) {
            durability.0 = durability.0.saturating_sub(1);
        }
        Ok(Vec::new())
    }
}

pub struct Pause;
impl Action for Pause {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult { Ok(Vec::new() )}
}

pub struct AddToInventory {
    pub entity: Entity
}
impl Action for AddToInventory {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        let player_query = world.query::<Player>();
        let player_item = player_query.iter().next().ok_or(())?;
        let mut player = player_item.get_mut::<Player>().ok_or(())?;
        let active = player.active_item;

        let current = player.items[active];
        player.items[active] = Some(self.entity);

        drop(player);

        world.remove_component::<Position>(self.entity);
        if let Some(current) = current {
            world.despawn_entity(current);
        }

        Ok(Vec::new())
    }
    // no score - npcs do not pick
}

pub struct Consume {
    pub entity: Entity,
    pub consumer: Entity
}
impl Action for Consume {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        if let Some(action) = get_consume_action(self.entity, self.consumer, world) {
            world.despawn_entity(self.entity);
            return Ok(vec![action]);
        }
        Ok(Vec::new())
    }
}

pub struct Damage {
    pub entity: Entity,
    pub value: u32
}
impl Action for Damage {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        let mut health = world.get_component_mut::<Health>(self.entity).ok_or(())?;
        health.0.current = health.0.current.saturating_sub(self.value);
        Ok(Vec::new())
    }
    // score is not implemented as it always should be a resulting action
}

pub struct Heal {
    pub entity: Entity,
    pub value: u32
}
impl Action for Heal {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        let mut health = world.get_component_mut::<Health>(self.entity).ok_or(())?;
        health.0.current = health.0.max.min(health.0.current + self.value);
        Ok(Vec::new())
    }
    // score is not implemented as it always should be a resulting action
}

pub struct Repair {
    pub entity: Entity,
    pub value: u32
}
impl Action for Repair {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        let mut durability = world.get_component_mut::<Durability>(self.entity).ok_or(())?;
        durability.0 += self.value;
        Ok(Vec::new())
    }
    // score is not implemented as it always should be a resulting action
}

pub struct UpgradeHealth {
    pub entity: Entity,
    pub value: u32
}
impl Action for UpgradeHealth {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        let mut health = world.get_component_mut::<Health>(self.entity).ok_or(())?;
        health.0.max += self.value;
        health.0.current += self.value;
        Ok(Vec::new())
    }
    // score is not implemented as it always should be a resulting action
}

// pub struct UpgradeOffensive {
//     pub entity: Entity,
//     pub value: u32
// }
// impl Action for UpgradeOffensive {
//     fn as_any(&self) -> &dyn Any { self }
//     fn execute(&self, world: &mut World) -> ActionResult {
//         let mut offensive = world.get_component_mut::<Offensive>(self.entity).ok_or(())?;
//         offensive.value += self.value;
//         Ok(Vec::new())
//     }
//     // score is not implemented as it always should be a resulting action
// }

pub struct PickGold {
    pub value: u32
}
impl Action for PickGold {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        let entity = get_player_entity(world).ok_or(())?;
        let mut player = world.get_component_mut::<Player>(entity).ok_or(())?;
        player.gold += self.value;
        Ok(Vec::new())
    }
    // score is not implemented as it always should be a resulting action
}

pub struct Pay {
    pub value: u32
}
impl Action for Pay {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        let entity = get_player_entity(world).ok_or(())?;
        let mut player = world.get_component_mut::<Player>(entity).ok_or(())?;
        player.gold = player.gold.saturating_sub(self.value);
        Ok(Vec::new())
    }
    // score is not implemented as it always should be a resulting action
}

pub struct Interact {
    pub entity: Entity
}
impl Action for Interact {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        let mut res: Vec<Box< dyn Action>> = Vec::new();
        let interactive = world.get_component::<Interactive>(self.entity).ok_or(())?;

        if let Some(cost) = interactive.cost {
            if cost > world.query::<Player>().iter().next().ok_or(())?
                .get::<Player>().unwrap().gold {
                    return Err(());
                }
                res.push(Box::new(Pay { value: cost }));
        }

        let action: Box<dyn Action> = match interactive.kind {
            InteractionKind::Ascend => Box::new(Ascend),
            InteractionKind::Repair(value) => {
                let idx = world.query::<Player>().iter().next().ok_or(())?
                    .get::<Player>().unwrap().active_item;
                let item = world.query::<Player>().iter().next().ok_or(())?
                    .get::<Player>().unwrap().items[idx].ok_or(())?;
                Box::new(Repair { entity: item, value } )
            },
            // InteractionKind::UpgradeOffensive(value) => {
            //     let idx = world.query::<Player>().iter().next().ok_or(())?
            //         .get::<Player>().unwrap().active_item;
            //     let item = world.query::<Player>().iter().next().ok_or(())?
            //         .get::<Player>().unwrap().items[idx].ok_or(())?;
            //     Box::new(UpgradeOffensive { entity: item, value } )
            // },
            InteractionKind::UpgradeHealth(value) => {
                let player = world.query::<Player>().iter().next().ok_or(())?.entity;
                Box::new(UpgradeHealth { entity: player, value } )
            }
        };
        res.push(action);
        if let Some(next) = &interactive.next {
            res.push(Box::new(Replace {
                entity: self.entity, name: next.to_string()
            }));
        }
        Ok(res)
    }
}

pub struct Replace {
    pub entity: Entity,
    pub name: String
}
impl Action for Replace {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        let position = world.get_component::<Position>(self.entity).ok_or(())?.0;
        world.despawn_entity(self.entity);
        spawn_with_position(world, &self.name, position);
        Ok(Vec::new())
    }
    fn score(&self, world: &World) -> i32 {
        // npcs should not do those things :)
        -200
    }
}

pub struct Ascend;
impl Action for Ascend {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        world.get_resource_mut::<Board>().ok_or(())?.exit = true;
        Ok(Vec::new())
    }
}

pub struct DropLoot {
    pub entity: Entity
}
impl Action for DropLoot {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        let mut rng = thread_rng();
        let loot = world.get_component::<Loot>(self.entity).ok_or(())?;
        let position = world.get_component::<Position>(self.entity).ok_or(())?.0;

        if !rng.gen_bool(loot.chance as f64) { return Ok(Vec::new()) };

        let name = loot.items.choose(&mut rng).ok_or(())?.to_string();
        drop(loot);
        spawn_with_position(world, &name, position);
        Ok(Vec::new())
    }
}
