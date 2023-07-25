use rogalik::{
    math::vectors::Vector2I,
    storage::Entity
};

#[derive(Clone, Copy)]
pub enum ActionEvent {
    Other,
    Melee(Entity, Entity, u32),
    Travel(Entity, Vector2I)
}