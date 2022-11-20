use core::fmt;
use std::ops::*;

use bevy::{input::mouse::MouseMotion, prelude::*};
use rand::Rng;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Crosshair;

#[derive(Resource)]
struct Sections(Vec<Section>);

#[derive(Resource)]
struct MouseOnScreen(bool);

#[derive(Resource)]
struct AimIsFocused(bool);

#[derive(Resource)]
struct MousePosition(Vec2);

#[derive(Resource)]
struct ScoreBoard {
    player: i32,
    opponent: i32,
}

struct Section {
    start: f32,
    end: f32,
    score: i32,
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Start: {}, End: {}", self.start, self.end)
    }
}

const BOARD_CENTER: Vec2 = Vec2::new(-25., 0.);
const BOARD_RADIUS: f32 = 300_f32;
const SECTION_ARC: f32 = 18_f32; // 20 sections divided by 360 = 18

// Board's rings' bounds (normalized, with respect to board's center)
const R_BULEYE: f32 = 0.02; // Bullseye
const R_HALBEY: f32 = 0.04; // Half-Bullseye
const R_TRINEA: f32 = 0.29; // Treble-near
const R_TRIFAR: f32 = 0.31; // Treble-far
const R_DOBNEA: f32 = 0.48; // Double-near
const R_DOBFAR: f32 = 0.50; // Double-far

const SCALE_FACTOR: f32 = 4.0;
const CROSSHAIR_RNG_RANGE: f32 = 1.5;

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
        .add_startup_system(setup)
        .add_system(move_crosshair)
        .add_system(shoot_dart)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: ResMut<Windows>) {
    windows
        .get_primary_mut()
        .unwrap()
        .set_cursor_visibility(false);
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);
    commands.spawn(SpriteBundle {
        texture: asset_server.load("board.png"),
        transform: Transform {
            translation: Vec3 {
                x: BOARD_CENTER.x,
                y: BOARD_CENTER.y,
                z: 1.,
            },
            scale: Vec3 {
                x: 1.,
                y: 1.,
                z: 0.,
            },
            ..default()
        },
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("frame.png"),
        transform: Transform {
            translation: Vec3 {
                x: BOARD_CENTER.x,
                y: BOARD_CENTER.y,
                z: 0.,
            },
            scale: Vec3 {
                x: 1.,
                y: 1.,
                z: 0.,
            },
            ..default()
        },
        ..default()
    });

    // Create crosshair
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("crosshair.png"),
            transform: Transform {
                translation: Vec3 {
                    x: BOARD_CENTER.x,
                    y: BOARD_CENTER.y,
                    z: 2.,
                },
                scale: Vec3 {
                    x: 1.,
                    y: 1.,
                    z: 0.,
                },
                ..default()
            },
            ..default()
        })
        .insert(Crosshair);

    // Create sections
    let mut sec: Vec<Section> = Vec::new();
    let scores = vec![
        20, 5, 12, 9, 14, 11, 8, 16, 7, 19, 3, 17, 2, 15, 10, 6, 13, 4, 18, 1,
    ];

    for i in 0..20 {
        let s = Section {
            start: i as f32 * 18_f32,
            end: (i as f32 * 18_f32) + 18_f32,
            // ingredient: ingredients[i % ingredients.len()],
            score: scores[i % scores.len()],
        };

        sec.push(s);
    }

    commands.insert_resource(Sections(sec));
    commands.insert_resource(MouseOnScreen(true));
    commands.insert_resource(MousePosition(Vec2 {
        x: BOARD_CENTER.x,
        y: BOARD_CENTER.y,
    }));
    commands.insert_resource(ScoreBoard {
        player: 301,
        opponent: 301,
    })
}

fn move_crosshair(
    r_windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut q_crosshair: Query<&mut Transform, With<Crosshair>>,
    mut evr_motion: EventReader<MouseMotion>,
    mut r_mouse_onscreen: ResMut<MouseOnScreen>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = r_windows.get_primary().unwrap();

    // Check if the cursor is inside the window and get its position
    if let Some(screen_pos) = window.cursor_position() {
        // Get the size of the window
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);

        // Convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // Matrix for undoing the projection and camera transform
        let matrix = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // Use it to convert ndc to world-space coordinates
        let world_pos = matrix.project_point3(ndc.extend(-1.0)).truncate();

        // Check if mouse is back on screen
        if r_mouse_onscreen.0 == false {
            q_crosshair.single_mut().translation = world_pos.extend(1.);
            r_mouse_onscreen.0 = true
        }

        // Move crosshair
        for evt in evr_motion.iter() {
            q_crosshair.single_mut().translation.x += evt.delta.x % 5.;
            q_crosshair.single_mut().translation.y += -evt.delta.y % 5.;
        }

        // Shake the crosshair
        q_crosshair.single_mut().translation.x +=
            rand::thread_rng().gen_range(-CROSSHAIR_RNG_RANGE..CROSSHAIR_RNG_RANGE);
        q_crosshair.single_mut().translation.y +=
            rand::thread_rng().gen_range(-CROSSHAIR_RNG_RANGE..CROSSHAIR_RNG_RANGE);
    } else {
        r_mouse_onscreen.0 = false;
    }
}

fn shoot_dart(
    r_mbuttons: Res<Input<MouseButton>>,
    r_sections: Res<Sections>,
    mut r_scoreboard: ResMut<ScoreBoard>,
    mut q_crosshair: Query<&mut Transform, With<Crosshair>>,
) {
    if r_mbuttons.just_pressed(MouseButton::Left) {
        // Calculate angle between board's center and mouse pos, then offset it so that angle 0 starts on score the right wire of score '20'
        let board_crosshair = BOARD_CENTER.sub(q_crosshair.single_mut().translation.truncate()); // Boardcenter to mouse vector
        let offset = 459_f32; // Just tweaked the number until I got this
        let degrees = (board_crosshair.y.atan2(board_crosshair.x).to_degrees() + offset) % 360_f32;

        let distance = BOARD_CENTER.distance(q_crosshair.single_mut().translation.truncate());

        let n_dist = round_to_two(normalize(distance, 0., BOARD_RADIUS));
        eprintln!(
            "Angle: {}, Distance: {}, Normalized distance: {}",
            degrees, distance, n_dist
        );

        // Calculate score
        if n_dist <= R_BULEYE {
            r_scoreboard.player -= 50;
            eprintln!("Hit bullseye worth 50 pts!")
        } else if n_dist <= R_HALBEY {
            r_scoreboard.player -= 25;
            eprintln!("Hit half bullseye worth 25 pts!")
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
                    eprintln!("Hit {} worth {} pts! ", section.score, section.score * multiplier);
                }
            }
        } else {
            eprintln!("Missed!");
        }
    }
}

fn normalize(val: f32, min: f32, max: f32) -> f32 {
    (val - min) / (max - min)
}

fn round_to_two(val: f32) -> f32 {
    (val * 100.0).round() / 100.0
}
