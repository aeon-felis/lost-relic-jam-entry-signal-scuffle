use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::global_types::{DownloadProgress, WifiClient};
use crate::utils::some_or;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(show_score);
    }
}

fn show_score(
    mut egui_context: ResMut<EguiContext>,
    player_query: Query<(&WifiClient, &DownloadProgress)>,
) {
    let (wifi_client, download_progress) = some_or!(player_query.get_single().ok(); return);
    let panel = egui::Area::new("score-area").fixed_pos([0.0, 0.0]);
    panel.show(egui_context.ctx_mut(), |ui| {
        ui.set_max_width(200.0);
        ui.scope(|ui| {
            ui.style_mut().visuals.selection.bg_fill = egui::Color32::YELLOW;
            ui.add(
                egui::ProgressBar::new(wifi_client.signal_strength).text(format!(
                    "Signal Strength {:.2}",
                    wifi_client.signal_strength
                )),
            );
        });
        ui.scope(|ui| match download_progress {
            DownloadProgress::Disconnected => {}
            DownloadProgress::LosingConnection {
                time_before_disconnection,
                progress,
            } => {
                ui.style_mut().visuals.selection.bg_fill = egui::Color32::RED;
                ui.add(egui::ProgressBar::new(*progress).text(format!(
                    "Losing Progress In {:.0}",
                    time_before_disconnection
                )));
            }
            DownloadProgress::Downloading { progress } => {
                ui.style_mut().visuals.selection.bg_fill = egui::Color32::BLUE;
                ui.add(
                    egui::ProgressBar::new(*progress)
                        .text(format!("Downloading: {:.0}%", 100.0 * progress)),
                );
            }
            DownloadProgress::Completed => {
                ui.style_mut().visuals.selection.bg_fill = egui::Color32::GREEN;
                ui.add(egui::ProgressBar::new(1.0).text("Download Complete"));
            }
        });
    });
}
