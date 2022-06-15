use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::JointAxesMask;
use bevy_yoleck::vpeol_2d::{yoleck_vpeol_position_edit_adapter, YoleckVpeolTransform2dProjection};
use bevy_yoleck::{egui, YoleckEdit, YoleckExtForApp, YoleckPopulate, YoleckTypeHandler};
use ezinput::prelude::{InputView, PressStateExt};
use serde::{Deserialize, Serialize};

use crate::global_types::{
    AppState, DownloadProgress, GrabStatus, Grabbable, InputBinding, IsPlayer, WifiClient,
};
use crate::loading::GameAssets;
use crate::movement_resolver::MoveController;
use crate::player_control::PlayerControl;

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
                .edit_with(edit)
        });
        app.add_system_set({
            SystemSet::on_update(AppState::Game)
                .with_system(control_grabbing_initiation)
                .with_system(handle_grabbing_taking_hold)
        });
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    #[serde(default)]
    position: Vec2,
    #[serde(default)]
    rotation: f32,
}

fn populate(mut populate: YoleckPopulate<Player>, game_assets: Res<GameAssets>) {
    populate.populate(|_ctx, data, mut cmd| {
        cmd.insert(IsPlayer);
        let transform = Transform::from_translation(data.position.extend(0.0))
            .with_rotation(Quat::from_rotation_z(data.rotation));
        cmd.insert_bundle(SpriteBundle {
            transform,
            global_transform: transform.into(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..Default::default()
            },
            texture: game_assets.player.clone(),
            ..Default::default()
        });
        cmd.insert(RigidBody::Dynamic);
        cmd.insert(Damping {
            linear_damping: 1.0,
            angular_damping: 1.0,
        });
        cmd.insert(Collider::cuboid(0.4, 0.2));
        cmd.insert(ColliderMassProperties::Density(10.0));
        cmd.insert(Velocity::default());
        cmd.insert(PlayerControl::default());
        cmd.insert(MoveController::default());
        cmd.insert(WifiClient::default());
        cmd.insert(DownloadProgress::Disconnected);
        cmd.insert(GrabStatus::NoGrab);
        cmd.insert(ActiveEvents::COLLISION_EVENTS);
    });
}

fn edit(mut edit: YoleckEdit<Player>) {
    edit.edit(|_, data, ui| {
        use std::f32::consts::{FRAC_PI_8, PI};
        ui.add({
            egui::Slider::new(&mut data.rotation, PI..=-PI)
                .prefix("Angle: ")
                .step_by(FRAC_PI_8 as f64)
        });
    });
}

fn control_grabbing_initiation(
    time: Res<Time>,
    input_views: Query<&InputView<InputBinding>>,
    mut grabbers_query: Query<(Entity, &mut GrabStatus)>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    let should_grab = input_views
        .iter()
        .any(|input_view| input_view.key(&InputBinding::Grab).pressed());

    for (grabber_entity, mut grab_status) in grabbers_query.iter_mut() {
        *grab_status = match *grab_status {
            GrabStatus::NoGrab => {
                if should_grab {
                    GrabStatus::Reaching {
                        hands_entity: {
                            let mut cmd = commands.spawn();
                            let transform = Transform::from_translation(Vec3::new(0.0, 0.4, 0.1));
                            cmd.insert_bundle(SpriteBundle {
                                transform,
                                global_transform: transform.into(),
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(1.0, 1.0)),
                                    ..Default::default()
                                },
                                texture: game_assets.hands.clone(),
                                ..Default::default()
                            });
                            cmd.insert(Collider::cuboid(0.4, 0.2));
                            cmd.insert(Sensor(true));

                            let hands_entity = cmd.id();
                            commands.entity(grabber_entity).add_child(hands_entity);
                            hands_entity
                        },
                        how_long: 0.0,
                    }
                } else {
                    GrabStatus::NoGrab
                }
            }
            GrabStatus::GrabFailed => {
                if should_grab {
                    GrabStatus::GrabFailed
                } else {
                    GrabStatus::NoGrab
                }
            }
            GrabStatus::Reaching {
                hands_entity,
                how_long,
            } => {
                if should_grab {
                    let how_long = how_long + time.delta_seconds();
                    if how_long < 0.2 {
                        GrabStatus::Reaching {
                            hands_entity,
                            how_long,
                        }
                    } else {
                        commands.entity(hands_entity).despawn_recursive();
                        GrabStatus::GrabFailed
                    }
                } else {
                    commands.entity(hands_entity).despawn_recursive();
                    GrabStatus::NoGrab
                }
            }
            GrabStatus::Holding {
                hands_entity,
                other,
            } => {
                if should_grab {
                    GrabStatus::Holding {
                        hands_entity,
                        other,
                    }
                } else {
                    commands.entity(hands_entity).despawn_recursive();
                    GrabStatus::NoGrab
                }
            }
        }
    }
}

fn handle_grabbing_taking_hold(
    mut grabbers_query: Query<(&GlobalTransform, &mut GrabStatus)>,
    grabbable_query: Query<(Entity, &GlobalTransform), With<Grabbable>>,
    mut commands: Commands,
) {
    for (grabber_transform, mut grab_status) in grabbers_query.iter_mut() {
        let hands_entity = if let GrabStatus::Reaching {
            hands_entity,
            how_long,
        } = *grab_status
        {
            if how_long < 0.01 {
                continue;
            }
            hands_entity
        } else {
            continue;
        };
        if let Some((grabbable_entity, _)) = grabbable_query
            .iter()
            .filter_map(|(grabbable_entity, grabbable_transform)| {
                let vec = (grabber_transform.rotation.inverse()
                    * (grabbable_transform.translation - grabber_transform.translation))
                    .truncate();
                if vec.x.abs() <= 1.0 && 0.0 <= vec.y && vec.y <= 1.0 {
                    Some((grabbable_entity, vec))
                } else {
                    None
                }
            })
            .min_by_key(|(_, vec)| float_ord::FloatOrd(vec.x.abs() + vec.y * 5.0))
        {
            let mut joint = GenericJoint::new(JointAxesMask::all());
            joint.set_local_anchor2(Vec2::new(0.0, 0.5));
            commands
                .entity(hands_entity)
                .insert(ImpulseJoint::new(grabbable_entity, joint));
            *grab_status = GrabStatus::Holding {
                hands_entity,
                other: grabbable_entity,
            };
        }
    }
}
