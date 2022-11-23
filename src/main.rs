mod e_board;
mod e_crosshair;
mod e_darts;
mod e_debug;
mod e_window;
mod g_controls;
mod g_logic;
mod z_utils;

use bevy::prelude::*;
use e_board::{setup_board_sections, setup_board_sprites};
use e_crosshair::setup_crosshair;
use e_darts::{setup_darts, spawn_darts};
use e_debug::{setup_debug_info, update_debug_info};
use e_window::setup_window;
use g_controls::{focus_aim, move_mouse, shoot_dart};
use g_logic::{bound_crosshair, check_turn, is_player_turn, setup_logic, shake_crosshair, AimFocusedEvent, DartShotEvent, update_scoreboard};
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InGame,
    Paused,
    GameOver,
}

const SCALE_FACTOR: f32 = 4.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Mini Darts".to_string(),
                        width: 200.,
                        height: 150.,
                        scale_factor_override: Some(SCALE_FACTOR.into()),
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()), // For pixel-art style
        )
        .add_event::<AimFocusedEvent>()
        .add_event::<DartShotEvent>()
        .add_loopless_state(AppState::InGame)
        .add_startup_system_set(
            SystemSet::new()
                .with_system(setup_window)
                .with_system(setup_logic)
                .with_system(setup_board_sprites)
                .with_system(setup_board_sections)
                .with_system(setup_crosshair)
                .with_system(setup_debug_info)
                .with_system(setup_darts),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::InGame)
                .run_if(is_player_turn)
                .label("p_turn")
                .with_system(move_mouse)
                .with_system(shoot_dart)
                .with_system(focus_aim)
                .with_system(shake_crosshair)
                .with_system(bound_crosshair)
                .with_system(update_debug_info)
                .with_system(check_turn)
                .with_system(update_scoreboard)
                .with_system(spawn_darts)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::InGame)
                .run_if_not(is_player_turn)
                .label("ai_turn")
                .with_system(say_hello)
                .into(),
        )
        .run();
}

fn say_hello() {
    // TODO
    println!("Hello world!");
}
