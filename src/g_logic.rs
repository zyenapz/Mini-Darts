use std::ops::Sub;

use bevy::prelude::*;

use crate::{
    e_board::{Sections, BOARD_CENTER, BOARD_RADIUS},
    e_crosshair::Crosshair,
    z_utils::{normalize, round_to_two},
};
use rand::Rng;

// Radii definition of the rings (normalized, with respect to board's center)
pub const R_BULEYE: f32 = 0.01; // Bullseye
pub const R_HALBEY: f32 = 0.02; // Half-Bullseye
pub const R_TRINEA: f32 = 0.10; // Treble-near
pub const R_TRIFAR: f32 = 0.11; // Treble-far
pub const R_DOBNEA: f32 = 0.17; // Double-near
pub const R_DOBFAR: f32 = 0.18; // Double-far

#[derive(Resource)]
pub struct MousePosition(Vec2);

#[derive(Resource)]
pub struct MouseOnScreen(pub bool);

#[derive(Resource)]
pub struct AimIsFocused(pub bool);

#[derive(Resource)]
pub struct ScoreBoard {
    pub player: i32,
    pub opponent: i32,
}

#[derive(Resource)]
pub struct DartsLeft(pub i8);

impl DartsLeft {
    pub fn reset(&mut self) {
        self.0 = 3_i8;
    }

    pub fn decrease(&mut self) {
        self.0 -= 1_i8;
    }
}

#[derive(Resource, Debug, Clone, Eq, PartialEq, Hash)]
pub enum CurrentTurn {
    Player,
    Opponent,
}

pub fn setup_logic(mut commands: Commands) {
    commands.insert_resource(MouseOnScreen(true));
    commands.insert_resource(MousePosition(Vec2 {
        x: BOARD_CENTER.x,
        y: BOARD_CENTER.y,
    }));
    commands.insert_resource(ScoreBoard {
        player: 301,
        opponent: 301,
    });
    commands.insert_resource(AimIsFocused(false));
    commands.insert_resource(DartsLeft(3));
    commands.insert_resource(CurrentTurn::Player);
}

pub fn bound_crosshair(
    r_windows: Res<Windows>,
    mut q_crosshair: Query<&mut Transform, With<Crosshair>>,
) {
    let window = r_windows.get_primary().unwrap();
    let mut crosshair = q_crosshair.single_mut();

    let window_size = Vec2 {
        x: window.width(),
        y: window.height(),
    };
    let w_halfwidth = window_size.x / 2.;
    let w_halfheight = window_size.y / 2.;

    if crosshair.translation.x > w_halfwidth {
        crosshair.translation.x = w_halfwidth
    }
    if crosshair.translation.x < -w_halfwidth {
        crosshair.translation.x = -w_halfwidth
    }
    if crosshair.translation.y > w_halfheight {
        crosshair.translation.y = w_halfheight
    }
    if crosshair.translation.y < -w_halfheight {
        crosshair.translation.y = -w_halfheight
    }
}

pub fn shake_crosshair(
    mut q_crosshair: Query<&mut Transform, With<Crosshair>>,
    r_focused: Res<AimIsFocused>,
) {
    const SHAKE_FOC: f32 = 0.2;
    const SHAKE_UFOC: f32 = 1.5;

    let mut crosshair = q_crosshair.single_mut();

    match r_focused.0 {
        true => {
            crosshair.translation.x += rand::thread_rng().gen_range(-SHAKE_FOC..SHAKE_FOC);
            crosshair.translation.y += rand::thread_rng().gen_range(-SHAKE_FOC..SHAKE_FOC);
        }
        false => {
            crosshair.translation.x += rand::thread_rng().gen_range(-SHAKE_UFOC..SHAKE_UFOC);
            crosshair.translation.y += rand::thread_rng().gen_range(-SHAKE_UFOC..SHAKE_UFOC);
        }
    }
}

pub fn update_scoreboard(n_dist: f32, degrees: f32, r_scoreboard: &mut ScoreBoard, r_sections: &Sections) {
    if n_dist <= R_BULEYE {
        r_scoreboard.player -= 50;
    } else if n_dist <= R_HALBEY {
        r_scoreboard.player -= 25;
    } else if n_dist <= R_DOBFAR {
        for section in &r_sections.0 {
            let mut multiplier = 1;

            let landed_treble = (R_TRINEA..=R_TRIFAR).contains(&n_dist);
            let landed_double = (R_DOBNEA..=R_DOBFAR).contains(&n_dist);
            let landed_single = (section.start..section.end).contains(&degrees);

            if landed_treble {
                multiplier = 3;
            } else if landed_double {
                multiplier = 2;
            }

            if landed_single {
                r_scoreboard.player -= section.score * multiplier;
                eprintln!(
                    "Hit {} worth {} pts! ",
                    section.score,
                    section.score * multiplier
                );
            }
        }
    }
}

pub fn check_turn(mut r_turn: ResMut<CurrentTurn>, mut r_darts: ResMut<DartsLeft>) {
    let is_player_turn = r_turn.eq(&CurrentTurn::Player);
    let no_darts_left = r_darts.0 <= 0;

    if is_player_turn && no_darts_left {
        *r_turn = CurrentTurn::Opponent;
        r_darts.reset();
    }
}

pub fn is_player_turn(r_turn: Res<CurrentTurn>) -> bool {
    r_turn.eq(&CurrentTurn::Player)
}
