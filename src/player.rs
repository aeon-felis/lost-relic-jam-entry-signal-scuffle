use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_yoleck::vpeol_2d::{yoleck_vpeol_position_edit_adapter, YoleckVpeolTransform2dProjection};
use bevy_yoleck::{YoleckExtForApp, YoleckPopulate, YoleckTypeHandler};
use serde::{Deserialize, Serialize};

use crate::global_types::IsPlayer;
use crate::loading::GameAssets;
use crate::player_control::PlayerControl;
//use crate::player_control::PlayerControl;
//use crate::yoleck_utils::{position_adapter, GRANULARITY};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_handler({
            YoleckTypeHandler::<Player>::new("Player")
                .populate_with(populate)
                .with(yoleck_vpeol_position_edit_adapter(|player: &mut Player| {
                    YoleckVpeolTransform2dProjection {
                        translation: &mut player.position,
                    }
                }))
        });
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    #[serde(default)]
    position: Vec2,
}

fn populate(mut populate: YoleckPopulate<Player>, game_assets: Res<GameAssets>) {
    populate.populate(|_ctx, data, mut cmd| {
        cmd.insert(IsPlayer);
        cmd.insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..Default::default()
            },
            texture: game_assets.player.clone(),
            ..Default::default()
        });
        cmd.insert_bundle(TransformBundle::from_transform(
            Transform::from_translation(data.position.extend(0.0)),
        ));
        cmd.insert(RigidBody::Dynamic);
        cmd.insert(Damping {
            linear_damping: 1.0,
            angular_damping: 1.0,
        });
        cmd.insert(Collider::cuboid(0.75, 0.25));
        cmd.insert(ActiveEvents::COLLISION_EVENTS);
        cmd.insert(Velocity::default());
        //cmd.insert(LockedAxes::ROTATION_LOCKED);
        cmd.insert(PlayerControl::default());
    });
}
