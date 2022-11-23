use bevy::prelude::*;

use crate::{e_crosshair::Crosshair, g_logic::DartShotEvent};

#[derive(Resource)]
pub struct DartImage {
    pub player: Handle<Image>,
    opponent: Handle<Image>,
}

#[derive(Component)] // Marker for player's dart image
pub struct PlayerDart;

pub fn setup_darts(mut commands: Commands, asset_server: Res<AssetServer>) {
    let img_darts_p: Handle<Image> = asset_server.load("images/darts_p.png");
    let img_darts_o: Handle<Image> = asset_server.load("images/darts_o.png");

    commands.insert_resource(DartImage {
        player: img_darts_p,
        opponent: img_darts_o,
    });
}

pub fn spawn_darts(
    mut commands: Commands,
    r_images: Res<DartImage>,
    mut q_crosshair: Query<&mut Transform, With<Crosshair>>,
    mut ev_dartshot: EventReader<DartShotEvent>,
) {
    let crosshair = q_crosshair.single_mut();

    for ev in ev_dartshot.iter() {
        let dart_was_shot = ev.0 == true;

        if dart_was_shot {
            commands
                .spawn(SpriteBundle {
                    texture: r_images.player.clone(),
                    transform: Transform {
                        translation: Vec3 {
                            x: crosshair.translation.x,
                            y: crosshair.translation.y,
                            z: 2.,
                        },
                        ..default()
                    },
                    ..default()
                })
                .insert(PlayerDart);
        }
    }
}
