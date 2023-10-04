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
    // Melee(Entity, Vector2i, u32),
    Travel(Entity, Vector2i)
}