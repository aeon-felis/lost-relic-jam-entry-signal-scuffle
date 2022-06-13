use bevy::prelude::*;
use bevy_yoleck::vpeol_2d::{yoleck_vpeol_position_edit_adapter, YoleckVpeolTransform2dProjection};
use bevy_yoleck::{YoleckExtForApp, YoleckPopulate, YoleckTypeHandler};
use serde::{Deserialize, Serialize};

use crate::global_types::{AppState, IsWifi, WifiClient};
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
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(update_access_points));
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

fn update_access_points(
    mut clients_query: Query<(&GlobalTransform, &mut WifiClient)>,
    wifis_query: Query<(Entity, &GlobalTransform), With<IsWifi>>,
) {
    for (client_transform, mut client) in clients_query.iter_mut() {
        if let Some((wifi_entity, signal_strength)) = wifis_query
            .iter()
            .map(|(wifi_entity, wifi_transform)| {
                let distance_sq = client_transform
                    .translation
                    .distance_squared(wifi_transform.translation);
                let signal_strength = if distance_sq < 1.0 {
                    1.0
                } else {
                    1.0 / distance_sq
                };
                (wifi_entity, signal_strength)
            })
            .max_by_key(|(_, signal_strength)| float_ord::FloatOrd(*signal_strength))
        {
            client.access_point = Some(wifi_entity);
            client.signal_strength = signal_strength;
        } else {
            client.access_point = None;
            client.signal_strength = 0.0;
        }
    }
}
