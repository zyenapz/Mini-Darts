use core::fmt;
use std::ops::*;

use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorGrabMode};
use rand::Rng;

#[derive(Component)]
struct MainCamera;

#[derive(Component, Default)]
struct Crosshair {
    distance: f32,
    n_dist: f32,
    degrees: f32,
}

#[derive(Component)]
struct DebugText;

#[derive(Resource)]
struct Sections(Vec<Section>);

#[derive(Resource)]
struct MouseOnScreen(bool);

#[derive(Resource)]
struct AimIsFocused(bool);

#[derive(Resource)]
struct CrosshairImages {
    unfocused: Handle<Image>,
    focused: Handle<Image>
}

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
const R_BULEYE: f32 = 0.01; // Bullseye
const R_HALBEY: f32 = 0.02; // Half-Bullseye
const R_TRINEA: f32 = 0.10; // Treble-near
const R_TRIFAR: f32 = 0.11; // Treble-far
const R_DOBNEA: f32 = 0.17; // Double-near
const R_DOBFAR: f32 = 0.18; // Double-far

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
        .add_system(update_texts)
        .add_system(focus_aim)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: ResMut<Windows>) {

    windows
        .get_primary_mut()
        .unwrap()
        .set_cursor_visibility(false);

    windows
        .get_primary_mut()
        .unwrap()
        .set_cursor_grab_mode(CursorGrabMode::Confined);

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
    let img_unfocused: Handle<Image> = asset_server.load("crosshair.png");
    let img_focused: Handle<Image> = asset_server.load("crosshair_foc.png");
    commands.insert_resource(CrosshairImages { unfocused: img_unfocused.clone(), focused: img_focused });

    commands
        .spawn(SpriteBundle {
            texture: img_unfocused,
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
        .insert(Crosshair { ..default() });

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
    });
    commands.insert_resource(AimIsFocused(false));

    // Create text
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "Player: 301",
                TextStyle {
                    font: asset_server.load("fonts/Minecraft.ttf"),
                    font_size: 5.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "\nOpponent: 301",
                TextStyle {
                    font: asset_server.load("fonts/Minecraft.ttf"),
                    font_size: 5.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "\nDistance: 0 | n_dist: 0 | Degrees: 0",
                TextStyle {
                    font: asset_server.load("fonts/Minecraft.ttf"),
                    font_size: 5.0,
                    color: Color::WHITE,
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(15.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        DebugText,
    ));
}

fn focus_aim(
    mut r_focused: ResMut<AimIsFocused>,
    r_keyboard: ResMut<Input<KeyCode>>,
    mut q_crosshair: Query<&mut Handle<Image>, With<Crosshair>>,
    r_images: Res<CrosshairImages>
) {
    let mut img_handle = q_crosshair.single_mut();

    if r_keyboard.pressed(KeyCode::Space) {
        r_focused.0 = true;
        *img_handle = r_images.focused.clone();
    }
    else if r_keyboard.just_released(KeyCode::Space) {
        r_focused.0 = false;
        *img_handle = r_images.unfocused.clone();
    }

}

fn move_crosshair(
    r_windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut q_crosshair: Query<&mut Transform, With<Crosshair>>,
    mut evr_motion: EventReader<MouseMotion>,
    mut r_mouse_onscreen: ResMut<MouseOnScreen>,
    r_focused: Res<AimIsFocused>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = r_windows.get_primary().unwrap();
    let mut crosshair = q_crosshair.single_mut();

    // Check if the cursor is inside the window and get its position
    if let Some(screen_pos) = window.cursor_position() {
        // Get mouse position
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let matrix = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        let mouse_pos = matrix.project_point3(ndc.extend(-1.0)).truncate();

        // Check if mouse is back on screen
        if r_mouse_onscreen.0 == false {
            crosshair.translation = mouse_pos.extend(1.);
            r_mouse_onscreen.0 = true
        }

        // Move crosshair
        for evt in evr_motion.iter() {
            crosshair.translation.x += evt.delta.x % 5.;
            crosshair.translation.y += -evt.delta.y % 5.;
        }
        
        // Bound crosshair within the window's bounds 

        let win_hw = window_size.x / 2.;
        let win_hh = window_size.y / 2.;

        if crosshair.translation.x > win_hw {
            crosshair.translation.x = win_hw
        }
        if crosshair.translation.x < -win_hw {
            crosshair.translation.x = -win_hw
        }
        if crosshair.translation.y > win_hh {
            crosshair.translation.y = win_hh
        }
        if crosshair.translation.y < -win_hh {
            crosshair.translation.y = -win_hh
        }

        // crosshair.translation.x = world_pos.x;
        // crosshair.translation.y = world_pos.y;

        // Shake the crosshair
        match r_focused.0 {
            true => {
                crosshair.translation.x +=
                    rand::thread_rng().gen_range(-0.2..0.2);
                    crosshair.translation.y +=
                    rand::thread_rng().gen_range(-0.2..0.2);
            }
            false => {
                crosshair.translation.x +=
                    rand::thread_rng().gen_range(-CROSSHAIR_RNG_RANGE..CROSSHAIR_RNG_RANGE);
                    crosshair.translation.y +=
                    rand::thread_rng().gen_range(-CROSSHAIR_RNG_RANGE..CROSSHAIR_RNG_RANGE);
            },
        }

    } else {
        r_mouse_onscreen.0 = false;
    }
}

fn shoot_dart(
    r_mbuttons: Res<Input<MouseButton>>,
    r_sections: Res<Sections>,
    mut r_scoreboard: ResMut<ScoreBoard>,
    mut q_crosshair: Query<(&mut Transform, &mut Crosshair)>,
) {
    let mut crosshair = q_crosshair.single_mut();

    // Calculate angle between board's center and mouse pos, then offset it so that angle 0 starts on score the right wire of score '20'
    let board_crosshair = BOARD_CENTER.sub(crosshair.0.translation.truncate()); // Boardcenter to mouse vector
    let offset = 459_f32; // Just tweaked the number until I got this
    let degrees = (board_crosshair.y.atan2(board_crosshair.x).to_degrees() + offset) % 360_f32;

    let distance = BOARD_CENTER.distance(crosshair.0.translation.truncate());

    let n_dist = round_to_two(normalize(distance, 0., BOARD_RADIUS));

    crosshair.1.degrees = degrees;
    crosshair.1.n_dist = n_dist;
    crosshair.1.distance = distance;

    if r_mbuttons.just_pressed(MouseButton::Left) {
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
                    eprintln!(
                        "Hit {} worth {} pts! ",
                        section.score,
                        section.score * multiplier
                    );
                }
            }
        } else {
            eprintln!("Missed!");
        }
    }
}

fn update_texts(
    mut q_debug: Query<&mut Text, With<DebugText>>,
    r_scoreboard: Res<ScoreBoard>,
    q_crosshair: Query<&Crosshair>,
) {
    let p_score = r_scoreboard.player;
    let o_score = r_scoreboard.opponent;
    let n_dist = q_crosshair.single().n_dist;
    let degrees = q_crosshair.single().degrees;
    let distance = q_crosshair.single().distance;

    for mut text in &mut q_debug {
        text.sections[0].value = format!("Player: {p_score:2}");
        text.sections[1].value = format!("\nOpponent: {o_score:2}");
        text.sections[2].value =
            format!("\nDistance: {distance:2}\n n_dist: {n_dist:2}\n Degrees: {degrees:2}")
    }
}

fn normalize(val: f32, min: f32, max: f32) -> f32 {
    (val - min) / (max - min)
}

fn round_to_two(val: f32) -> f32 {
    (val * 100.0).round() / 100.0
}
