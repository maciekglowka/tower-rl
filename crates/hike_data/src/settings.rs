use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Settings {
    pub swipe_sensitivity: u32,
    pub swipe_repeat_delay: u32,
    pub dirty: bool
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            swipe_sensitivity: 5,
            swipe_repeat_delay: 2,
            dirty: false
        }
    }
}
