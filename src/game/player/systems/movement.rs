use std::f32::consts::{ PI, FRAC_PI_2 };

use bevy::{ prelude::*, input::mouse::MouseMotion };
use components::MovementType;

use crate::game::player::{ *, components::Player };

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_er: EventReader<MouseMotion>,
    mut player_query: Query<(&mut Transform, &mut Player), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut player)) = player_query.get_single_mut(){
        let mut direction = Vec3::ZERO;

        let yaw_quat = Quat::from_axis_angle(Vec3::Z, player.yaw - FRAC_PI_2);

        if keyboard_input.pressed(KeyCode::ArrowLeft)
            || keyboard_input.pressed(KeyCode::KeyA) {
            direction += yaw_quat * -Vec3::Y;
        }

        if keyboard_input.pressed(KeyCode::ArrowRight)
            || keyboard_input.pressed(KeyCode::KeyD) {
            direction += yaw_quat * Vec3::Y;
        }

        if keyboard_input.pressed(KeyCode::ArrowUp)
            || keyboard_input.pressed(KeyCode::KeyW)
        {
            if NOCLIP {
                direction += yaw_quat * -Vec3::X + Vec3::new(
                    0.0,
                    0.0,
                    (Quat::from_axis_angle(Vec3::X, player.pitch) * -Vec3::Z).z
                );
            } else {
                direction += yaw_quat * -Vec3::X;
            }
        }

        if keyboard_input.pressed(KeyCode::ArrowDown)
            || keyboard_input.pressed(KeyCode::KeyS) {
            if NOCLIP {
                direction += yaw_quat * Vec3::X - Vec3::new(
                    0.0,
                    0.0,
                    (Quat::from_axis_angle(Vec3::X, player.pitch) * -Vec3::Z).z
                );
            } else {
                direction += yaw_quat * Vec3::X;
            }
        }

        let old_movement_type = player.movement_type.clone();

        if keyboard_input.pressed(KeyCode::ControlLeft) {
            player.movement_type = MovementType::Crouch;
        } else if keyboard_input.pressed(KeyCode::ShiftLeft) {
            player.movement_type = MovementType::Sprint;
        } else {
            player.movement_type = MovementType::Walk;
        }

        for mouse in mouse_er.read() {
            player.yaw -= FRAC_PI_2 * MOUSE_SENSITIVITY * mouse.delta.x;
            player.pitch -= FRAC_PI_2 * MOUSE_SENSITIVITY * mouse.delta.y;
            player.pitch = player.pitch.clamp(0.0, PI);
        }

        match player.movement_type {
            MovementType::Sprint => {
                transform.translation += direction * PLAYER_RUNNING_SPEED * time.delta_seconds();

                if old_movement_type == MovementType::Crouch {
                    transform.translation.z += PLAYER_CROUCH_DELTA;
                }
            }

            MovementType::Walk => {
                transform.translation += direction * PLAYER_WALKING_SPEED * time.delta_seconds();

                if old_movement_type == MovementType::Crouch {
                    transform.translation.z += PLAYER_CROUCH_DELTA;
                }
            }

            MovementType::Crouch => {
                transform.translation += direction * PLAYER_CROUCH_SPEED * time.delta_seconds();

                if old_movement_type != MovementType::Crouch {
                    transform.translation.z -= PLAYER_CROUCH_DELTA;
                }
            }
        }

        transform.rotation = Quat::from_axis_angle(Vec3::Z, player.yaw)
            * Quat::from_axis_angle(Vec3::X, player.pitch);
    }
}

