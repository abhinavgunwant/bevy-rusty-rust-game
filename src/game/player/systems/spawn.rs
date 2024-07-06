use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::game::player::components::{ Player, PlayerCamera };

pub fn spawn_player(mut commands: Commands) {
    let transform_camera = Transform::from_xyz(0.0, 0.0, 0.0)
        .looking_at(Vec3::new(0.0, 0.0, 1.71), Vec3::Z);

    commands.spawn((
        Player::default(),
        SpatialBundle {
            transform: Transform::from_xyz(0.0, -5.0, 0.0),
            ..default()
        },
        Collider::cuboid(0.125, 0.125, 1.71),
        RigidBody::Dynamic,
    ))
    .with_children(|player|{
        player.spawn((
            PlayerCamera,
            Camera3dBundle { transform: transform_camera, ..default() },
        ));
    });
}

