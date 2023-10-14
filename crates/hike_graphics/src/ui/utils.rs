use rogalik::{
    engine::Color,
    storage::{Entity, World}
};

use hike_data::GameData;
use hike_game::{
    components::{Durability, Discoverable, Effects, Lunge, Name, Offensive, Player, Push, Swing},
    structs::{Attack, AttackKind, Effect, EffectKind},
    get_player_entity
};
use crate::ui::span::Span;


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
        if it.peek().is_some() {
            // non-last element
            // span = span.with_spacer(0.1);
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
        output.push((2, Some(durability.0)));
    }
    if let Some(_) = world.get_component::<Swing>(entity) {
        output.push((8, None));
    }
    if let Some(_) = world.get_component::<Lunge>(entity) {
        output.push((9, None));
    }
    if let Some(_) = world.get_component::<Push>(entity) {
        output.push((10, None));
    }
    output
}

fn get_attack_icon(attack: &Attack) -> (u32, Option<u32>) {
    let icon = match attack.kind {
        AttackKind::Hit => 0,
        AttackKind::Poison => 1,
        AttackKind::Stun => 3,
    };
    (icon, Some(attack.value))
}

fn get_effect_icon(effect: &Effect) -> (u32, Option<u32>) {
    let icon = match effect.kind {
        EffectKind::Gold => 16,
        EffectKind::Heal => 17,
        EffectKind::Immunity => 18,
        EffectKind::Poison => 1
    };
    (icon, Some(effect.value))
}