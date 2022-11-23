use bevy::{prelude::*, window::CursorGrabMode};

#[derive(Component)]
pub struct MainCamera;

pub fn setup_window(mut commands: Commands, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();

    // Setup cursor
    window.set_cursor_visibility(false);
    window.set_cursor_grab_mode(CursorGrabMode::Confined);

    // Setup camera
    commands
        .spawn(Camera2dBundle {
            transform: Transform {
                translation: Vec3 {
                    z: 999.,
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .insert(MainCamera);
}
