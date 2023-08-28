use std::{
    any::TypeId,
    collections::VecDeque
};
use rogalik::math::vectors::Vector2F;
use rogalik::storage::{Entity, World, WorldEvent};

use hike_data::GameData;
use hike_game::{
    ActionEvent,
    Board,
    components::{Actor, Fixture, Name, Paralyzed, Position, Projectile, Tile}
};

use super::super::{GraphicsState, GraphicsBackend, SpriteColor, world_to_tile};
use super::utils::move_towards;
use crate::globals::{TILE_SIZE, ACTOR_Z, FIXTURE_Z, PROJECTILE_Z, TILE_Z, MOVEMENT_SPEED, PARALYZE_FADE};

pub struct SpriteRenderer {
    pub entity: Entity,
    pub v: Vector2F,
    pub path: VecDeque<Vector2F>,
    pub atlas_name: String,
    pub index: u32,
    pub z_index: u32,
    pub color: SpriteColor,
    pub fade: f32
}

pub fn handle_world_events(
    world: &World,
    state: &mut GraphicsState
) {
    let mut sprites_updated = false;
    for ev in state.ev_world.read().iter().flatten() {
        match ev {
            WorldEvent::ComponentRemoved(entity, type_id) => {
                match *type_id {
                    a if a == TypeId::of::<Paralyzed>() => {
                        fade_sprite(*entity, state, 1.)
                    },
                    a if a == TypeId::of::<Position>() || a == TypeId::of::<Projectile>() => {
                        state.sprites.retain(|a| a.entity != *entity);
                    },
                    _ => continue
                }
            },
            WorldEvent::ComponentSpawned(entity, type_id) => {
                match *type_id {
                    a if a == TypeId::of::<Paralyzed>() => {
                        fade_sprite(*entity, state, PARALYZE_FADE)
                    },
                    a if a == TypeId::of::<Position>() => {
                        state.sprites.push(
                            get_sprite_renderer(*entity, world)
                        );
                        sprites_updated = true;
                    },
                    a if a == TypeId::of::<Projectile>() => {
                        state.sprites.push(
                            get_projectile_renderer(*entity, world)
                        );
                        sprites_updated = true;
                    },
                    _ => continue
                }
            },
            _ => continue
        }
    }
    if sprites_updated {
        state.sort_sprites();
    }
}

pub fn handle_action_events(
    world: &World,
    state: &mut GraphicsState
) {
    for ev in state.ev_actions.read().iter().flatten() {
        match ev {
            ActionEvent::Melee(entity, target, _) => {
                if let Some(sprite) = get_entity_sprite(*entity, state) {
                    sprite.path.push_back(target.as_f32() * TILE_SIZE);
                    sprite.path.push_back(sprite.v);
                }
            },
            ActionEvent::Travel(entity, target) => {
                if let Some(sprite) = get_entity_sprite(*entity, state) {
                    sprite.path.push_back(target.as_f32() * TILE_SIZE);
                }
            },
            _ => continue
        }
    }
}

pub fn update_sprites(
    state: &mut GraphicsState
) -> bool {
    let mut ready = true;
    for sprite in state.sprites.iter_mut() {
        let Some(target) = sprite.path.get(0) else { continue };
        sprite.v = move_towards(sprite.v, *target, MOVEMENT_SPEED);
        if sprite.v == *target {
            sprite.path.pop_front();
        }
        if sprite.path.len() > 0 { ready = false }
    }
    ready
}

pub fn draw_sprites(world: &World, state: &GraphicsState, backend: &dyn GraphicsBackend) {
    let Some(board) = world.get_resource::<Board>() else { return };
    for sprite in state.sprites.iter() {
        let tile = world_to_tile(sprite.v);

        backend.draw_world_sprite(
            &sprite.atlas_name,
            sprite.index,
            sprite.v,
            Vector2F::new(TILE_SIZE, TILE_SIZE),
            sprite.color * sprite.fade
        );
    }
}

fn fade_sprite(entity: Entity, state: &mut GraphicsState, value: f32) {
    let Some(sprite) = get_entity_sprite(entity, state) else { return };
    sprite.fade = value;
}

fn get_sprite_renderer(
    entity: Entity,
    world: &World,
) -> SpriteRenderer {
    let mut z_index = 0;
    
    let game_data = world.get_resource::<GameData>().unwrap();
    let name = world.get_component::<Name>(entity).unwrap();
    let position = world.get_component::<Position>(entity).unwrap();

    if world.get_component::<Fixture>(entity).is_some() {
        z_index = FIXTURE_Z
    } else if world.get_component::<Tile>(entity).is_some() {
        z_index = TILE_Z
    } else if world.get_component::<Actor>(entity).is_some() {
        z_index = ACTOR_Z
    }

    let data = game_data.entities.get(&name.0).expect(
        &format!("No data found for {}", name.0)
    );

    SpriteRenderer { 
        entity: entity,
        v: position.0.as_f32() * TILE_SIZE,
        path: VecDeque::new(),
        atlas_name: data.sprite.atlas_name.clone(),
        index: data.sprite.index,
        z_index,
        color: data.sprite.color,
        fade: 1.
    }
}

fn get_projectile_renderer(
    entity: Entity,
    world: &World,
) -> SpriteRenderer {
    let projectile = world.get_component::<Projectile>(entity).unwrap();
    let mut path = VecDeque::new();
    path.push_back(projectile.target.as_f32() * TILE_SIZE);

    SpriteRenderer { 
        entity: entity,
        v: projectile.source.as_f32() * TILE_SIZE,
        path,
        atlas_name: "ascii".into(),
        index: 249,
        z_index: PROJECTILE_Z,
        color: SpriteColor(255, 255, 255, 255),
        fade: 1.
    }
}

fn get_entity_sprite(entity: Entity, state: &mut GraphicsState) -> Option<&mut SpriteRenderer> {
    state.sprites.iter_mut()
        .find(|a| a.entity == entity)
}