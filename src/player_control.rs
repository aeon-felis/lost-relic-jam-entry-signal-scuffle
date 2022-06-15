#![allow(unused)]
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use ezinput::prelude::*;

use crate::global_types::{AppState, GameSystemLabel, InputBinding};
use crate::movement_resolver::MoveController;

pub struct PlayerControlPlugin;

impl Plugin for PlayerControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set({
            SystemSet::on_update(AppState::Game)
                .with_system(control_player.before(GameSystemLabel::ApplyMovement))
        });
        app.insert_resource(PlayerMovementSettings {
            max_speed: 10.0,
            impulse_exponent: 4.0,
            impulse_coefficient: 400.0,
            jump_power_coefficient: 10.0,
            jump_brake_coefficient: 0.02,
            start_fall_before_peak: 10.0,
            start_of_fall_range: 10.0,
            start_of_fall_gravity_boost: 30.0,
            fall_boost_coefficient: 1.06,
            stood_on_time_coefficient: 10.0,
            uphill_move_exponent: 0.5,
            downhill_brake_exponent: 1.0,
        });
    }
}

#[derive(Component)]
pub struct PlayerControl {
    mid_jump: bool,
    last_stood_on: Vec2,
    stood_on_potential: f32,
}

impl Default for PlayerControl {
    fn default() -> Self {
        Self {
            mid_jump: false,
            last_stood_on: Vec2::Y,
            stood_on_potential: 0.0,
        }
    }
}

// TODO: Clear old settings from Danger Doofus that are not needed here because it's a top-down
// game.
struct PlayerMovementSettings {
    pub max_speed: f32,
    pub impulse_exponent: f32,
    pub impulse_coefficient: f32,
    pub jump_power_coefficient: f32,
    pub jump_brake_coefficient: f32,
    pub start_fall_before_peak: f32,
    pub start_of_fall_range: f32,
    pub start_of_fall_gravity_boost: f32,
    pub fall_boost_coefficient: f32,
    pub stood_on_time_coefficient: f32,
    pub uphill_move_exponent: f32,
    pub downhill_brake_exponent: f32,
}

fn control_player(
    time: Res<Time>,
    input_views: Query<&InputView<InputBinding>>,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut Velocity,
        &mut PlayerControl,
        &mut MoveController,
    )>,
    player_movement_settings: Res<PlayerMovementSettings>,
    //rapier_context: Res<RapierContext>,
) {
    let mut movement_values = [0.0, 0.0];
    let mut num_participating = 0;
    //let mut is_jumping = false;
    for input_view in input_views.iter() {
        let mut is_participating = false;
        for (movement_value, key) in movement_values
            .iter_mut()
            .zip([InputBinding::MoveHorizontal, InputBinding::MoveVertical])
        {
            for axis_value in input_view.axis(&key) {
                if !axis_value.released() {
                    is_participating = true;
                    *movement_value += axis_value.value
                }
            }
        }
        if is_participating {
            num_participating += 1;
        }
    }
    let mut target_speed = if 0 < num_participating {
        Vec2::from(movement_values) / num_participating as f32
    } else {
        Vec2::ZERO
    };
    if 1.0 < target_speed.length_squared() {
        target_speed = target_speed.normalize();
    }

    for (player_entity, mut transform, mut velocity, _player_control, mut move_controller) in
        query.iter_mut()
    {
        move_controller.target_speed = target_speed;
    }
}
