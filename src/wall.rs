use bevy::math::Affine2;
use bevy::prelude::*;
use bevy_egui::egui;
use bevy_rapier2d::prelude::*;
use bevy_yoleck::vpeol_2d::{yoleck_vpeol_position_edit_adapter, YoleckVpeolTransform2dProjection};
use bevy_yoleck::{YoleckEdit, YoleckExtForApp, YoleckPopulate, YoleckTypeHandler};
use serde::{Deserialize, Serialize};

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_handler({
            YoleckTypeHandler::<Wall>::new("Wall")
                .populate_with(populate)
                .with(yoleck_vpeol_position_edit_adapter(|wall: &mut Wall| {
                    YoleckVpeolTransform2dProjection {
                        translation: &mut wall.position,
                    }
                }))
                .edit_with(edit)
        });
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Wall {
    #[serde(default)]
    position: Vec2,
    #[serde(default = "default_size")]
    size: Vec2,
    #[serde(default)]
    rotation: f32,
}

fn default_size() -> Vec2 {
    Vec2::new(1.0, 1.0)
}

fn populate(mut populate: YoleckPopulate<Wall>) {
    populate.populate(|_ctx, data, mut cmd| {
        cmd.insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::DARK_GRAY,
                custom_size: Some(data.size),
                ..Default::default()
            },
            transform: Transform::from_translation(data.position.extend(0.0))
                .with_rotation(Quat::from_rotation_z(data.rotation)),
            ..Default::default()
        });
        cmd.insert(RigidBody::Fixed);
        cmd.insert(Collider::cuboid(0.5 * data.size.x, 0.5 * data.size.y));
    });
}

fn edit(mut edit: YoleckEdit<Wall>) {
    edit.edit(|_ctx, data, ui| {
        use std::f32::consts::{FRAC_PI_8, PI};
        let orig_rotation = data.rotation;
        ui.add({
            egui::Slider::new(&mut data.rotation, PI..=-PI)
                .prefix("Angle: ")
                .step_by(FRAC_PI_8 as f64)
        });
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
        if (orig_size, orig_rotation) != (data.size, data.rotation) {
            let top_left = data.position
                - 0.5 * Affine2::from_angle(orig_rotation).transform_vector2(orig_size);
            data.position =
                top_left + 0.5 * Affine2::from_angle(data.rotation).transform_vector2(data.size);
        }
    });
}
