use rogalik::{
    math::vectors::Vector2i,
    storage::Entity
};
use serde::Deserialize;

use crate::actions::{
    Action, Heal, PickGold, GiveImmunity,
    HitAction, StunAction, PoisonAction
};
use crate::utils::deserialize_random_u32;

#[derive(Deserialize)]
pub enum AttackKind {
    Hit,
    Poison,
    Stun
}

#[derive(Deserialize)]
pub struct Attack {
    pub kind: AttackKind,
    #[serde(deserialize_with="deserialize_random_u32")]
    pub value: u32
}

#[derive(Deserialize)]
pub enum EffectKind {
    Gold,
    Heal,
    Immunity,
}

#[derive(Deserialize)]
pub struct Effect {
    pub kind: EffectKind,
    #[serde(deserialize_with="deserialize_random_u32")]
    pub value: u32
}

pub struct ValueMax {
    pub current: u32,
    pub max: u32
}

#[derive(Deserialize, PartialEq)]
pub enum InteractionKind {
    Ascend,
    Repair(#[serde(deserialize_with="deserialize_random_u32")] u32),
    UpgradeHealth(#[serde(deserialize_with="deserialize_random_u32")] u32),
}
impl InteractionKind {
    pub fn to_str(&self) -> String {
        match self {
            InteractionKind::Ascend => "Ascend".to_string(),
            InteractionKind::Repair(v) => format!("Repair({})", v),
            InteractionKind::UpgradeHealth(v) => format!("Incr. HP({})", v),
        }
    }
}

pub fn get_effect_action(
    effect: &Effect,
    entity: Entity,
) -> Box<dyn Action> {
    match effect.kind {
        EffectKind::Gold => Box::new(
            PickGold { value: effect.value }
        ),
        EffectKind::Heal => Box::new(
            Heal { entity, value: effect.value }
        ),
        EffectKind::Immunity => Box::new(
            GiveImmunity { entity, value: effect.value }
        )
    }
}

pub fn get_attack_action(
    attack: &Attack,
    entity: Entity,
    target: Vector2i
) -> Box<dyn Action> {
    match attack.kind {
        AttackKind::Hit => Box::new(
            HitAction { entity, target, value: attack.value }
        ),
        AttackKind::Poison => Box::new(
            PoisonAction { entity, target, value: attack.value }
        ),
        AttackKind::Stun => Box::new(
            StunAction { entity, target, value: attack.value }
        ),
    }
}
