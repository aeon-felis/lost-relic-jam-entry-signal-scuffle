use bevy::prelude::*;
use bevy_egui::egui;
use bevy_rapier2d::prelude::*;
use bevy_yoleck::vpeol_2d::{yoleck_vpeol_position_edit_adapter, YoleckVpeolTransform2dProjection};
use bevy_yoleck::{YoleckEdit, YoleckExtForApp, YoleckPopulate, YoleckTypeHandler};
use serde::{Deserialize, Serialize};

pub struct PlayableAreaPlugin;

impl Plugin for PlayableAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_handler({
            YoleckTypeHandler::<PlayableArea>::new("PlayableArea")
                .populate_with(populate)
                .with(yoleck_vpeol_position_edit_adapter(|playable_area: &mut PlayableArea| {
                    YoleckVpeolTransform2dProjection {
                        translation: &mut playable_area.position,
                    }
                }))
                .edit_with(edit)
        });
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayableArea {
    #[serde(default)]
    position: Vec2,
    #[serde(default = "default_size")]
    size: Vec2,
}

fn default_size() -> Vec2 {
    Vec2::new(1.0, 1.0)
}

fn populate(mut populate: YoleckPopulate<PlayableArea>) {
    populate.populate(|_ctx, data, mut cmd| {
        cmd.despawn_descendants();
        cmd.insert_bundle(TransformBundle::from_transform(Transform::from_translation(data.position.extend(-0.1))));
        cmd.with_children(|commands| {
            for (offset_direction, size) in [
                (-Vec2::Y, Vec2::new(data.size.x, 0.0)),
            ] {
                let mut cmd = commands.spawn();
                cmd.insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::DARK_GRAY,
                        custom_size: Some(size),
                        ..Default::default()
                    },
                    transform: Transform::from_translation((offset_direction * 0.5 * data.size).extend(0.0)),
                    ..Default::default()
                });
                cmd.insert(RigidBody::Fixed);
                cmd.insert(Collider::cuboid(0.5 * size.x, 0.5 * size.y));
            }
        });
    });
}

fn edit(mut edit: YoleckEdit<PlayableArea>) {
    edit.edit(|_ctx, data, ui| {
        let orig_size = data.size;
        ui.horizontal(|ui| {
            ui.add(
                egui::DragValue::new(&mut data.size.x)
                    .prefix("Width:")
                    .speed(0.05),
            );
            ui.add(
                egui::DragValue::new(&mut data.size.y)
                    .prefix("Height:")
                    .speed(0.05),
            );
        });
        if orig_size != data.size {
            let top_left = data.position - 0.5 * orig_size;
            data.position = top_left + 0.5 * data.size;
        }
    });
}
