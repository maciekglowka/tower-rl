pub const TILE_SIZE: f32 = 1.;
pub const PERSP_RATIO: f32 = 1.0;
pub const BOARD_V_OFFSET: f32 = -2.;
pub const BACKGROUND_COLOR: super::Color = super::Color(38, 18, 34, 255);

pub const ACTOR_Z: u32 = 200;
pub const FIXTURE_Z: u32 = 100;
pub const ITEM_Z: u32 = 150;
pub const PROJECTILE_Z: u32 = 250;
pub const TILE_Z: u32 = 50;

pub const INACTIVE_FADE: f32 = 0.5;
pub const ANIMATION_TICK: f32 = 1.;

pub const MOVEMENT_SPEED: f32 = 10.;
pub const FADE_SPEED: f32 = 5.;

// ui
pub const UI_BOTTOM_PANEL_HEIGHT: f32 = 4.25;
pub const UI_BUTTON_HEIGHT: f32 = 1.25;
pub const UI_GAP: f32 = 0.2;
pub const UI_TEXT_GAP: f32 = 0.1;
pub const UI_BUTTON_TEXT_SIZE: f32 = 0.5;
pub const UI_STATUS_TEXT_SIZE: f32 = 0.5;
pub const BUTTON_COLOR: super::Color = super::Color(98, 81, 81, 255);
pub const BUTTON_COLOR_SELECTED: super::Color = super::Color(150, 128, 128, 255);
pub const PANEL_BORDER: f32 = 0.05;

#[cfg(target_os = "android")]
pub const UI_TOP_OFFSET: f32 = 1.;
#[cfg(not(target_os = "android"))]
pub const UI_TOP_OFFSET: f32 = 0.;
