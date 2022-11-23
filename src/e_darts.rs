use bevy::prelude::*;

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

pub fn spawn_darts(mut commands: Commands, images: Res<DartImage>, transform: &Transform) {
    commands
        .spawn(SpriteBundle {
            texture: images.player.clone(),
            transform: Transform {
                translation: Vec3 {
                    x: transform.translation.x,
                    y: transform.translation.y,
                    z: 2.,
                },
                ..default()
            },
            ..default()
        })
        .insert(PlayerDart);
}
