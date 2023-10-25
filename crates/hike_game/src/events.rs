use rogalik::{
    math::vectors::Vector2i,
    storage::Entity
};

#[derive(Clone, Copy)]
pub enum ActionEvent {
    Other,
    BoardReady,
    Bump(Entity, Vector2i),
    Attack(Entity, Vector2i),
    Travel(Entity, bool) // bool: is_animated
}