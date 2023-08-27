use serde::Deserialize;

#[derive(Deserialize)]
pub enum ItemKind {
    Debris,
    Survivor
}