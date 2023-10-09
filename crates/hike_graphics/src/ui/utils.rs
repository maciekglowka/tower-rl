use rogalik::{
    engine::Color,
    storage::{Entity, World}
};

use hike_game::{
    components::{Durability, Effects, Lunge, Offensive, Swing},
    structs::{Attack, AttackKind, Effect, EffectKind},
};
use crate::ui::span::Span;


pub fn get_item_span<'a>(entity: Entity, world: &World) -> Span<'a> {
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
    };
    (icon, Some(effect.value))
}