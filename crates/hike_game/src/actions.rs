use rogalik::{
    math::vectors::{Vector2I, ORTHO_DIRECTIONS},
    storage::{Entity, World}
};
use std::{
    any::{Any, TypeId},
    cell::Ref,
    collections::VecDeque
};

use crate::board::Board;
use crate::components::{AttackKind, Durability, Health, Obstacle, Offensive, Position, Player};
use crate::events::ActionEvent;
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

    let bumpable = entities.iter()
        .any(|&e| world.get_component::<Obstacle>(e).is_some());
    if bumpable {
        return Some(Box::new(Bump { entity, target }))
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
    fn get_offensive<'a>(&'a self, world: &'a World) -> Option<(Ref<'a, Offensive>, Option<Entity>)> {
        // returns (Attack, SourceItem)

        if let Some(player) = world.get_component::<Player>(self.entity) {
            if let Some(item) = player.items[player.active_item] {
                if let Some(offensive) = world.get_component::<Offensive>(item) {
                    return Some((offensive, Some(item)));
                }
            }
        }
        Some((world.get_component::<Offensive>(self.entity)?, None))
    }
    fn get_attack_action(&self, offensive: &Offensive) -> Box<dyn Action> {
        match offensive.kind {
            AttackKind::Hit => Box::new(Hit { 
                entity: self.entity,
                target: self.target,
                value: offensive.value 
            })
        }
    }
}
impl Action for Attack {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        let (offensive, item) = self.get_offensive(world).ok_or(())?;
        let mut actions = vec![self.get_attack_action(&offensive)];

        if let Some(item) = item {
            actions.push(Box::new(
                UseItem { entity: item }
            ))
        }

        Ok(actions)
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

pub struct Hit {
    pub entity: Entity,
    pub target: Vector2I,
    pub value: u32
}
impl Action for Hit {
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
    fn event(&self) -> ActionEvent {
        ActionEvent::Melee(self.entity, self.target, self.value)
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
        ActionEvent::Travel(self.entity, self.target)
    }
}

pub struct UseItem {
    pub entity: Entity
}
impl Action for UseItem {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        if let Some(mut durability) = world.get_component_mut::<Durability>(self.entity) {
            durability.value = durability.value.saturating_sub(1);
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
        health.0 = health.0.saturating_sub(self.value);
        Ok(Vec::new())
    }
    // score is not implemented as it always should be a resulting action
}

pub struct Pause;
impl Action for Pause {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult { Ok(Vec::new() )}
}

pub struct PickItem {
    pub entity: Entity
}
impl Action for PickItem {
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
    fn score(&self, world: &World) -> i32 {
        // npcs do not pick
        0
    }
}