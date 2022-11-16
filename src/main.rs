use std::ops::Sub;

use bevy::{prelude::*, render::camera::RenderTarget, sprite::MaterialMesh2dBundle};

#[derive(Component)]
struct MainCamera;

struct Ingredient(String);

struct Section {
    start: f32,
    end: f32,
    ingredient: Ingredient
}

struct Ring {
    start: f32,
    end: f32
}

const BOARD_CENTER: Vec2 = Vec2::new(0., 0.);
const BOARD_RADIUS: f32 = 300_f32;
const SECTION_ARC: f32 = 18_f32; // 20 sections divided by 360

// Board's rings' bounds (normalized, with respect to board's center)
const R_BE:f32 = 0.02; // Bullseye
const R_HB:f32 = 0.04; // Half-Bullseye
const R_TN:f32 = 0.29; // Triple-near
const R_TF:f32 = 0.31; // Triple-far
const R_DN:f32 = 0.48; // Double-near
const R_DF:f32 = 0.50; // Double-far

fn main() {
    App::new().add_plugins(DefaultPlugins).add_startup_system(setup).add_system(cursor_position).run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);
    commands.spawn(SpriteBundle {
        texture: asset_server.load("dart.png"),
        transform: Transform {
            translation: Vec3 {x: BOARD_CENTER.x, y: BOARD_CENTER.y, z: 0.},
            scale: Vec3 { x: 1.2, y: 1.2, z: 1. }, ..default() },
        ..default()
    });

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(5.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PINK)),
        transform: Transform::from_translation(Vec3::new(-0., 0., 1.)),
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

        // --------- Unused, for posterity
        //let x1 = BOARD_CENTER.x;
        //let y1 = BOARD_CENTER.y;

        //let x2 = world_pos.x;
        //let y2 = world_pos.y;

        //let board_origin = Vec2{x: 0., y:0.};

        //let degrees = ((y1.atan2(x1) - y2.atan2(x2)).to_degrees() + (SECTION_ARC / 2_f32) + 180_f32) % 360_f32;
        //let degrees = ((board_mouse.y.atan2(board_mouse.x) - board_origin.y.atan2(board_origin.x)).to_degrees() + (SECTION_ARC / 2_f32) + 180_f32) % 360_f32;
        
        // ---------

        // calculate angle between board's center and mouse pos, then offset it so that angle 0 starts on a wire or line
        let board_mouse = BOARD_CENTER.sub(world_pos); // Boardcenter to mouse vector
        let degrees = (board_mouse.y.atan2(board_mouse.x).to_degrees() + (SECTION_ARC / 2_f32) + 180_f32) % 360_f32;

        let distance = BOARD_CENTER.distance(world_pos);

        let q = normalize(distance, 0., BOARD_RADIUS);
        eprintln!("Angle: {}, Distance: {}, Normalized distance: {}", degrees, distance, q);
        
    }
}

fn normalize(val: f32, min: f32, max: f32) -> f32 {
    return (val - min) / (max - min);
}

