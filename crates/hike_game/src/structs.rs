use rogalik::{
    math::vectors::Vector2i,
    storage::Entity
};
use serde::Deserialize;

use crate::actions::{
    Action, Heal, PickGold, GiveImmunity, GiveRegeneration,
    HitAction, StunAction, PoisonAction, ApplyPoison, HealPoison, Teleport, WinAction
};
use crate::utils::deserialize_random_u32;

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq)]
pub enum Attitude {
    #[default]
    Neutral,
    Aware,
    Hostile,
    Panic
}

#[derive(Clone, Copy, Deserialize)]
pub enum AttackKind {
    Hit,
    Poison,
    Stun
}

#[derive(Clone, Copy, Deserialize)]
pub struct Attack {
    pub kind: AttackKind,
    #[serde(deserialize_with="deserialize_random_u32")]
    pub value: u32
}

#[derive(Deserialize)]
pub enum EffectKind {
    Gold,
    Heal,
    HealPoison,
    Immunity,
    Poison,
    Regenerate,
    Teleport,
    Win
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
        EffectKind::HealPoison => Box::new(
            HealPoison { entity }
        ),
        EffectKind::Immunity => Box::new(
            GiveImmunity { entity, value: effect.value }
        ),
        EffectKind::Poison => Box::new(
            ApplyPoison { entity, value: effect.value }
        ),
        EffectKind::Regenerate => Box::new(
            GiveRegeneration { entity, value: effect.value }
        ),
        EffectKind::Teleport => Box::new(
            Teleport { entity }
        ),
        EffectKind::Win => Box::new(WinAction)
    }
}

pub fn get_attack_action(
    attack: &Attack,
    // entity: Entity,
    target: Vector2i
) -> Box<dyn Action> {
    match attack.kind {
        AttackKind::Hit => Box::new(
            HitAction { target, value: attack.value }
        ),
        AttackKind::Poison => Box::new(
            PoisonAction { target, value: attack.value }
        ),
        AttackKind::Stun => Box::new(
            StunAction { target, value: attack.value }
        ),
    }
}
