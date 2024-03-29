use rogalik::{
    math::vectors::Vector2i,
    storage::Entity
};

#[derive(Clone, Copy, Debug)]
pub enum GameEvent {
    Other,
    TurnEnd,
    BoardReady,
    Bump(Entity, Vector2i),
    Health(Entity, i32),
    HealPoison(Entity),
    Immunity(Entity),
    Regeneration(Entity),
    Poison(Entity, i32),
    Attack(Entity, Vector2i),
    HitProjectile(Vector2i),
    Travel(Entity, bool), // bool: is_animated,
    Ascend,
    PickItem,
    UseCollectable,
    Upgrade,
    Spawn,
    Win,
    Defeat
}