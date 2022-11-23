use std::ops::Sub;

use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::{
    e_crosshair::{Crosshair, CrosshairImage},
    g_logic::{AimIsFocused, MouseOnScreen, ScoreBoard, DartsLeft, R_TRINEA, R_DOBNEA, R_DOBFAR, R_TRIFAR, R_HALBEY, R_BULEYE, update_scoreboard},
    e_window::MainCamera, e_board::{Sections, BOARD_CENTER, BOARD_RADIUS}, e_darts::{DartImage, PlayerDart}, z_utils::{round_to_two, normalize},
};

pub fn focus_aim(
    mut r_focused: ResMut<AimIsFocused>,
    r_keyboard: ResMut<Input<KeyCode>>,
    mut q_crosshair: Query<&mut Handle<Image>, With<Crosshair>>,
    r_images: Res<CrosshairImage>,
) {
    let mut img_handle = q_crosshair.single_mut();

    if r_keyboard.pressed(KeyCode::Space) {
        r_focused.0 = true;
        *img_handle = r_images.focused.clone();
    } else if r_keyboard.just_released(KeyCode::Space) {
        r_focused.0 = false;
        *img_handle = r_images.unfocused.clone();
    }
}

pub fn move_crosshair(
    r_windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut q_crosshair: Query<&mut Transform, With<Crosshair>>,
    mut evr_motion: EventReader<MouseMotion>,
    mut r_mouse_onscreen: ResMut<MouseOnScreen>,
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

        // crosshair.translation.x = world_pos.x;
        // crosshair.translation.y = world_pos.y;
    } else {
        r_mouse_onscreen.0 = false;
    }
}

pub fn shoot_dart(
    mut commands: Commands,
    r_mbuttons: Res<Input<MouseButton>>,
    r_sections: Res<Sections>,
    r_dart_img: Res<DartImage>,
    mut r_scoreboard: ResMut<ScoreBoard>,
    mut q_crosshair: Query<(&mut Transform, &mut Crosshair)>,
    mut r_darts_left: ResMut<DartsLeft>,
) {
    let mut crosshair = q_crosshair.single_mut();

    // Calculate angle between board's center and mouse pos, then offset it so that angle 0 starts on score the right wire of score '20'
    let board_crosshair = BOARD_CENTER.sub(crosshair.0.translation.truncate()); // Boardcenter to mouse vector
    let offset = 459_f32; // Just tweaked the number until I got this
    let degrees = (board_crosshair.y.atan2(board_crosshair.x).to_degrees() + offset) % 360_f32;

    let distance = BOARD_CENTER.distance(crosshair.0.translation.truncate());

    let n_dist = round_to_two(normalize(distance, 0., BOARD_RADIUS));

    // crosshair.1.degrees = degrees;
    // crosshair.1.n_dist = n_dist;
    // crosshair.1.distance = distance;

    if r_mbuttons.just_pressed(MouseButton::Left) {
        // Decrease dart count
        r_darts_left.decrease();

        // Update scoreboard
        update_scoreboard(n_dist, degrees, &mut r_scoreboard, &r_sections);

        // Spawn darts sprite
        commands
            .spawn(SpriteBundle {
                texture: r_dart_img.player.clone(),
                transform: Transform {
                    translation: Vec3 {
                        x: crosshair.0.translation.x,
                        y: crosshair.0.translation.y,
                        z: 2.,
                    },
                    ..default()
                },
                ..default()
            })
            .insert(PlayerDart);
    }
}
