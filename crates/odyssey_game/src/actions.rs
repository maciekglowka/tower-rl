use rogalik::{
    math::vectors::Vector2I,
    storage::{Entity, World}
};
use std::{
    any::{Any, TypeId},
    collections::VecDeque
};

use crate::components::{
    Blocker, Health, Name, Player, PlayerCharacter, Position, Projectile
};
use crate::events::ActionEvent;

pub struct PendingActions(pub VecDeque<Box<dyn Action>>);
pub struct ActorQueue(pub VecDeque<Entity>);

pub type ActionResult = Result<(), ()>;

pub trait Action {
    fn as_any(&self) -> &dyn Any;
    fn execute(&self, world: &mut World) -> ActionResult;
    fn event(&self) -> ActionEvent { ActionEvent::Other }
    fn score(&self, world: &World) -> i32 { 0 }
    fn type_id(&self) -> TypeId where Self: 'static {
        TypeId::of::<Self>()
    }
}

pub struct SelectedAction {
    pub action: Box<dyn Action>,
    pub card: Option<Entity>
}

pub struct Travel {
    pub entity: Entity,
    pub target: Vector2I
}
impl Action for Travel {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        let mut position = world.get_component_mut::<Position>(self.entity).ok_or(())?;
        position.0 = self.target;
        Ok(())
    }
    fn event(&self) -> ActionEvent {
        ActionEvent::Travel(self.entity, self.target)
    }
    fn score(&self, world: &World) -> i32 {
        let Some(player_position) = world.query::<PlayerCharacter>().with::<Position>()
            .iter()
            .map(|i| i.get::<Position>().unwrap().0)
            .next()
            else { return 0 };

        20 - self.target.manhattan(player_position)
    }
}

pub struct Shoot {
    pub source: Vector2I,
    pub dir: Vector2I,
    pub dist: u32,
    pub damage:  u32
}
impl Shoot {
    fn get_target(&self, world: &World) -> Vector2I {
        let blocker_positions = world.query::<Blocker>().with::<Position>()
        .iter()
        .map(|i| i.get::<Position>().unwrap().0)
        .collect::<Vec<_>>();

        // find target - eg. the max dist or first blocker on the way
        let mut target = self.source;
        for _ in 1..=self.dist {
            target += self.dir;
            if blocker_positions.contains(&target) { break };
        }
        target
    }
}
impl Action for Shoot {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        let target = self.get_target(world);
        let entity = world.spawn_entity();
        let _ = world.insert_component(entity, Projectile{
            damage: self.damage,
            target,
            source: self.source
        });
        Ok(())
    }
    fn score(&self, world: &World) -> i32 {
        let Some(player_position) = world.query::<PlayerCharacter>().with::<Position>()
            .iter()
            .map(|i| i.get::<Position>().unwrap().0)
            .next()
            else { return 0 };
        let target = self.get_target(world);
        if target == player_position {
            100
        } else {
            0
        }
    }
}

pub struct PlaceBouy {
    pub position: Vector2I,
    pub health:  u32
}
impl Action for PlaceBouy {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        let entity = world.spawn_entity();
        let _ = world.insert_component(entity, Name("Buoy".into()));
        let _ = world.insert_component(entity, Blocker);
        let _ = world.insert_component(entity, Position(self.position));
        let _ = world.insert_component(entity, Player);
        let _ = world.insert_component(entity, Health(self.health));
        Ok(())
    }
    fn score(&self, world: &World) -> i32 {
        // atm whatever ;)
        25
    }
}

pub struct Pause;
impl Action for Pause {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult { Ok(()) }
}

pub struct MeleeHit {
    // used in proximity effects
    // dummy struct used for eg. firing the event
    pub entity: Entity,
    pub target: Entity,
    pub value: u32
}
impl Action for MeleeHit {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        let position = world.get_component::<Position>(self.entity).ok_or(())?;
        let other = world.get_component::<Position>(self.target).ok_or(())?;
        if position.0.manhattan(other.0) != 1 { return Err(()) }
        let _ = world.get_component::<Health>(self.target).ok_or(())?;
        Ok(())
    }
    fn event(&self) -> ActionEvent {
        ActionEvent::Melee(self.entity, self.target, self.value)
    }
    fn score(&self, world: &World) -> i32 {
        // not used atm
        0
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
        Ok(())
    }
    // score is not implemented as it always should be a resulting action
}