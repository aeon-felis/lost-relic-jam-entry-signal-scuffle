use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::global_types::{IsPlayer, WifiClient};
use crate::utils::some_or;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(show_score);
    }
}

fn show_score(
    mut egui_context: ResMut<EguiContext>,
    player_query: Query<&WifiClient, With<IsPlayer>>,
) {
    let player_wifi_client = some_or!(player_query.get_single().ok(); return);
    let panel = egui::Area::new("score-area").fixed_pos([0.0, 0.0]);
    panel.show(egui_context.ctx_mut(), |ui| {
        ui.add(
            egui::ProgressBar::new(player_wifi_client.signal_strength)
                .text("Signal Strength")
                .desired_width(200.0),
        );
    });
}
