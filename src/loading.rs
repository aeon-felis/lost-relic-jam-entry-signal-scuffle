use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetCollectionApp};
use bevy_yoleck::YoleckLevelIndex;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<GameAssets>();
    }
}

#[derive(AssetCollection)]
pub struct GameAssets {
    #[asset(path = "sprites/player.png")]
    pub player: Handle<Image>,
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub font: Handle<Font>,
    #[asset(path = "levels/index.yoli")]
    pub level_index: Handle<YoleckLevelIndex>,
}
