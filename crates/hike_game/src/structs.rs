use rogalik::{
    math::vectors::Vector2i,
    storage::Entity
};
use serde::{Serialize, Deserialize, Deserializer};
use serde::de::Visitor;

use crate::actions::{
    Action, Heal, PickGold, GiveImmunity, GiveRegeneration,
    HitAction, StunAction, PoisonAction, ApplyPoison, HealPoison, Teleport, WinAction
};
use crate::utils::deserialize_random_u32;

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq)]
pub enum Attitude {
    #[default]
    Neutral,
    Aware,
    Hostile,
    Panic
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum AttackKind {
    Hit,
    Poison,
    Stun
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Attack {
    pub kind: AttackKind,
    #[serde(deserialize_with="deserialize_random_u32")]
    pub value: u32
}

#[derive(Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
pub struct Effect {
    pub kind: EffectKind,
    #[serde(deserialize_with="deserialize_random_u32")]
    pub value: u32
}

#[derive(Serialize)]
pub struct ValueMax {
    pub current: u32,
    pub max: u32
}
impl<'de> Deserialize<'de> for ValueMax {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        if !d.is_human_readable() {
            return d.deserialize_struct(
                "ValueMax",
                &["current", "max"],
                ValueMaxVisitor
            )
        }
        // if let Ok(s) = &deserializer.deserialize_struct(
        //     "ValueMax",
        //     &["current", "max"],
        //     ValueMaxVisitor
        // ) {2
        //     return Ok(s);
        // }
        // let n = u32::deserialize(deserializer)?;
        // Ok(ValueMax { current: n, max: n })
        d.deserialize_any(ValueMaxVisitor)
        // deserializer.deserialize_u32(ValueMaxVisitor)
        // deserializer.deserialize_struct(
        //     "ValueMax",
        //     &["current", "max"],
        //     ValueMaxVisitor
        // )
    }
}
struct ValueMaxVisitor;
impl<'de> Visitor<'de> for ValueMaxVisitor {
    type Value = ValueMax;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("struct ValueMax")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where E: serde::de::Error
    {
        Ok(ValueMax { current: v as u32, max: v as u32 })
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where A: serde::de::SeqAccess<'de>
    {
        println!("SEQ");
        let current = seq.next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
        if let Some(max) = seq.next_element()? {
            return Ok(ValueMax { current, max })
        }
        Ok(ValueMax { current, max: current })
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where A: serde::de::MapAccess<'de>
    {
        println!("MAP");
        let mut current = None;
        let mut max = None;

        while let Some(k) = map.next_key()? {
            match k {
                "current" | "Current" | "CURRENT" => current = Some(map.next_value()?),
                "max" | "Max" | "MAX" => max = Some(map.next_value()?),
                _ => ()
            }
        }
        Ok(ValueMax{ 
            current: current.ok_or_else(|| serde::de::Error::missing_field("current"))?,
            max: max.ok_or_else(|| serde::de::Error::missing_field("max"))?
        })
    }
}

#[derive(Deserialize, Serialize, PartialEq)]
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
