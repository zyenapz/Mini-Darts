use bevy::{prelude::*, input::mouse::MouseMotion, render::camera::RenderTarget};

#[derive(Component)]
struct MainCamera;

const BOARD_CENTER: Vec2 = Vec2::new(0., 0.);
const BOARD_RADIUS: f32 = 300_f32;

// Board's bounds (normalized, with respect to board's center)
const R_BE:f32 = 0.07; // Bullseye
const R_HB:f32 = 0.11; // Half-Bullseye
const R_TN:f32 = 0.29; // Triple-near
const R_TF:f32 = 0.31; // Triple-far
const R_DN:f32 = 0.48; // Double-near
const R_DF:f32 = 0.50; // Double-far

fn main() {
    App::new().add_plugins(DefaultPlugins).add_startup_system(setup).add_system(cursor_position).run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);
    commands.spawn(SpriteBundle {
        texture: asset_server.load("smiley_Face.png"),
        transform: Transform {scale: Vec3 { x: 0.2, y: 0.2, z: 1. }, ..default() },
        ..default()
    });
}

fn cursor_position(
   // need to get window dimensions
   wnds: Res<Windows>,
   // query to get camera transform
   q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        //eprintln!("World coords: {}/{}", world_pos.x, world_pos.y);

        let x1 = BOARD_CENTER.x;
        let y1 = BOARD_CENTER.y;

        let x2 = world_pos.x;
        let y2 = world_pos.y;

        let degrees = (y1.atan2(x1) - y2.atan2(x2)).to_degrees();
        //eprintln!("Angle: {}", degrees);

        let distance = BOARD_CENTER.distance(world_pos);

        let q = normalize(distance, 0., BOARD_RADIUS);
        eprintln!("Distance: {}, Normalized: {}", distance, q);
        
        
    }
}

fn normalize(val: f32, min: f32, max: f32) -> f32 {
    return (val - min) / (max - min);
}