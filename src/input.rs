use bevy::prelude::*;
use ezinput::prelude::*;

use crate::global_types::InputBinding;

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EZInputPlugin::<InputBinding>::default());
        app.init_resource::<InputConfig>();
        app.add_startup_system(setup_keyboard_input);
        app.add_system(handle_gamepad_events);
    }
}

struct InputConfig(InputView<InputBinding>);

impl Default for InputConfig {
    fn default() -> Self {
        let mut view = InputView::new();

        for (key, axes, negatives, positives) in [
            (
                InputBinding::MoveHorizontal,
                [
                    InputReceiver::GamepadAxis(GamepadAxisType::LeftStickX),
                    InputReceiver::GamepadAxis(GamepadAxisType::DPadX),
                ],
                [
                    InputReceiver::KeyboardKey(KeyCode::Left),
                    InputReceiver::KeyboardKey(KeyCode::A),
                    InputReceiver::GamepadButton(GamepadButtonType::DPadLeft),
                ],
                [
                    InputReceiver::KeyboardKey(KeyCode::Right),
                    InputReceiver::KeyboardKey(KeyCode::D),
                    InputReceiver::GamepadButton(GamepadButtonType::DPadRight),
                ],
            ),
            (
                InputBinding::MoveVertical,
                [
                    InputReceiver::GamepadAxis(GamepadAxisType::LeftStickY),
                    InputReceiver::GamepadAxis(GamepadAxisType::DPadY),
                ],
                [
                    InputReceiver::KeyboardKey(KeyCode::Down),
                    InputReceiver::KeyboardKey(KeyCode::S),
                    InputReceiver::GamepadButton(GamepadButtonType::DPadDown),
                ],
                [
                    InputReceiver::KeyboardKey(KeyCode::Up),
                    InputReceiver::KeyboardKey(KeyCode::W),
                    InputReceiver::GamepadButton(GamepadButtonType::DPadUp),
                ],
            ),
        ] {
            let mut binding = ActionBinding::from(key);
            for axis in axes {
                binding.receivers(axis.into());
            }
            for input in negatives {
                binding
                    .receivers(input.into())
                    .default_axis_value(input, -1.0);
            }
            for input in positives {
                binding
                    .receivers(input.into())
                    .default_axis_value(input, 1.0);
            }
            view.add_binding(&mut binding);
        }

        view.add_binding(&mut {
            let mut binding = ActionBinding::from(InputBinding::Grab);
            binding.receivers(InputReceiver::KeyboardKey(KeyCode::Space).into());
            binding.receivers(InputReceiver::GamepadButton(GamepadButtonType::South).into());
            binding
        });

        Self(view)
    }
}

fn setup_keyboard_input(mut commands: Commands, input_config: Res<InputConfig>) {
    commands
        .spawn()
        .insert(input_config.0.clone())
        .insert(KeyboardMarker);
}

fn handle_gamepad_events(
    mut reader: EventReader<GamepadEvent>,
    gamepad_services: Query<(Entity, &GamepadMarker), With<InputView<InputBinding>>>,
    mut commands: Commands,
    input_config: Res<InputConfig>,
) {
    for GamepadEvent(gamepad, event_type) in reader.iter() {
        match event_type {
            GamepadEventType::Connected => {
                if !gamepad_services
                    .iter()
                    .any(|(_, service)| service.gamepad == *gamepad)
                {
                    commands
                        .spawn()
                        .insert(input_config.0.clone())
                        .insert(GamepadMarker::with_dead_zone(gamepad.0, (0.25, 0.25)));
                }
            }
            GamepadEventType::Disconnected => {
                for (entity, service) in gamepad_services.iter() {
                    if service.gamepad == *gamepad {
                        commands.entity(entity).despawn();
                    }
                }
            }
            _ => {}
        }
    }
}
