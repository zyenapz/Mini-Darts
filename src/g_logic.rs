use std::{ops::Sub, time::Duration};

use bevy::prelude::*;

use crate::{
    e_board::{Sections, BOARD_CENTER, BOARD_RADIUS, R_BULEYE, R_HALBEY, R_DOBFAR, R_TRINEA, R_DOBNEA, R_TRIFAR},
    e_crosshair::Crosshair,
    z_utils::{normalize, round_to_two}, g_events::{AimFocusedEvent, DartShotEvent},
};
use rand::Rng;

#[derive(Resource)]
pub struct MousePosition(Vec2);

#[derive(Resource)]
pub struct MouseOnScreen(pub bool); // TODO: Convert this into an event

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

#[derive(Resource)]
pub struct AimFocusTimer {
    pub timer: Timer,
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
    commands.insert_resource(DartsLeft(3));
    commands.insert_resource(CurrentTurn::Player);

    // Timers
    commands.insert_resource(AimFocusTimer { timer: Timer::new(Duration::from_secs(1), TimerMode::Once) });
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
    mut ev_aimfocused: EventReader<AimFocusedEvent>,
) {
    const SHAKE_FOC: f32 = 0.2;
    const SHAKE_UFOC: f32 = 1.5;

    let mut crosshair = q_crosshair.single_mut();

    for ev in ev_aimfocused.iter() {
        match ev.0 {
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
}

pub fn update_scoreboard(
    r_sections: Res<Sections>,
    mut r_scoreboard: ResMut<ScoreBoard>,
    mut q_crosshair: Query<(&mut Transform, &mut Crosshair)>,
    mut ev_dartshot: EventReader<DartShotEvent>,
) {
    for ev in ev_dartshot.iter() {
        let dart_was_shot = ev.0 == true;

        if dart_was_shot {
            let crosshair = q_crosshair.single_mut();

            // Calculate angle between board's center and mouse pos, then offset it so that angle 0 starts on score the right wire of score '20'
            let bc_vect = BOARD_CENTER.sub(crosshair.0.translation.truncate()); // Boardcenter to mouse vector
            let offset = 459_f32; // Just tweaked the number until I got this
            let degrees =
                (bc_vect.y.atan2(bc_vect.x).to_degrees() + offset) % 360_f32;

            let distance = BOARD_CENTER.distance(crosshair.0.translation.truncate());

            let n_dist = round_to_two(normalize(distance, 0., BOARD_RADIUS));

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
                    }
                }
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
