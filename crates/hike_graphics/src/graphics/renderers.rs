use std::{
    any::TypeId,
    collections::{HashSet, VecDeque}
};
use rogalik::engine::{Color, GraphicsContext, Params2d};
use rogalik::math::vectors::{Vector2f, Vector2i};
use rogalik::storage::{Entity, World, WorldEvent};

use hike_data::GameData;
use hike_game::{
    GameEvent,
    Board,
    components::{Actor, Discoverable, Fixture, Item, Name, Stunned, Position, Projectile, Tile},
    globals::BOARD_SIZE,
    get_entities_at_position
};

use super::super::{
    GraphicsState, Context_, world_to_tile, tile_to_world
};
use super::utils::move_towards;
use crate::globals::{
    TILE_SIZE, ACTOR_Z, FIXTURE_Z, ITEM_Z, PROJECTILE_Z, TILE_Z, FOG_Z,
    MOVEMENT_SPEED, INACTIVE_FADE, FADE_SPEED, BACKGROUND_COLOR
};

#[derive(Debug, PartialEq)]
pub enum SpriteState {
    Added,
    Existing,
    Removed
}

#[derive(Debug)]
pub struct SpriteRenderer {
    pub entity: Entity,
    pub v: Vector2f,
    pub path: VecDeque<Vector2f>,
    pub atlas_name: String,
    pub index: u32,
    pub z_index: i32,
    pub color: Color,
    pub fade: f32,
    pub state: SpriteState,
    pub frame: u32,
    pub frame_count: u32
}

pub fn handle_world_events(
    world: &World,
    state: &mut GraphicsState
) {
    // let mut sprites_updated = false;
    for ev in state.ev_world.read().iter().flatten() {
        match ev {
            WorldEvent::ComponentRemoved(entity, type_id) => {
                match *type_id {
                    a if a == TypeId::of::<Stunned>() => {
                        fade_sprite(*entity, state, 1.)
                    },
                    a if a == TypeId::of::<Position>() => {
                        if let Some(sprite) = get_entity_sprite_mut(*entity, state) {
                            sprite.state = SpriteState::Removed;
                        }
                    },
                    a if a == TypeId::of::<Projectile>() => {
                        state.sprites.retain(|a| a.entity != *entity);
                    },
                    _ => continue
                }
            },
            WorldEvent::ComponentSpawned(entity, type_id) => {
                match *type_id {
                    a if a == TypeId::of::<Position>() => {
                        state.sprites.push(
                            get_sprite_renderer(*entity, world)
                        );
                        // sprites_updated = true;
                    },
                    a if a == TypeId::of::<Projectile>() => {
                        state.sprites.push(
                            get_projectile_renderer(*entity, world)
                        );
                        // sprites_updated = true;
                    },
                    a if a == TypeId::of::<Stunned>() => {
                        fade_sprite(*entity, state, INACTIVE_FADE)
                    },
                    _ => continue
                }
            },
            _ => continue
        }
    }
    // if sprites_updated {
    //     state.sort_sprites();
    // }
}

pub fn handle_action_events(
    world: &World,
    state: &mut GraphicsState
) {
    for ev in state.ev_game.read().iter().flatten() {
        // temp bubble -> create a common handler?
        crate::game_ui::bubbles::handle_game_event(ev, world, state);
        match ev {
            GameEvent::BoardReady => update_wall_sprites(world, state),
            GameEvent::Attack(entity, target) | GameEvent::Bump(entity, target) => {
                if let Some(sprite) = get_entity_sprite_mut(*entity, state) {
                    sprite.path.push_back((sprite.v + tile_to_world(*target)) * 0.5);
                    sprite.path.push_back(sprite.v);
                }
            },
            GameEvent::Travel(entity, is_animated) => {
                if let Some(sprite) = get_entity_sprite_mut(*entity, state) {
                    if let Some(target) = world.get_component::<Position>(*entity) {
                        if *is_animated {
                            sprite.path.push_back(tile_to_world(target.0));
                        } else {
                            sprite.v = tile_to_world(target.0);
                        }
                    }
                }
            },
            _ => continue
        }
    }
}

pub fn update_wall_sprites(world: &World, state: &mut GraphicsState) {
    let Some(board) = world.get_resource::<Board>() else { return };
    for (v, _) in board.tiles.iter() {
        let Some(wall) = get_wall_at(*v, world) else { continue };
        let mut offset = 0;
        if get_wall_at(Vector2i::new(v.x, v.y - 1), world).is_none() && v.y != -1 as i32 {
            offset += 1;
        }
        if get_wall_at(Vector2i::new(v.x, v.y + 1), world).is_none() && v.y >= 0 {
            offset += 2;
        }
        if let Some(sprite) = get_entity_sprite_mut(wall, state) {
            sprite.index += offset;
        }
    }
}

fn get_wall_at(v: Vector2i, world: &World) -> Option<Entity> {
    get_entities_at_position(world, v).iter()
        .filter_map(|&e| match world.get_component::<Name>(e) {
                Some(a) if a.0 == "Wall" => Some(e),
                _ => None
            }
        )
        .next()
}

pub fn update_sprites(world: &World, state: &mut GraphicsState, tick: bool, delta: f32) -> bool {
    update_added_sprites(state, delta);
    update_removed_sprites(state, delta);
    if tick { update_sprite_frames(state) };
    update_sprite_positions(world, state, delta)
}

fn update_sprite_frames(state: &mut GraphicsState) {
    for sprite in state.sprites.iter_mut().filter(|s| s.frame_count > 1) {
        sprite.frame = (sprite.frame + 1) % sprite.frame_count
    }
}

fn update_sprite_positions(world: &World, state: &mut GraphicsState, delta: f32) -> bool {
    let Some(board) = world.get_resource::<Board>() else { return true };
    let mut ready = true;
    for sprite in state.sprites.iter_mut() {
        let Some(target) = sprite.path.get(0) else { continue };

        let target_tile = world_to_tile(*target);
        let source_tile = world_to_tile(sprite.v);
        if !(board.visible.contains(&target_tile) || board.visible.contains(&source_tile)) { 
            sprite.v = *target;
        } else {
            sprite.v = move_towards(sprite.v, *target, delta * MOVEMENT_SPEED);
        }

        if sprite.v == *target {
            sprite.path.pop_front();
        }
        if sprite.path.len() > 0 { ready = false }
    }
    ready
}

fn update_added_sprites(state: &mut GraphicsState, delta: f32) -> bool {
    let mut ready = true;
    for sprite in state.sprites.iter_mut().filter(|a| a.state == SpriteState::Added) {
        ready = false;
        sprite.fade += delta * FADE_SPEED;
        if sprite.fade >= 1. {
            sprite.fade = 1.;
            sprite.state = SpriteState::Existing;
        }
    }
    ready
}

fn update_removed_sprites(state: &mut GraphicsState, delta: f32) -> bool {
    let mut ready = true;
    let mut to_remove = HashSet::new();
    for sprite in state.sprites.iter_mut().filter(|a| a.state == SpriteState::Removed) {
        ready = false;
        sprite.fade -= delta * FADE_SPEED;
        if sprite.fade <= 0. {
            to_remove.insert(sprite.entity);
        }
    }
    state.sprites.retain(|a| !to_remove.contains(&a.entity) && a.state != SpriteState::Removed);
    ready
}

pub fn draw_sprites(world: &World, state: &GraphicsState, context: &mut Context_) {
    let Some(board) = world.get_resource::<Board>() else { return };
    for sprite in state.sprites.iter() {
        let tile = world_to_tile(sprite.v);
        if !board.discovered.contains(&tile) { continue; }
        if !board.visible.contains(&tile) {
            if world.get_component::<Actor>(sprite.entity).is_some() { continue; }
        }

        let color = Color(
            sprite.color.0,
            sprite.color.1,
            sprite.color.2,
            (sprite.color.3 as f32 * sprite.fade) as u8,
        );
        let _ = context.graphics.draw_atlas_sprite(
            &sprite.atlas_name,
            (sprite.index + sprite.frame) as usize,
            sprite.v,
            sprite.z_index,
            Vector2f::new(TILE_SIZE, TILE_SIZE),
            Params2d { color, ..Default::default() }
        );
    }
}

pub fn draw_fog(world: &World, state: &GraphicsState, context: &mut Context_) {
    let Some(board) = world.get_resource::<Board>() else { return };
    for x in -2..BOARD_SIZE as i32 + 2 {
        for y in -2..BOARD_SIZE as i32 + 2 {
            let vi = Vector2i::new(x, y);
            if board.visible.contains(&vi) { continue; }
            let idx = if board.discovered.contains(&vi) { 1 } else { 0 };

            let _ = context.graphics.draw_atlas_sprite(
                "fog",
                idx,
                tile_to_world(vi) - Vector2f::new(0.5, 0.5) * TILE_SIZE,
                FOG_Z,
                Vector2f::new(TILE_SIZE, TILE_SIZE) * 2.0,
                Params2d { color: BACKGROUND_COLOR, ..Default::default() }
            );
        }
    }
}

fn fade_sprite(entity: Entity, state: &mut GraphicsState, value: f32) {
    let Some(sprite) = get_entity_sprite_mut(entity, state) else { return };
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
    } else if world.get_component::<Item>(entity).is_some() {
        z_index = ITEM_Z
    } else if world.get_component::<Actor>(entity).is_some() {
        z_index = ACTOR_Z
    }

    let data = game_data.entities.get(&name.0).expect(
        &format!("No data found for {}", name.0)
    );

    let color = match world.get_component::<Discoverable>(entity) {
        Some(_) => game_data.discoverable_colors.get(&name.0)
            .expect(&format!("No color assigned for {}!", name.0)).1,
        None => data.sprite.color
    };

    let frame_count = match data.sprite.frames {
        Some(a) => a,
        None => 1
    };

    SpriteRenderer { 
        entity: entity,
        v: tile_to_world(position.0),
        path: VecDeque::new(),
        atlas_name: data.sprite.atlas_name.clone(),
        index: data.sprite.index,
        z_index,
        color,
        fade: 0.,
        state: SpriteState::Added,
        frame: 0,
        frame_count
    }
}

fn get_projectile_renderer(
    entity: Entity,
    world: &World,
) -> SpriteRenderer {
    let projectile = world.get_component::<Projectile>(entity).unwrap();
    let mut path = VecDeque::new();
    path.push_back(tile_to_world(projectile.target));

    SpriteRenderer { 
        entity: entity,
        v: tile_to_world(projectile.source),
        path,
        atlas_name: "items".into(),
        index: 8,
        z_index: PROJECTILE_Z,
        color: Color(189, 200, 220, 255),
        fade: 1.,
        state: SpriteState::Existing,
        frame: 0,
        frame_count: 1
    }
}

pub fn get_entity_sprite(entity: Entity, state: &GraphicsState) -> Option<&SpriteRenderer> {
    state.sprites.iter()
        .find(|a| a.entity == entity)
}

pub fn get_entity_sprite_mut(entity: Entity, state: &mut GraphicsState) -> Option<&mut SpriteRenderer> {
    state.sprites.iter_mut()
        .find(|a| a.entity == entity)
}
