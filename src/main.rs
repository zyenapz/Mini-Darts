use core::fmt;
use std::ops::{Sub, Mul};

use bevy::{prelude::*, render::camera::RenderTarget, sprite::MaterialMesh2dBundle, input::mouse::MouseMotion};
use rand::Rng;

#[derive(Component)]
struct MainCamera;

#[derive(Resource)]
struct Sections(Vec<Section>);

#[derive(Resource)]
struct MouseOnScreen(bool);

#[derive(Clone, Copy)]
enum Ingredient {
    Free,
    Mushroom,
    Pepperoni,
    Onions,
    Garlic
}

impl fmt::Display for Ingredient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ingredient::Free => write!(f, "{}", "FREE"),
            Ingredient::Mushroom => write!(f, "{}", "MUSHROOM"),
            Ingredient::Pepperoni => write!(f, "{}", "PEPPERONI"),
            Ingredient::Onions => write!(f, "{}", "ONIONS"),
            Ingredient::Garlic => write!(f, "{}", "GARLIC"),
        }
    }
}

struct Section {
    start: f32,
    end: f32,
    ingredient: Ingredient,
    score: i32,
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Start: {}, End: {}, Ingred: {}", self.start, self.end, self.ingredient)
    }
}

#[derive(Component)]
struct Crosshair;

const BOARD_CENTER: Vec2 = Vec2::new(0., 0.);
const BOARD_RADIUS: f32 = 300_f32;
const SECTION_ARC: f32 = 18_f32; // 20 sections divided by 360 = 18

// Board's rings' bounds (normalized, with respect to board's center)
const R_BULEYE:f32 = 0.02; // Bullseye
const R_HALBEY:f32 = 0.04; // Half-Bullseye
const R_TRINEA:f32 = 0.29; // Triple-near
const R_TRIFAR:f32 = 0.31; // Triple-far
const R_DOBNEA:f32 = 0.48; // Double-near
const R_DOBFAR:f32 = 0.50; // Double-far

const SCALE_FACTOR:f32 = 1.0;
const CROSSHAIR_RNG_RANGE: f32 = 1.0;

fn main() {
    App::new().add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "Pizza Darts".to_string(),
            width: 800. * SCALE_FACTOR,
            height: 600. * SCALE_FACTOR,
            ..default()
        },
        ..default()
    }))
    .add_startup_system(setup)
    .add_system(cursor_position)
    .run();
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>    
) {
    windows.get_primary_mut().unwrap().set_cursor_visibility(false);
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);
    commands.spawn(SpriteBundle {
        texture: asset_server.load("dart.png"),
        transform: Transform {
            translation: Vec3 {x: BOARD_CENTER.x, y: BOARD_CENTER.y, z: 0.},
            //scale: Vec3 { x: 1., y: 1., z: 1. }, 
            ..default() 
        },
        ..default()
    });

    // Create crosshair
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(3.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::BLUE)),
        transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
        ..default()
    }).insert(Crosshair);

    // Create sections
    let mut sec: Vec<Section> = Vec::new();
    let ingredients: Vec<Ingredient> = vec![
        Ingredient::Mushroom,
        Ingredient::Pepperoni,
        Ingredient::Onions,
        Ingredient::Garlic,
        Ingredient::Free,
    ];

    let scores = vec![20, 5, 12, 9, 14, 11, 8, 16, 7, 19, 3, 17, 2, 15, 10, 6, 13, 4, 18, 1];

    for i in 0..20{
        let s = Section {
            start: i as f32 * 18_f32,
            end: (i as f32 * 18_f32) + 18_f32,
            ingredient: ingredients[i % ingredients.len()],
            score: scores[i % scores.len()]
        };

        sec.push(s);
    }

    commands.insert_resource(Sections(sec));
    commands.insert_resource(MouseOnScreen(true));


}

fn cursor_position(
   // need to get window dimensions
   wnds: Res<Windows>,
   // query to get camera transform
   q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
   sections: Res<Sections>,
   buttons: Res<Input<MouseButton>>,
   mut crosshair: Query<(&mut Transform), With<Crosshair>>,
   mut motion_evr: EventReader<MouseMotion>,
   mut mouse_onscreen: ResMut<MouseOnScreen>,
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
        let mut world_pos: Vec2 = world_pos.truncate();

        // check if mouse is back on screen
           if(mouse_onscreen.0 == false) {
            crosshair.single_mut().translation = world_pos.extend(1.);
            mouse_onscreen.0 = true
        }

        // move crosshair
        for evt in motion_evr.iter() {
            crosshair.single_mut().translation.x += evt.delta.x % 10.;
            crosshair.single_mut().translation.y += -evt.delta.y % 10.;
        }

        crosshair.single_mut().translation.x += rand::thread_rng().gen_range(-CROSSHAIR_RNG_RANGE..CROSSHAIR_RNG_RANGE);
        crosshair.single_mut().translation.y += rand::thread_rng().gen_range(-CROSSHAIR_RNG_RANGE..CROSSHAIR_RNG_RANGE);
        
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

        if buttons.just_pressed(MouseButton::Left) {
            // - calculate angle between board's center and mouse pos, then offset it so that angle 0 starts on score the right wire of score '20'
            let board_mouse = BOARD_CENTER.sub(world_pos); // Boardcenter to mouse vector
            let offset = 459_f32; // Just tweaked the number until I got this
            let degrees = (board_mouse.y.atan2(board_mouse.x).to_degrees() + offset) % 360_f32;

            let distance = BOARD_CENTER.distance(world_pos);

            let n_dist = round_to_two(
                normalize(distance, 0., BOARD_RADIUS)
            );
            eprintln!("Angle: {}, Distance: {}, Normalized distance: {}", degrees, distance, n_dist);
            
            // - Determine score
            if n_dist <= R_BULEYE {
                eprintln!("Hit bullseye worth 50 pts!")
            }
            else if n_dist <= R_HALBEY {
                eprintln!("Hit half bullseye worth 25 pts!")
            }
            else if n_dist <= R_DOBFAR {
                for s in &sections.0 {
                
                    let mut multiplier = 1;
    
                    if (R_TRINEA..=R_TRIFAR).contains(&n_dist) {
                        multiplier = 3;
                    }
                    else if (R_DOBNEA..=R_DOBFAR).contains(&n_dist) {
                        multiplier = 2;
                    }
    
                    if (s.start..s.end).contains(&degrees) {
                        eprintln!("Hit {} worth {} pts! ", s.ingredient, s.score * multiplier);
                    }
                }
            }
            else {
                eprintln!("Missed!");
            }
            
        }
    }
    else {
       mouse_onscreen.0 = false;
    }
}

fn normalize(val: f32, min: f32, max: f32) -> f32 {
    (val - min) / (max - min)
}

fn round_to_two(val: f32) -> f32 {
    (val * 100.0).round() / 100.0
}