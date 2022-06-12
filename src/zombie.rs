use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_yoleck::vpeol_2d::{yoleck_vpeol_position_edit_adapter, YoleckVpeolTransform2dProjection};
use bevy_yoleck::{YoleckExtForApp, YoleckPopulate, YoleckTypeHandler};
use serde::{Deserialize, Serialize};

use crate::global_types::IsZombie;
use crate::loading::GameAssets;

pub struct ZombiePlugin;

impl Plugin for ZombiePlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_handler({
            YoleckTypeHandler::<Zombie>::new("Zombie")
                .populate_with(populate)
                .with(yoleck_vpeol_position_edit_adapter(|zombie: &mut Zombie| {
                    YoleckVpeolTransform2dProjection {
                        translation: &mut zombie.position,
                    }
                }))
        });
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Zombie {
    #[serde(default)]
    position: Vec2,
}

fn populate(mut populate: YoleckPopulate<Zombie>, game_assets: Res<GameAssets>) {
    populate.populate(|_ctx, data, mut cmd| {
        cmd.insert(IsZombie);
        cmd.insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..Default::default()
            },
            texture: game_assets.zombie.clone(),
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
        cmd.insert(Collider::cuboid(0.4, 0.2));
        cmd.insert(ColliderMassProperties::Density(10.0));
        cmd.insert(Velocity::default());
    });
}
