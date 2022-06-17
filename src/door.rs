use bevy::prelude::*;
use bevy_rapier2d::pipeline::CollisionEvent;
use bevy_rapier2d::prelude::{ActiveEvents, Collider, Sensor};
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;
use bevy_yoleck::vpeol_2d::{yoleck_vpeol_position_edit_adapter, YoleckVpeolTransform2dProjection};
use bevy_yoleck::{YoleckExtForApp, YoleckPopulate, YoleckTypeHandler};
use serde::{Deserialize, Serialize};

use crate::global_types::{AppState, CameraInclude, DoorStatus, DownloadProgress, IsPlayer};
use crate::loading::GameAssets;
use crate::utils::entities_ordered_by_type;

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_handler({
            YoleckTypeHandler::<Door>::new("Door")
                .populate_with(populate)
                .with(yoleck_vpeol_position_edit_adapter(|door: &mut Door| {
                    YoleckVpeolTransform2dProjection {
                        translation: &mut door.position,
                    }
                }))
        });
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(update_doors_status));
        app.add_system(handle_door_reached_events);
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct Door {
    #[serde(default)]
    position: Vec2,
}

fn populate(mut populate: YoleckPopulate<Door>, game_assets: Res<GameAssets>) {
    populate.populate(|_, data, mut cmd| {
        cmd.insert(DoorStatus { is_open: false });
        cmd.insert_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 0,
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..Default::default()
            },
            texture_atlas: game_assets.door.clone(),
            ..Default::default()
        });
        cmd.insert_bundle(TransformBundle::from_transform(
            Transform::from_translation(data.position.extend(-0.1)),
        ));
        cmd.insert(CameraInclude);
        cmd.insert(Collider::cuboid(0.5, 0.5));
        cmd.insert(Sensor(true));
        cmd.insert(ActiveEvents::COLLISION_EVENTS);
    });
}

fn update_doors_status(
    downloads_query: Query<&DownloadProgress>,
    mut doors_query: Query<(&mut DoorStatus, &mut TextureAtlasSprite)>,
) {
    let should_be_open = downloads_query
        .iter()
        .any(|progress| matches!(progress, DownloadProgress::Completed));
    for (mut door_status, mut sprite) in doors_query.iter_mut() {
        door_status.is_open = should_be_open;
        sprite.index = if should_be_open { 1 } else { 0 };
    }
}

fn handle_door_reached_events(
    mut reader: EventReader<CollisionEvent>,
    player_query: Query<(), With<IsPlayer>>,
    door_query: Query<&DoorStatus>,
    mut state: ResMut<State<AppState>>,
) {
    for event in reader.iter() {
        let [_player_entity, door_entity] = match event {
            CollisionEvent::Started(entity1, entity2, flags) => {
                if flags.contains(CollisionEventFlags::SENSOR) {
                    if let Some(entities) =
                        entities_ordered_by_type!([*entity1, *entity2], player_query, door_query)
                    {
                        entities
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            }
            CollisionEvent::Stopped(_, _, _) => {
                continue;
            }
        };
        let door_status = door_query.get(door_entity).unwrap();
        if door_status.is_open {
            state.set(AppState::LevelCompleted).unwrap();
            return;
        }
    }
}
