use std::f32::consts::FRAC_PI_2;

use bevy::{ prelude::*, input::mouse::MouseMotion };

use crate::game::player::{ *, components::Player };

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut mouse_er: EventReader<MouseMotion>,
    mut player_query: Query<(&mut Transform, &mut Player), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut player)) = player_query.get_single_mut() {
        let mut sprint = false;
        let mut direction = Vec3::ZERO;

        let yaw_quat = Quat::from_axis_angle(Vec3::Z, player.yaw - FRAC_PI_2);

        if keyboard_input.pressed(KeyCode::Left)
            || keyboard_input.pressed(KeyCode::A) {
            direction += yaw_quat * -Vec3::Y;
        }

        if keyboard_input.pressed(KeyCode::Right)
            || keyboard_input.pressed(KeyCode::D) {
            direction += yaw_quat * Vec3::Y;
        }

        if keyboard_input.pressed(KeyCode::Up)
            || keyboard_input.pressed(KeyCode::W)
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

        if keyboard_input.pressed(KeyCode::Down)
            || keyboard_input.pressed(KeyCode::S) {
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

        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            sprint = true;
        }

        // let mut camera_angle_changed = false;

        for mouse in mouse_er.read() {
            player.yaw -= FRAC_PI_2 * MOUSE_SENSITIVITY * mouse.delta.x;
            player.pitch -= FRAC_PI_2 * MOUSE_SENSITIVITY * mouse.delta.y;
            player.pitch = player.pitch.clamp(-FRAC_PI_2, FRAC_PI_2);

            // camera_angle_changed = true;

            /*
            if mouse_motion.delta.x != 0.0 || mouse_motion.delta.y != 0.0 {
                println!(
                    "Mouse moved: X: {} px, Y: {} px",
                    mouse_motion.delta.x,
                    mouse_motion.delta.y
                );
            }
            */
        }

        /*
        if camera_angle_changed {
            println!(
                "Player rotation angle: yaw: {} radians, pitch: {} radians",
                player.yaw,
                player.pitch,
            );
        }
        */

        if sprint {
            transform.translation += direction * PLAYER_RUNNING_SPEED * time.delta_seconds();
        } else {
            transform.translation += direction * PLAYER_WALKING_SPEED * time.delta_seconds();
        }

        transform.rotation = Quat::from_axis_angle(Vec3::Z, player.yaw)
            * Quat::from_axis_angle(Vec3::X, player.pitch);
    }
}

