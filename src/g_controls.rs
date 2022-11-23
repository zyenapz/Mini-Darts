use std::ops::Not;

use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::{
    e_crosshair::{Crosshair, CrosshairImage},
    e_window::MainCamera,
    g_events::{AimFocusedEvent, DartShotEvent},
    g_logic::{AimFocusTimer, DartsLeft, MouseOnScreen},
    g_zval::Z_CROSSHAIR,
};

pub fn focus_aim(
    mut r_timer: ResMut<AimFocusTimer>,
    r_time: Res<Time>,
    r_keyboard: ResMut<Input<KeyCode>>,
    r_images: Res<CrosshairImage>,
    mut q_crosshair: Query<&mut Handle<Image>, With<Crosshair>>,
    mut ev_aimfocused: EventWriter<AimFocusedEvent>,
) {
    let mut img_handle = q_crosshair.single_mut();

    let can_focus = r_timer.timer.finished().not();

    if r_keyboard.pressed(KeyCode::Space) && can_focus {
        ev_aimfocused.send(AimFocusedEvent(true));
        *img_handle = r_images.focused.clone();

        // Tick focus timer
        r_timer.timer.tick(r_time.delta());

    } else {
        ev_aimfocused.send(AimFocusedEvent(false));
        *img_handle = r_images.unfocused.clone();
    }
}

pub fn move_mouse(
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut q_crosshair: Query<&mut Transform, With<Crosshair>>,
    mut r_mouse_onscreen: ResMut<MouseOnScreen>,
    r_time: Res<Time>,
    r_windows: Res<Windows>,
    mut evr_motion: EventReader<MouseMotion>,
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
            crosshair.translation = mouse_pos.extend(Z_CROSSHAIR);
            r_mouse_onscreen.0 = true
        }

        // Move crosshair
        for evt in evr_motion.iter() {
            crosshair.translation.x += evt.delta.x * r_time.delta_seconds() * 10.;
            crosshair.translation.y += -evt.delta.y * r_time.delta_seconds() * 10.;
        }

        // crosshair.translation.x = world_pos.x;
        // crosshair.translation.y = world_pos.y;
    } else {
        r_mouse_onscreen.0 = false;
    }
}

pub fn shoot_dart(
    r_mbuttons: Res<Input<MouseButton>>,
    mut r_darts_left: ResMut<DartsLeft>,
    mut r_timer: ResMut<AimFocusTimer>,
    mut ev_dartshot: EventWriter<DartShotEvent>,
) {
    if r_mbuttons.just_pressed(MouseButton::Left) {
        // Send events
        ev_dartshot.send(DartShotEvent(true));

        // Reset Focus timer
        r_timer.timer.reset();

        // Decrease dart count
        r_darts_left.decrease();
    }
}
