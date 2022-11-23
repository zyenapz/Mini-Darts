use bevy::prelude::*;

use crate::{e_board::BOARD_CENTER, g_zval::Z_CROSSHAIR};

#[derive(Component, Default)]
pub struct Crosshair {
    pub distance: f32,
    pub n_dist: f32,
    pub degrees: f32,
}

#[derive(Resource)]
pub struct CrosshairImage {
    pub unfocused: Handle<Image>,
    pub focused: Handle<Image>,
}

pub fn setup_crosshair(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load crosshair images
    let img_unfocused: Handle<Image> = asset_server.load("images/crosshair.png");
    let img_focused: Handle<Image> = asset_server.load("images/crosshair_foc.png");

    // Insert crosshair image resource
    commands.insert_resource(CrosshairImage {
        unfocused: img_unfocused.clone(),
        focused: img_focused,
    });
    
    // Spawn crosshair sprite
    commands
        .spawn(SpriteBundle {
            texture: img_unfocused,
            transform: Transform {
                translation: Vec3 {
                    x: BOARD_CENTER.x,
                    y: BOARD_CENTER.y,
                    z: Z_CROSSHAIR,
                },
                ..default()
            },
            ..default()
        })
        .insert(Crosshair::default());
}
