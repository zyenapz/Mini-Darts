use std::fmt;

use bevy::prelude::*;

pub const BOARD_CENTER: Vec2 = Vec2::new(-25., 0.);
pub const BOARD_RADIUS: f32 = 300_f32;
const SECTION_ARC: f32 = 18_f32; // 20 sections divided by 360 = 18

pub struct Section {
    pub start: f32,
    pub end: f32,
    pub score: i32,
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Start: {}, End: {}", self.start, self.end)
    }
}

#[derive(Resource)]
pub struct Sections(pub Vec<Section>);

pub fn setup_board_sprites(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("images/board.png"),
        transform: Transform {
            translation: Vec3 {
                x: BOARD_CENTER.x,
                y: BOARD_CENTER.y,
                z: 1.,
            },
            ..default()
        },
        ..default()
    });

    commands.spawn(SpriteBundle {
        texture: asset_server.load("images/frame.png"),
        transform: Transform {
            translation: Vec3 {
                x: BOARD_CENTER.x,
                y: BOARD_CENTER.y,
                z: 0.,
            },
            ..default()
        },
        ..default()
    });
}

pub fn setup_board_sections(mut commands: Commands) {
    let mut sections: Vec<Section> = Vec::new();
    let scores: Vec<i32> = vec![
        20, 5, 12, 9, 14, 11, 8, 16, 7, 19, 3, 17, 2, 15, 10, 6, 13, 4, 18, 1,
    ];

    for num in 0..20 {
        let start_angle = num as f32 * SECTION_ARC;
        let end_angle = (num as f32 * SECTION_ARC) + SECTION_ARC;

        sections.push(Section {
            start: start_angle,
            end: end_angle,
            score: scores[num % scores.len()],
        });
    }

    commands.insert_resource(Sections(sections));
}
