use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub swipe_sensitivity: u32,
    pub dirty: bool
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            swipe_sensitivity: 5,
            dirty: false
        }
    }
}
