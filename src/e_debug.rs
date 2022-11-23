use bevy::prelude::*;

use crate::{z_utils::round_to_two, e_crosshair::Crosshair, g_logic::{ScoreBoard, DartsLeft}};

#[derive(Component)]
pub struct DebugInfo;

pub fn setup_debug_info(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/Minecraft.ttf");

    let text_style = TextStyle {
        font: font.clone(),
        font_size: 5.0,
        color: Color::WHITE,
    };

    let bundle_style = Style {
        position_type: PositionType::Absolute,
        position: UiRect {
            top: Val::Px(15.0),
            right: Val::Px(15.0),
            ..default()
        },
        ..default()
    };

    commands
        .spawn(
            TextBundle::from_sections([
                TextSection::new("PlayerScore", text_style.clone()),
                TextSection::new("OpponentScore", text_style.clone()),
                TextSection::new("Distance", text_style.clone()),
                TextSection::new("N_dist", text_style.clone()),
                TextSection::new("Degrees", text_style.clone()),
                TextSection::new("PlayerDarts", text_style.clone()),
            ])
            .with_style(bundle_style),
        )
        .insert(DebugInfo);
}

pub fn update_debug_info(
    mut q_debug: Query<&mut Text, With<DebugInfo>>,
    q_crosshair: Query<&Crosshair>,
    r_scoreboard: Res<ScoreBoard>,
    r_dartsleft: Res<DartsLeft>,
) {
    let p_score = r_scoreboard.player;
    let o_score = r_scoreboard.opponent;
    let n_dist = q_crosshair.single().n_dist;
    let degrees = round_to_two(q_crosshair.single().degrees);
    let distance = round_to_two(q_crosshair.single().distance);
    let p_darts = r_dartsleft.0;

    for mut text in &mut q_debug {
        text.sections[0].value = format!("Player: {p_score:10}");
        text.sections[1].value = format!("\nOpponent: {o_score:10}");
        text.sections[2].value = format!("\nDistance: {distance:10}");
        text.sections[3].value = format!("\nn_dist:{n_dist:10}");
        text.sections[4].value = format!("\nDegrees: {degrees:10}");
        text.sections[5].value = format!("\nPlayer darts: {p_darts:10}");

    }
}