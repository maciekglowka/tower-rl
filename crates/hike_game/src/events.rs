use rogalik::{
    math::vectors::Vector2i,
    storage::Entity
};

#[derive(Clone, Copy, Debug)]
pub enum GameEvent {
    Other,
    BoardReady,
    Bump(Entity, Vector2i),
    Health(Entity, i32),
    HealPoison(Entity),
    Immunity(Entity),
    Poison(Entity, i32),
    Attack(Entity, Vector2i),
    HitProjectile(Vector2i),
    Travel(Entity, bool), // bool: is_animated,
    PickInstant,
}