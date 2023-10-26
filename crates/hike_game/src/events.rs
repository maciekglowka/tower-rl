use rogalik::{
    math::vectors::Vector2i,
    storage::Entity
};

#[derive(Clone, Copy, Debug)]
pub enum ActionEvent {
    Other,
    BoardReady,
    Bump(Entity, Vector2i),
    Health(Entity, i32),
    HealPoison(Entity),
    Poison(Entity, i32),
    Attack(Entity, Vector2i),
    Travel(Entity, bool) // bool: is_animated
}