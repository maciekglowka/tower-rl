use rogalik::{
    math::vectors::Vector2I,
    storage::Entity
};

#[derive(Clone, Copy)]
pub enum ActionEvent {
    Other,
    BoardReady,
    Bump(Entity, Vector2I),
    Attack(Entity, Vector2I),
    // Melee(Entity, Vector2I, u32),
    Travel(Entity, Vector2I)
}