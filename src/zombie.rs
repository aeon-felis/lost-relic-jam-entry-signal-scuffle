use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_yoleck::vpeol_2d::{yoleck_vpeol_position_edit_adapter, YoleckVpeolTransform2dProjection};
use bevy_yoleck::{egui, YoleckEdit, YoleckExtForApp, YoleckPopulate, YoleckTypeHandler};
use serde::{Deserialize, Serialize};

use crate::global_types::{AppState, IsWifi, IsZombie};
use crate::loading::GameAssets;
use crate::movement_resolver::MoveController;
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
                .edit_with(edit)
        });
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(follow_wifi_signal));
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Zombie {
    #[serde(default)]
    position: Vec2,
    #[serde(default)]
    rotation: f32,
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
            Transform::from_translation(data.position.extend(0.0))
                .with_rotation(Quat::from_rotation_z(data.rotation)),
        ));
        cmd.insert(RigidBody::Dynamic);
        cmd.insert(Damping {
            linear_damping: 1.0,
            angular_damping: 1.0,
        });
        cmd.insert(Collider::cuboid(0.4, 0.2));
        cmd.insert(ColliderMassProperties::Density(10.0));
        cmd.insert(Velocity::default());
        cmd.insert(MoveController {
            max_speed: 1.0,
            ..Default::default()
        });
        cmd.insert(ActiveEvents::COLLISION_EVENTS);
    });
}

fn edit(mut edit: YoleckEdit<Zombie>) {
    edit.edit(|_, data, ui| {
        use std::f32::consts::{FRAC_PI_8, PI};
        ui.add({
            egui::Slider::new(&mut data.rotation, PI..=-PI)
                .prefix("Angle: ")
                .step_by(FRAC_PI_8 as f64)
        });
    });
}

fn follow_wifi_signal(
    _time: Res<Time>,
    mut zombies_query: Query<(&GlobalTransform, &mut MoveController), With<IsZombie>>,
    wifi_query: Query<&GlobalTransform, With<IsWifi>>,
) {
    for (zombie_transform, mut move_controller) in zombies_query.iter_mut() {
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
            move_controller.target_speed = vec_to_wifi;
        } else {
            move_controller.target_speed = vec_to_wifi.normalize();
        }
    }
}
