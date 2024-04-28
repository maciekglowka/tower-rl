pub const TILE_SIZE: f32 = 1.;
pub const PIXEL: f32 = TILE_SIZE / 16.;
pub const PERSP_RATIO: f32 = 1.0;
pub const BOARD_V_OFFSET: f32 = -2.;

// colors:
pub const BACKGROUND_COLOR: super::Color = super::Color(38, 18, 34, 255);
pub const HEALTH_COLOR: super::Color = super::Color(255, 126, 102, 255);
pub const IMMUNITY_COLOR: super::Color = super::Color(189, 200, 220, 255);
pub const POISON_COLOR: super::Color = super::Color(145, 200, 185, 255);

pub const ACTOR_Z: i32 = 200;
pub const FIXTURE_Z: i32 = 100;
pub const FOG_Z: i32 = 300;
pub const ITEM_Z: i32 = 150;
pub const PROJECTILE_Z: i32 = 250;
pub const TILE_Z: i32 = 50;

pub const UI_BUBBLE_Z: i32 = 410;
pub const UI_OVERLAY_Z: i32 = 400;
pub const UI_BG_Z: i32 = 500;
pub const UI_BUTTON_Z: i32 = 550;
pub const UI_TEXT_Z: i32 = 600;

pub const UI_BUBBLE_MAX_AGE: f32 = 2.;
pub const UI_BUBBLE_SPEED: f32 = 1.;
pub const UI_OVERLAY_FONT_SIZE: f32 = 0.4;

pub const INACTIVE_FADE: f32 = 0.5;
pub const ANIMATION_TICK: f32 = 1.;

pub const MOVEMENT_SPEED: f32 = 10.;
pub const FADE_SPEED: f32 = 5.;

// ui
pub const UI_BOTTOM_PANEL_HEIGHT: f32 = 4.25;
pub const UI_BUTTON_HEIGHT: f32 = 1.25;
pub const UI_GAP: f32 = 3. * PIXEL;
pub const UI_TEXT_GAP: f32 = 0.1;
pub const UI_BUTTON_TEXT_SIZE: f32 = 0.5;
pub const UI_STATUS_TEXT_SIZE: f32 = 0.5;
pub const BUTTON_COLOR: super::Color = super::Color(98, 81, 81, 255);
pub const BUTTON_COLOR_SELECTED: super::Color = super::Color(150, 128, 128, 255);
pub const PANEL_BORDER: f32 = 0.05;
pub const UI_BOTTOM_SAFE_AREA: f32 = UI_BOTTOM_PANEL_HEIGHT + UI_GAP; // + UI_BUTTON_HEIGHT;

#[cfg(target_os = "android")]
pub const UI_TOP_OFFSET: f32 = 0.;
#[cfg(not(target_os = "android"))]
pub const UI_TOP_OFFSET: f32 = 0.;
