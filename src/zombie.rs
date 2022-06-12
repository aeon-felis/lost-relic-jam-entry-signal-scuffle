use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_yoleck::vpeol_2d::{yoleck_vpeol_position_edit_adapter, YoleckVpeolTransform2dProjection};
use bevy_yoleck::{YoleckExtForApp, YoleckPopulate, YoleckTypeHandler};
use serde::{Deserialize, Serialize};

use crate::global_types::{AppState, IsWifi, IsZombie};
use crate::loading::GameAssets;
use crate::utils::some_or;

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
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(follow_wifi_signal));
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

fn follow_wifi_signal(
    time: Res<Time>,
    mut zombies_query: Query<(&GlobalTransform, &mut Velocity), With<IsZombie>>,
    wifi_query: Query<&GlobalTransform, With<IsWifi>>,
) {
    for (zombie_transform, mut velocity) in zombies_query.iter_mut() {
        let zombie_position = zombie_transform.translation.truncate();
        let closest_wifi_position = wifi_query
            .iter()
            .map(|transform| transform.translation.truncate())
            .min_by_key(|wifi_position| {
                float_ord::FloatOrd(zombie_position.distance_squared(*wifi_position))
            });
        let closest_wifi_position = some_or!(closest_wifi_position; continue);
        let vec_to_wifi = closest_wifi_position - zombie_position;
        if vec_to_wifi.length_squared() < 1.0 {
            continue;
        }
        let direction_to_wifi = vec_to_wifi.normalize();
        let current_zombie_direction = zombie_transform.rotation.mul_vec3(Vec3::Y).truncate();
        let angle_diff = direction_to_wifi.angle_between(current_zombie_direction);
        if angle_diff.is_nan() {
            continue;
        }
        if 0.1 < angle_diff.abs() {
            velocity.angvel = -angle_diff.signum();
        } else {
            velocity.angvel = -angle_diff;
        }
        let dot_product = current_zombie_direction.dot(direction_to_wifi);
        if 0.3 < dot_product {
            let current_forward_speed = current_zombie_direction.dot(velocity.linvel);
            const ZOMBIE_SPEED: f32 = 1.0;
            if current_forward_speed < dot_product * ZOMBIE_SPEED {
                velocity.linvel += current_zombie_direction * dot_product * time.delta_seconds();
            }
        }
    }
}
