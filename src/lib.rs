mod loading;
mod menu;

use crate::loading::LoadingPlugin;

use bevy::app::App;
use bevy::prelude::*;
// use bevy_yoleck::{
// YoleckEditorState, YoleckLoadingCommand, YoleckManaged, YoleckSyncWithEditorState,
// };

use self::menu::MenuPlugin;

pub struct GamePlugin {
    pub is_editor: bool,
    pub start_at_level: Option<String>,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LoadingPlugin);
        // app.add_plugin(CameraPlugin {
        // is_editor: self.is_editor,
        // });

        app.add_system(enable_disable_physics);
        if self.is_editor {
            // app.add_plugin(YoleckSyncWithEditorState {
            // when_editor: AppState::Editor,
            // when_game: AppState::Game,
            // });
        } else {
            app.add_plugin(MenuPlugin);
            // app.add_state(AppState::Menu(MenuState::Main));
            // app.add_system_set(
            // SystemSet::on_enter(AppState::LoadLevel).with_system(handle_level_loading),
            // );
            // if let Some(start_at_level) = &self.start_at_level {
            // let start_at_level = format!("{}.yol", start_at_level);
            // app.add_startup_system(
            // move |mut level_progress: ResMut<LevelProgress>,
            // mut state: ResMut<State<AppState>>| {
            // level_progress.current_level = Some(start_at_level.clone());
            // state.set(AppState::LoadLevel).unwrap();
            // },
            // );
            // }
        }
    }
}

fn enable_disable_physics(// state: Res<State<AppState>>,
    // mut rapier_configuration: ResMut<RapierConfiguration>,
) {
    // rapier_configuration.physics_pipeline_active = *state.current() == AppState::Game;
}

// fn handle_level_loading(
// level_entities_query: Query<Entity, With<YoleckManaged>>,
// mut commands: Commands,
// asset_server: Res<AssetServer>,
// level_progress: Res<LevelProgress>,
// mut yoleck_loading_command: ResMut<YoleckLoadingCommand>,
// mut state: ResMut<State<AppState>>,
// ) {
// for entity in level_entities_query.iter() {
// commands.entity(entity).despawn_recursive();
// }
// let current_level = level_progress
// .current_level
// .as_ref()
// .expect("Entered LoadLevel state when current_level is None");
// *yoleck_loading_command =
// YoleckLoadingCommand::FromAsset(asset_server.load(&format!("levels/{}", current_level)));
// state.set(AppState::Game).unwrap();
// }
