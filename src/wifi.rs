use bevy::prelude::*;
use bevy_yoleck::vpeol_2d::{yoleck_vpeol_position_edit_adapter, YoleckVpeolTransform2dProjection};
use bevy_yoleck::{YoleckExtForApp, YoleckPopulate, YoleckTypeHandler};
use serde::{Deserialize, Serialize};

use crate::global_types::IsWifi;
use crate::loading::GameAssets;

pub struct WifiPlugin;

impl Plugin for WifiPlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_handler({
            YoleckTypeHandler::<Wifi>::new("Wifi")
                .populate_with(populate)
                .with(yoleck_vpeol_position_edit_adapter(|wifi: &mut Wifi| {
                    YoleckVpeolTransform2dProjection {
                        translation: &mut wifi.position,
                    }
                }))
        });
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Wifi {
    #[serde(default)]
    position: Vec2,
}

fn populate(mut populate: YoleckPopulate<Wifi>, game_assets: Res<GameAssets>) {
    populate.populate(|_ctx, data, mut cmd| {
        cmd.insert(IsWifi);
        cmd.insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0)),
                color: Color::rgba(1.0, 1.0, 1.0, 0.9),
                ..Default::default()
            },
            texture: game_assets.wifi.clone(),
            ..Default::default()
        });
        cmd.insert_bundle(TransformBundle::from_transform(
            Transform::from_translation(data.position.extend(1.0)),
        ));
    });
}
