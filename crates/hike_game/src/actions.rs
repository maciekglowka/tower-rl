use rogalik::{
    math::vectors::{Vector2i, ORTHO_DIRECTIONS, find_path},
    storage::{Entity, World}
};
use rand::prelude::*;
use std::{
    any::{Any, TypeId},
    collections::{HashSet, VecDeque}
};

use crate::board::Board;
use crate::components::{
    Actor, Discoverable, Durability, Stunned, Fixture, Health, Interactive, Loot, Defensive,
    Obstacle, Position, Player, Name, Poisoned, Effects, Projectile, Budding, Switch,
    Swing, Immune, Lunge, Push, Offensive, Ranged, Tile, Regeneration, Immaterial, Summoner
};
use crate::GameStats;
use crate::globals::MAX_COLLECTABLES;
use crate::events::GameEvent;
use crate::player::{get_player_entity, get_player_position};
use crate::structs::{
    AttackKind, InteractionKind, Attitude,
    get_attack_action, get_effect_action
};
use crate::utils::{visibility, get_entities_at_position, spawn_with_position, is_hostile};

pub struct PendingActions(pub VecDeque<Box<dyn Action>>);
pub struct ActorQueue(pub VecDeque<Entity>);

pub type ActionResult = Result<Vec<Box<dyn Action>>, ()>;

pub trait Action {
    fn as_any(&self) -> &dyn Any;
    fn execute(&self, world: &mut World) -> ActionResult;
    fn event(&self) -> GameEvent { GameEvent::Other }
    fn score(&self, world: &World) -> i32 { 0 }
    fn type_id(&self) -> TypeId where Self: 'static {
        TypeId::of::<Self>()
    }
}

pub fn get_action_at_dir(
    entity: Entity,
    world: &World,
    dir: Vector2i
) -> Option<Box<dyn Action>> {
    let position = world.get_component::<Position>(entity)?;
    let target = position.0 + dir;
    let board = world.get_resource::<Board>()?;
    if !board.tiles.contains_key(&target) { return None };

    let entities = get_entities_at_position(world, target);

    // attack is prioritized
    if world.get_component::<Offensive>(entity).is_some() {
        let attackable = entities.iter()
            .any(|&e| world.get_component::<Health>(e).is_some());
        if attackable {
            return Some(Box::new(AttackAction { entity, target }));
        }
    }

    // other actions
    if let Some(door) = entities.iter()
        .find(|&e| world.get_component::<Name>(*e).unwrap().0 == "Closed_Door") {
            return Some(Box::new(Replace { entity: *door, name: "Open_Door".to_string() }))
        }

    let has_obstacle = entities.iter().any(|&e| world.get_component::<Obstacle>(e).is_some());

    if has_obstacle && world.get_component::<Immaterial>(entity).is_none() { 
        return None
    }
    Some(Box::new(Walk { entity, target }))
}

fn is_shooting_range(
    source: Vector2i,
    target: Vector2i,
    distance: u32,
    world: &World
) -> bool {
    if target.manhattan(source) == 1 { return false }
    let d = (target - source).clamped();
    if d.x != 0 && d.y != 0 { return false } // only ORTHO

    for i in 1..=distance as i32 {
        let v = source + i * d;
        if v == target { return true };
        if get_entities_at_position(world, v).iter()
            .any(|&e| 
                world.get_component::<Obstacle>(e).is_some()
                && world.get_component::<Immaterial>(e).is_none()
            )
            { break }
    }
    false
}

fn get_ranged_action(
    entity: Entity,
    world: &World,
) -> Option<Box<dyn Action>> {
    // for now only the player can be attacked this way
    let ranged = world.get_component::<Ranged>(entity)?;
    let position = world.get_component::<Position>(entity)?;
    let player_v = get_player_position(world)?;

    if player_v.manhattan(position.0) > ranged.distance as i32 { return None }

    if !is_shooting_range(position.0, player_v, ranged.distance, world) { return None };
    Some(Box::new(Shoot {entity, target: player_v }))
}

pub fn get_npc_action(
    entity: Entity,
    world: &World
) -> Box<dyn Action> {
    let mut possible_actions = ORTHO_DIRECTIONS.iter()
       .filter_map(|dir| get_action_at_dir(entity, world, *dir))
       .collect::<Vec<_>>();

    if let Some(action) = get_ranged_action(entity, world) {
        possible_actions.push(action);
    }

    if let Some(summoner) = world.get_component::<Summoner>(entity) {
        if summoner.cooldown.current == 0 {
            possible_actions.push(Box::new(Summon { entity }))
        }
    }

    possible_actions.sort_by(|a, b| a.score(world).cmp(&b.score(world)));
    match possible_actions.pop() {
        Some(a) => a,
        _ => Box::new(Pause)
    }
}

fn get_empty_neighboring_tile(entity: Entity, world: &World) -> Option<Vector2i> {
    let position = world.get_component::<Position>(entity)?.0;
    let pool = ORTHO_DIRECTIONS.iter()
        .map(|&v| v + position)
        .filter(|v| !get_entities_at_position(world, *v)
            .iter()
            .any(|&e| world.get_component::<Obstacle>(e).is_some())
        );
    let mut rng = thread_rng();
    pool.choose(&mut rng)
}

pub struct Walk {
    pub entity: Entity,
    pub target: Vector2i
}
impl Action for Walk {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        let mut position = world.get_component_mut::<Position>(self.entity).ok_or(())?;
        position.0 = self.target;

        Ok(Vec::new())
    }
    fn event(&self) -> GameEvent {
        GameEvent::Travel(self.entity, true)
    }
    fn score(&self, world: &World) -> i32 {
        if get_entities_at_position(world, self.target)
            .iter()
            .any(|&e| world.get_component::<Offensive>(e).is_some() &&
                world.get_component::<Fixture>(e).is_some()
            ) {
                return -10
            };
        let mut rng = thread_rng();
        let r = rng.gen_range(0..4);
        let Some(position) = world.get_component::<Position>(self.entity) else { return r };
        let Some(actor) = world.get_component::<Actor>(self.entity) else { return r };

        if let Some(player_v) = get_player_position(world) {
            if let Attitude::Panic = actor.attitude {
                return player_v.manhattan(self.target);
            }

            if let Some(ranged) = world.get_component::<Ranged>(self.entity) {
                if is_shooting_range(self.target, player_v, ranged.distance, world) {
                    return 50;
                }
                if player_v.manhattan(self.target) == 1 {
                    return -5;
                }
            }
        }

        let Some(target) = actor.target else { return r };
        let Some(board) = world.get_resource::<Board>() else { return r };

        let blockers = match world.get_component::<Immaterial>(self.entity) {
            Some(_) => HashSet::new(),
            None => world.query::<Obstacle>().with::<Position>().build().iter::<Position>()
                .map(|p| p.0)
                .collect::<HashSet<_>>()
        };

        let Some(path) = find_path(
            position.0,
            target,
            &board.tiles.keys().map(|&v| v).collect::<HashSet<_>>(),
            &blockers
        ) else { return r };

        if path.contains(&self.target) { 20 } else { r }
    }
}

pub struct AttackAction {
    // base attack action used to dispatch specific attack types
    pub entity: Entity,
    pub target: Vector2i
}
impl AttackAction {
    fn get_offending_entity(&self, world: &World) -> Entity {
        if let Some(player) = world.get_component::<Player>(self.entity) {
            if let Some(weapon) = player.weapons[player.active_weapon] {
                return weapon
            }
        }
        self.entity
    }
    fn get_attack_targets(&self, entity: Entity, world: &World) -> HashSet<Vector2i> {
        let mut output = HashSet::from_iter([self.target]);
        let Some(position) = world.get_component::<Position>(self.entity) else { return output };
        let dir = self.target - position.0;

        if world.get_component::<Swing>(entity).is_some() {
            let vs = ORTHO_DIRECTIONS.iter()
                .filter(|d| **d != dir * -1)
                .map(|d| position.0 + *d)
                .collect::<Vec<_>>();
            output.extend(vs);
        }
        if world.get_component::<Lunge>(entity).is_some() {
            output.insert(position.0 + dir * 2);
        }

        output
    }
    fn get_attack_actions(&self, entity: Entity, world: &World, target: Vector2i) -> Vec<Box<dyn Action>> {
        let Some(offensive) = world.get_component::<Offensive>(entity) else { return Vec::new() };
        offensive.attacks.iter()
            .map(|a| get_attack_action(a, target))
            .collect()
    }
    fn get_attack_side_effects(&self, entity: Entity, world: &World, target: Vector2i)  -> Vec<Box<dyn Action>> {
        let mut actions: Vec<Box<dyn Action>> = Vec::new();

        if world.get_component::<Push>(entity).is_some() {
            if let Some(position) = world.get_component::<Position>(self.entity) {
                actions.push(Box::new(
                    PushAction { source: position.0, target, distance: 2 }
                ));
            }
        }
        if world.get_component::<Switch>(entity).is_some() {
            actions.push(Box::new(
                SwitchAction { entity: self.entity, target }
            ));
        }
        actions
    }
}
impl Action for AttackAction {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        let offending_entity = self.get_offending_entity(world);
        let mut actions = self.get_attack_targets(offending_entity, world).iter()
            .map(|&v| 
                self.get_attack_actions(offending_entity, world, v)
                .into_iter()
                .chain(self.get_attack_side_effects(offending_entity, world, v))
            )
            .flatten()
            .collect::<Vec<_>>();

        if actions.len() > 0 { actions.push(Box::new(
            Defend { attacker: self.entity, target: self.target }
        ))};
        
        if world.get_component::<Durability>(offending_entity).is_some() {
            actions.push(Box::new(
                TakeDurability { entity: offending_entity, owner: self.entity }
            ));
        }
        Ok(actions)
    }
    fn event(&self) -> GameEvent {
        GameEvent::Attack(self.entity, self.target)
    }
    fn score(&self, world: &World) -> i32 {
        if !is_hostile(self.entity, world) { return -50 };
        if get_entities_at_position(world, self.target).iter().any(
            |e| world.get_component::<Player>(*e).is_some()
        ) {
            200
        } else {
            -50
        }
    }
}


pub struct Defend {
    // base attack action used to dispatch specific attack types
    pub attacker: Entity,
    pub target: Vector2i
}
impl Defend {
    fn get_defensive_actions(&self, entity: Entity, world: &World, target: Vector2i) -> Vec<Box<dyn Action>> {
        let Some(defensive) = world.get_component::<Defensive>(entity) else { return Vec::new() };
        defensive.attacks.iter()
            .map(|a| get_attack_action(a, target))
            .collect()
    }
}
impl Action for Defend {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        let offending_position = world.get_component::<Position>(self.attacker).ok_or(())?.0;
        let actions = get_entities_at_position(world, self.target)
            .iter()
            .map(|&e| self.get_defensive_actions(e, world, offending_position))
            .flatten()
            .collect::<Vec<_>>();
        Ok(actions)
    }
}


pub struct HitAction {
    // pub entity: Entity,
    pub target: Vector2i,
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
    pub target: Vector2i,
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
    // pub entity: Entity,
    pub target: Vector2i,
    pub value: u32
}
impl Action for PoisonAction {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        let mut actions = Vec::new();
        for entity in get_entities_at_position(world, self.target) {
            if world.get_component::<Health>(entity).is_none() { continue }
            actions.push(Box::new(
                ApplyPoison { entity, value: self.value }
            ) as Box<dyn Action>);
        }
        Ok(actions)
    }
}

pub struct PushAction {
    pub source: Vector2i,
    pub target: Vector2i,
    pub distance: u32
}
impl PushAction {
    fn get_furthest_walkable(&self, world: &World, dir: Vector2i) -> Option<Vector2i> {
        let obstacle_positions = world.query::<Obstacle>()
            .with::<Position>()
            .build()
            .iter::<Position>()
            .map(|a| a.0)
            .collect::<Vec<_>>();
        let mut result = self.target;
        for _ in 1..=self.distance {
            let t = result + dir;
            if obstacle_positions.contains(&t) { break };
            result = t;
        }
        if result == self.target { None } else { Some(result) }
    }
}
impl Action for PushAction {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        if self.source.manhattan(self.target) > 1 {
            return Err(())
        }
        let dir = self.target - self.source;
        let actions = if let Some(tile) = self.get_furthest_walkable(world, dir) {
            get_entities_at_position(world, self.target).iter()
                .filter(|&e| world.get_component::<Actor>(*e).is_some())
                .map(|&e| Box::new(Walk { entity: e, target: tile }) as Box<dyn Action>)
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        Ok(actions)
    }
}

pub struct SwitchAction {
    pub entity: Entity,
    pub target: Vector2i
}
impl Action for SwitchAction {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        let source = world.get_component::<Position>(self.entity).ok_or(())?.0;
        let mut actions = {
            get_entities_at_position(world, self.target).iter()
                .filter(|&e| world.get_component::<Actor>(*e).is_some())
                .map(|&e| Box::new(Walk { entity: e, target: source }) as Box<dyn Action>)
                .collect::<Vec<_>>()
        };
        if actions.len() > 0 {
            actions.push(Box::new(Walk { entity: self.entity, target: self.target }));
            // stun switched actor so he cannot strike back
            actions.push(Box::new(StunAction { target: source, value: 1 }));
            if let Some(mut actor) = world.get_component_mut::<Actor>(self.entity) {
                // this avoids an infinite switch loop
                actor.attitude = Attitude::Neutral;
            }
        }
        Ok(actions)
    }
}

pub struct Bump {
    pub entity: Entity,
    pub target: Vector2i
}
impl Action for Bump {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        Ok(Vec::new())
    }
    fn event(&self) -> GameEvent {
        GameEvent::Bump(self.entity, self.target)
    }
}

pub struct TakeDurability {
    pub entity: Entity,
    pub owner: Entity
}
impl Action for TakeDurability {
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

pub struct WieldWeapon {
    pub entity: Entity
}
impl Action for WieldWeapon {
    fn as_any(&self) -> &dyn Any { self }
    fn event(&self) -> GameEvent {
        GameEvent::PickItem
    }
    fn execute(&self, world: &mut World) -> ActionResult {
        let mut replaced = None;
        if let Some(mut player) = world.query::<Player>().build().single_mut::<Player>() {
            let index = if let Some(free) = player.weapons.iter()
                .enumerate()
                .filter(|a| a.1.is_none())
                .next() {
                    free.0
                } else {
                    player.active_weapon
                };
    
            replaced = player.weapons[index];
            player.weapons[index] = Some(self.entity);
        };

        world.remove_component::<Position>(self.entity);
        if let Some(replaced) = replaced {
            world.despawn_entity(replaced);
        }

        Ok(Vec::new())
    }
    // no score - npcs do not pick
}

pub struct PickCollectable {
    pub entity: Entity
}
impl Action for PickCollectable {
    fn as_any(&self) -> &dyn Any { self }
    fn event(&self) -> GameEvent {
        GameEvent::PickItem
    }
    fn execute(&self, world: &mut World) -> ActionResult {
        if let Some(mut player) = world.query::<Player>().build().single_mut::<Player>() {
            if player.collectables.len() >= MAX_COLLECTABLES {
                return Err(())
            }
        
            player.collectables.push(self.entity);
        };
        world.remove_component::<Position>(self.entity);

        Ok(Vec::new())
    }
    // no score - npcs do not pick
}

pub struct UseCollectable {
    pub entity: Entity
}
impl Action for UseCollectable {
    fn as_any(&self) -> &dyn Any { self }
    fn event(&self) -> GameEvent {
        GameEvent::UseCollectable
    }
    fn execute(&self, world: &mut World) -> ActionResult {
        let mut actions: Vec<Box<dyn Action>> = Vec::new();
        let player_query = world.query::<Player>().build();
        let Some(player_entity) = player_query.single_entity() else { return Err(()) };
        if let Some(effects) = world.get_component::<Effects>(self.entity) {
            actions.extend(
                effects.effects.iter()
                    .map(|e| get_effect_action(e, player_entity))
            );
        }
        if let Some(mut player) = player_query.single_mut::<Player>() {
            player.collectables.retain(|&e| e != self.entity);

            if world.get_component::<Discoverable>(self.entity).is_some() {
                if let Some(name) = world.get_component::<Name>(self.entity) {
                    player.discovered.insert(name.0.clone());
                }
            }
        };
        world.despawn_entity(self.entity);

        Ok(actions)
    }
}

pub struct UseInstant {
    pub entity: Entity
}
impl Action for UseInstant {
    fn as_any(&self) -> &dyn Any { self }
    fn event(&self) -> GameEvent {
        GameEvent::PickItem
    }
    fn execute(&self, world: &mut World) -> ActionResult {
        let mut actions: Vec<Box<dyn Action>> = Vec::new();
        let player_entity = world.query::<Player>().build().single_entity().ok_or(())?;
        if let Some(effects) = world.get_component::<Effects>(self.entity) {
            actions.extend(
                effects.effects.iter()
                    .map(|e| get_effect_action(e, player_entity))
            );
        }
        world.despawn_entity(self.entity);
        Ok(actions)
    }
}

pub struct Damage {
    pub entity: Entity,
    pub value: u32
}
impl Action for Damage {
    fn as_any(&self) -> &dyn Any { self }
    fn event(&self) -> GameEvent {
        GameEvent::Health(self.entity, -(self.value as i32))
    }
    fn execute(&self, world: &mut World) -> ActionResult {
        if world.get_component::<Immune>(self.entity).is_some() {
            return Err(())
        }
        let mut health = world.get_component_mut::<Health>(self.entity).ok_or(())?;
        health.0.current = health.0.current.saturating_sub(self.value);
        if world.get_component::<Budding>(self.entity).is_some() {
            return Ok(vec![Box::new(BuddingActon { entity: self.entity })])
        }
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
    fn event(&self) -> GameEvent {
        GameEvent::Health(self.entity, self.value as i32)
    }
    fn execute(&self, world: &mut World) -> ActionResult {
        let mut health = world.get_component_mut::<Health>(self.entity).ok_or(())?;
        health.0.current = health.0.max.min(health.0.current + self.value);
        Ok(Vec::new())
    }
    // score is not implemented as it always should be a resulting action
}

pub struct ApplyPoison {
    pub entity: Entity,
    pub value: u32
}
impl Action for ApplyPoison {
    fn as_any(&self) -> &dyn Any { self }
    fn event(&self) -> GameEvent {
        GameEvent::Poison(self.entity, self.value as i32)
    }
    fn execute(&self, world: &mut World) -> ActionResult {
        if let Some(mut poisoned) = world.get_component_mut::<Poisoned>(self.entity) {
            poisoned.0 += self.value;
            return Ok(Vec::new())
        };

        let _ = world.insert_component(self.entity, Poisoned(self.value));
        Ok(Vec::new())
    }
    // score is not implemented as it always should be a resulting action
}


pub struct HealPoison {
    pub entity: Entity,
}
impl Action for HealPoison {
    fn as_any(&self) -> &dyn Any { self }
    fn event(&self) -> GameEvent {
        GameEvent::HealPoison(self.entity)
    }
    fn execute(&self, world: &mut World) -> ActionResult {
        if world.get_component_mut::<Poisoned>(self.entity).is_none() {
            return Ok(Vec::new())
        };
        let _ = world.remove_component::<Poisoned>(self.entity);
        Ok(Vec::new())
    }
    // score is not implemented as it always should be a resulting action
}

pub struct GiveImmunity {
    pub entity: Entity,
    pub value: u32
}
impl Action for GiveImmunity {
    fn as_any(&self) -> &dyn Any { self }
    fn event(&self) -> GameEvent {
        GameEvent::Immunity(self.entity)
    }
    fn execute(&self, world: &mut World) -> ActionResult {
        if let Some(mut immune)  = world.get_component_mut::<Immune>(self.entity) {
            immune.0 += self.value;
            return Ok(Vec::new());
        }

        let _ = world.insert_component(self.entity, Immune(self.value));
        Ok(Vec::new())
    }
}

pub struct GiveRegeneration {
    pub entity: Entity,
    pub value: u32
}
impl Action for GiveRegeneration {
    fn as_any(&self) -> &dyn Any { self }
    fn event(&self) -> GameEvent {
        GameEvent::Regeneration(self.entity)
    }
    fn execute(&self, world: &mut World) -> ActionResult {
        if let Some(mut regeneration) = world.get_component_mut::<Regeneration>(self.entity) {
            regeneration.0 += self.value;
            return Ok(Vec::new())
        };

        let _ = world.insert_component(self.entity, Regeneration(self.value));
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
    fn event(&self) -> GameEvent {
        GameEvent::Upgrade
    }
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
    fn event(&self) -> GameEvent {
        GameEvent::Upgrade
    }
    fn execute(&self, world: &mut World) -> ActionResult {
        let mut health = world.get_component_mut::<Health>(self.entity).ok_or(())?;
        health.0.max += self.value;
        health.0.current += self.value;
        Ok(Vec::new())
    }
    // score is not implemented as it always should be a resulting action
}

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
        let query = world.query::<Player>().build();
        let player = query.single::<Player>().ok_or(())?;

        if let Some(cost) = interactive.cost {
            if cost > player.gold {
                    return Err(());
                }
                res.push(Box::new(Pay { value: cost }));
        }

        let action: Box<dyn Action> = match interactive.kind {
            InteractionKind::Ascend => Box::new(Ascend),
            InteractionKind::Repair(value) => {
                Box::new(Repair { entity: player.weapons[player.active_weapon].ok_or(())?, value } )
            },
            // InteractionKind::UpgradeOffensive(value) => {
            //     let idx = world.query::<Player>().iter().next().ok_or(())?
            //         .get::<Player>().unwrap().active_item;
            //     let item = world.query::<Player>().iter().next().ok_or(())?
            //         .get::<Player>().unwrap().items[idx].ok_or(())?;
            //     Box::new(UpgradeOffensive { entity: item, value } )
            // },
            InteractionKind::UpgradeHealth(value) => {
                let player_entity = world.query::<Player>().build().single_entity().ok_or(())?;
                Box::new(UpgradeHealth { entity: player_entity, value } )
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

pub struct BuddingActon {
    pub entity: Entity
}
impl Action for BuddingActon {
    fn as_any(&self) -> &dyn Any { self }
    fn event(&self) -> GameEvent {
        GameEvent::Spawn
    }
    fn execute(&self, world: &mut World) -> ActionResult {
        let health = world.get_component::<Health>(self.entity).ok_or(())?.0.current;

        let target = get_empty_neighboring_tile(self.entity, world).ok_or(())?;
        let name = world.get_component::<Name>(self.entity).ok_or(())?.0.to_string();

        let spawned = spawn_with_position(world, &name, target).ok_or(())?;

        if let Some(mut h) = world.get_component_mut::<Health>(spawned) {
            h.0.current = health;
        }
        Ok(Vec::new())
    }
}

pub struct Summon {
    pub entity: Entity
}
impl Action for Summon {
    fn as_any(&self) -> &dyn Any { self }
    fn event(&self) -> GameEvent {
        GameEvent::Spawn
    }
    fn execute(&self, world: &mut World) -> ActionResult {
        let target = get_empty_neighboring_tile(self.entity, world).ok_or(())?;
        let mut summoner = world.get_component_mut::<Summoner>(self.entity).ok_or(())?;
        if summoner.cooldown.current > 0 {
            return Err(())
        }
        summoner.cooldown.current = summoner.cooldown.max;
        let name = summoner.creature.clone();
        drop(summoner);
        let _ = spawn_with_position(world, &name, target).ok_or(())?;

        Ok(Vec::new())
    }
    fn score(&self, world: &World) -> i32 {
        if !is_hostile(self.entity, world) { return -50 };
        200
    }
}

pub struct Ascend;
impl Action for Ascend {
    fn as_any(&self) -> &dyn Any { self }
    fn event(&self) -> GameEvent {
        GameEvent::Ascend
    }
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

pub struct Teleport {
    pub entity: Entity
}
impl Action for Teleport {
    fn as_any(&self) -> &dyn Any { self }
    fn event(&self) -> GameEvent {
        GameEvent::Travel(self.entity, false)
    }
    fn execute(&self, world: &mut World) -> ActionResult {
        let position = if let Some(position) = world.get_component::<Position>(self.entity) {
            position.0
        } else {
            return Err(());
        };

        let query = world.query::<Tile>().with::<Position>().build();
        let pool = query.iter::<Position>()
            .filter(|p| !get_entities_at_position(world, p.0)
                .iter()
                .any(|&e| world.get_component::<Obstacle>(e).is_some())
            )
            .map(|p| {
                let d = position.manhattan(p.0);
                (d, p.0)
            })
            .collect::<Vec<_>>();

        let mut rng = thread_rng();
        let target = pool.choose_weighted(&mut rng, |a| a.0).unwrap();

        let mut position = world.get_component_mut::<Position>(self.entity).ok_or(())?;
        position.0 = target.1;

        Ok(Vec::new())
    }
}

pub struct Shoot {
    entity: Entity,
    target: Vector2i
}
impl Action for Shoot {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> ActionResult {
        let attacks = world.get_component::<Ranged>(self.entity)
            .ok_or(())?
            .attacks.clone();
        let source = world.get_component::<Position>(self.entity)
            .ok_or(())?
            .0;

        let projectile_entity = world.spawn_entity();
        let _ = world.insert_component(
            projectile_entity,
            Projectile {
                attacks,
                source,
                target: self.target
            }
        );
        Ok(Vec::new())
    }
    fn score(&self, world: &World) -> i32 {
        if !is_hostile(self.entity, world) { return -50 };
        if get_entities_at_position(world, self.target).iter().any(
            |e| world.get_component::<Player>(*e).is_some()
        ) {
            200
        } else {
            -50
        }
    }
}

pub struct WinAction;
impl Action for WinAction {
    fn as_any(&self) -> &dyn Any { self }
    fn event(&self) -> GameEvent {
        GameEvent::Win
    }
    fn execute(&self, world: &mut World) -> ActionResult {
        if let Some(mut stats) = world.get_resource_mut::<GameStats>() {
            stats.win = true;
        }
        Ok(Vec::new())
    }
}