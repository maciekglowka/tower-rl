use rogalik::{
    engine::Color,
    storage::{Entity, World}
};

use hike_data::GameData;
use hike_game::{
    components::{
        Durability, Discoverable, Effects, Lunge, Name, Interactive,
        Offensive, Player, Push, Swing, Switch
    },
    structs::{Attack, AttackKind, Effect, EffectKind, InteractionKind},
    get_player_entity
};
use crate::game_ui::span::Span;

pub const ICON_HIT: u32 = 0;
pub const ICON_POISON: u32 = 1;
pub const ICON_DURABILITY: u32 = 2;
pub const ICON_STUN: u32 = 3;

pub const ICON_SWING: u32 = 8;
pub const ICON_LUNGE: u32 = 9;
pub const ICON_PUSH: u32 = 10;
pub const ICON_SWITCH: u32 = 11;

pub const ICON_GOLD: u32 = 16;
pub const ICON_HEAL: u32 = 17;
pub const ICON_IMMUNITY: u32 = 18;
pub const ICON_HEAL_POISON: u32 = 19;
pub const ICON_TELEPORT: u32 = 20;
pub const ICON_REGENERATION: u32 = 21;

pub const ICON_LEVEL: u32 = 24;
pub const ICON_WIN: u32 = 25;


fn needs_discovery(entity: Entity, world: &World) -> Option<(&str, Color)> {
    if world.get_component::<Discoverable>(entity).is_none() { return None };
    let name = world.get_component::<Name>(entity)?.0.clone();
    let player_entity = get_player_entity(world)?;
    let player = world.get_component::<Player>(player_entity)?;
    if player.discovered.contains(&name) { return None };
    let data = world.get_resource::<GameData>()?;
    data.discoverable_colors.get(&name).map(|&a| a)
}


pub fn get_item_name(entity: Entity, world: &World) -> Option<String> {
    if let Some(color) = needs_discovery(entity, world) {
        return Some(format!("Unidentified {} item", color.0));
    }
    Some(world.get_component::<Name>(entity)?.0.clone().replace("_", " "))
}


pub fn get_item_span<'a>(entity: Entity, world: &World) -> Span<'a> {
    if let Some(color) = needs_discovery(entity, world) {
        return Span::new()
            .with_text_color(color.1)
            .with_text_borrowed("?")
    }

    let mut span = Span::new()
        .with_text_color(Color(255, 255, 255, 255));

    let icons = get_entity_icons(entity, world);
    let mut it = icons.iter().peekable();
    while let Some((idx, val)) = it.next() {
        span = span.with_sprite("icons", *idx);
        if let Some(val) = val {
            span = span.with_text_owned(format!("{}", val));
        }
    }
    span
}

pub fn get_interactive_span<'a>(entity: Entity, world: &World) -> Span<'a> {
    let mut span = Span::new()
        .with_text_color(Color(255, 255, 255, 255));
    if let Some(interective) = world.get_component::<Interactive>(entity) {
        let (icon, text) = get_interaction_icon(&interective.kind);
        if let Some(text) = text {
            span = span.with_text_owned(text);
        }
        span = span.with_sprite("icons", icon);

        if let Some(cost) = interective.cost {
            span = span.with_spacer(crate::globals::UI_STATUS_TEXT_SIZE);
            span = span.with_text_owned(format!("-{}", cost));
            span = span.with_sprite("icons", ICON_GOLD);
        }
    }
    span
}

fn get_entity_icons(entity: Entity, world: &World) -> Vec<(u32, Option<u32>)> {
    // surely can be done better?
    let mut output = Vec::new();

    if let Some(effects) = world.get_component::<Effects>(entity) {
        output.extend(
            effects.effects.iter()
                .map(|e| get_effect_icon(e))
        );
    }
    if let Some(offensive) = world.get_component::<Offensive>(entity) {
        output.extend(
            offensive.attacks.iter()
                .map(|e| get_attack_icon(e))
        );
    }
    if let Some(durability) = world.get_component::<Durability>(entity) {
        output.push((ICON_DURABILITY, Some(durability.0)));
    }
    if let Some(_) = world.get_component::<Swing>(entity) {
        output.push((ICON_SWING, None));
    }
    if let Some(_) = world.get_component::<Lunge>(entity) {
        output.push((ICON_LUNGE, None));
    }
    if let Some(_) = world.get_component::<Push>(entity) {
        output.push((ICON_PUSH, None));
    }
    if let Some(_) = world.get_component::<Switch>(entity) {
        output.push((ICON_SWITCH, None));
    }
    output
}

fn get_attack_icon(attack: &Attack) -> (u32, Option<u32>) {
    let icon = match attack.kind {
        AttackKind::Hit => ICON_HIT,
        AttackKind::Poison => ICON_POISON,
        AttackKind::Stun => ICON_STUN,
    };
    (icon, Some(attack.value))
}

fn get_effect_icon(effect: &Effect) -> (u32, Option<u32>) {
    let icon = match effect.kind {
        EffectKind::Gold => ICON_GOLD,
        EffectKind::Heal => ICON_HEAL,
        EffectKind::HealPoison => ICON_HEAL_POISON,
        EffectKind::Immunity => ICON_IMMUNITY,
        EffectKind::Poison => ICON_POISON,
        EffectKind::Regenerate => ICON_REGENERATION,
        EffectKind::Teleport => ICON_TELEPORT,
        EffectKind::Win => ICON_WIN
    };
    (icon, if effect.value > 0 { Some(effect.value) } else { None })
}

fn get_interaction_icon(interaction: &InteractionKind) -> (u32, Option<String>) {
    match interaction {
        InteractionKind::Ascend => (ICON_LEVEL, None),
        InteractionKind::Repair(v) => (
            ICON_DURABILITY,
            Some(format!("+{}", v))
        ),
        InteractionKind::UpgradeHealth(v) => (
            ICON_HEAL,
            Some(format!("+{}max", v))
        )
    }
}