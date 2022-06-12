use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_rapier2d::pipeline::CollisionEvent;
use bevy_rapier2d::prelude::Velocity;

use crate::global_types::{AppState, GameSystemLabel};

pub struct MovementResolverPlugin;

impl Plugin for MovementResolverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(maintain_contact_lists);
        app.add_system_set({
            SystemSet::on_update(AppState::Game)
                .with_system(apply_movement.label(GameSystemLabel::ApplyMovement))
        });
    }
}

#[derive(Component)]
pub struct MoveController {
    pub target_speed: Vec2,
    pub contacts_with: HashSet<Entity>,
    pub max_speed: f32,
    pub impulse_exponent: f32,
    pub impulse_coefficient: f32,
}

impl Default for MoveController {
    fn default() -> Self {
        Self {
            target_speed: Vec2::ZERO,
            contacts_with: Default::default(),
            max_speed: 10.0,
            impulse_exponent: 4.0,
            impulse_coefficient: 200.0,
        }
    }
}

fn maintain_contact_lists(
    mut reader: EventReader<CollisionEvent>,
    mut query: Query<&mut MoveController>,
) {
    for event in reader.iter() {
        match event {
            CollisionEvent::Started(entity1, entity2, _flags) => {
                if let Ok([mut push_adjustable1, mut push_adjustable2]) =
                    query.get_many_mut([*entity1, *entity2])
                {
                    push_adjustable1.contacts_with.insert(*entity2);
                    push_adjustable2.contacts_with.insert(*entity1);
                    // info!("{:?} {:?}", push_adjustable1.contacts_with, push_adjustable2.contacts_with);
                }
            }
            CollisionEvent::Stopped(entity1, entity2, _) => {
                for (remove, remove_from) in [(entity1, entity2), (entity2, entity1)] {
                    if let Ok(mut push_adjustable) = query.get_mut(*remove_from) {
                        push_adjustable.contacts_with.remove(remove);
                    }
                }
            }
        }
    }
}

fn apply_movement(
    time: Res<Time>,
    mut query: Query<(&MoveController, &mut Transform, &mut Velocity)>,
) {
    for (move_controller, mut transform, mut velocity) in query.iter_mut() {
        let current_speed = velocity.linvel / move_controller.max_speed;

        // TODO: Use different impulses for accelerate, decelerate and turn

        let impulse = move_controller.target_speed - current_speed;

        let orig_impulse_magnitude = impulse.length();

        // TODO: Implement a better way to finish the brake
        if orig_impulse_magnitude < 0.4 {
            velocity.linvel = move_controller.max_speed * move_controller.target_speed;
        } else {
            let impulse = if 1.0 < orig_impulse_magnitude {
                impulse.normalize()
            } else {
                impulse.normalize() * orig_impulse_magnitude.powf(move_controller.impulse_exponent)
            };

            let impulse =
                time.delta().as_secs_f32() * move_controller.impulse_coefficient * impulse;
            velocity.linvel += impulse;
        }

        if 0.1 < move_controller.target_speed.length_squared() {
            let player_forward = (transform.rotation * Vec3::Y).truncate();
            let angle_to_direction = move_controller.target_speed.angle_between(player_forward);
            if 0.1 < angle_to_direction {
                velocity.angvel = -20.0;
            } else if angle_to_direction < -0.1 {
                velocity.angvel = 20.0;
            } else {
                velocity.angvel = 0.0;
                transform.rotation =
                    Quat::from_rotation_z(-move_controller.target_speed.angle_between(Vec2::Y));
            }
        } else if move_controller.target_speed.length_squared() < 0.1 {
            velocity.angvel = 0.0;
        }
    }
}
