use rogalik::engine::{Color, GraphicsContext, Instant};
use rogalik::events::{EventBus, SubscriberHandle};
use rogalik::math::vectors::{Vector2i, Vector2f};
use rogalik::storage::{ComponentSet, Entity, World, WorldEvent};

use hike_data::Settings;
use hike_game::{get_player_entity, GameEvent};

use crate::UiEvent;
use super::GraphicsState;
use super::globals::{TILE_SIZE, BOARD_V_OFFSET, UI_TOP_OFFSET};

pub mod bubbles;
mod buttons;
mod context_menu;
mod game_end;
mod help;
mod input;
mod inventory;
mod messages;
mod modal;
mod overlays;
mod span;
mod status;
mod text_box;
mod utils;

#[derive(Clone, Copy, Default)]
pub struct InputState {
    pub mouse_world_position: Vector2f,
    pub mouse_screen_position: Vector2f,
    pub mouse_button_left: ButtonState,
    pub touch: bool,
    pub direction: InputDirection,
    pub action_left: ButtonState,
    pub action_right: ButtonState,
    pub pause: ButtonState,
    pub digits: [ButtonState; 10],
    pub item_action: [ButtonState; 4], // ZXCV
}

pub struct UiState {
    pub direction_buffer: Option<InputDirection>,
    mode: UiMode,
    bubbles: Vec<bubbles::Bubble>,
    game_duration: f32,
    pub build_version: String,
    pub message: Option<String>,
    ev_game: SubscriberHandle<GameEvent>,
    pub last_action: Option<Instant>
}
impl UiState {
    pub fn new(events: &mut EventBus<GameEvent>) -> Self {
        Self {
            direction_buffer: None,
            mode: UiMode::default(),
            bubbles: Vec::new(),
            game_duration: 0.,
            build_version: String::new(),
            message: None,
            ev_game: events.subscribe(),
            last_action: None
        }
    }
}

#[derive(Default)]
pub enum UiMode {
    #[default]
    Game,
    HelpMenu,
    GameEnd,
}

#[derive(Clone, Copy, Default, PartialEq)]
pub enum InputDirection {
    #[default]
    None,
    Up,
    Down,
    Left,
    Right,
    Still
}

#[derive(Clone, Copy, Default, PartialEq)]
pub enum ButtonState {
    #[default]
    Up,
    Down,
    Pressed,
    Released
}

pub fn draw_world_ui(
    world: &World,
    context: &mut crate::Context_,
    state: &mut GraphicsState
) {
    overlays::draw_overlays(world, context, state);
}

pub fn ui_update(
    world: &mut World,
    input_state: &mut InputState,
    ui_state: &mut UiState,
    events: &mut EventBus<UiEvent>,
    context: &mut crate::Context_,
    settings: &mut Settings
) {
    match ui_state.mode {
        UiMode::Game => {
            update_game_ui(world, input_state, ui_state, context);
            if let Some(stats) = world.get_resource::<hike_game::GameStats>() {
                if get_player_entity(world).is_none() || stats.win {
                    ui_state.mode = UiMode::GameEnd;
                    ui_state.game_duration = stats.start.elapsed()
                }
            }
        },
        UiMode::HelpMenu => help::handle_help_menu(context, input_state, ui_state, settings),
        UiMode::GameEnd => game_end::handle_menu(context, input_state, ui_state, events, world)
    }
}

fn update_game_ui(
    world: &mut World,
    input_state: &mut InputState,
    ui_state: &mut UiState,
    context: &mut crate::Context_,
) {
    handle_action_events(world, ui_state);
    status::draw_status(world, context);
    let mut ui_click = false;

    inventory::handle_inventory(world, context, input_state);
    messages::update_messages(world, context, ui_state);
    bubbles::handle_bubbles(world, ui_state, context);

    if context_menu::handle_menu(world, context, input_state) {
        ui_click = true
    }
    if help::handle_help_button(context, input_state, ui_state) {
        ui_click = true
    }
    if ui_click { return };
    input::handle_dir_input(world, input_state, ui_state, context);
}

pub fn get_viewport_bounds(context: &crate::Context_) -> (Vector2f, Vector2f) {
    let scale = context.graphics.get_current_camera().get_scale();
    let half_size = 0.5 * context.get_physical_size() / scale;
    let centre = (hike_game::globals::BOARD_SIZE as f32) * TILE_SIZE / 2.;
    (
        Vector2f::new(centre - half_size.x, centre - half_size.y + BOARD_V_OFFSET),
        Vector2f::new(centre + half_size.x, centre + half_size.y + BOARD_V_OFFSET - UI_TOP_OFFSET)
    )
}

fn handle_action_events(
    world: &World,
    state: &mut UiState
) {
    for ev in state.ev_game.read().iter().flatten() {
        bubbles::handle_game_event(ev, world, state);
        messages::handle_game_event(ev, world, state);
    }
}
