use bevy::{prelude::*, render::{mesh::VertexAttributeValues, render_asset::RenderAssetUsages}};
use bevy_rapier3d::prelude::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(400.0, 400.0, 0.1)),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0.0, 0.0, -0.05),
            ..default()
        },
        Collider::cuboid(400.0, 400.0, 0.1)
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 800.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Draw x-axis
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::new(
            bevy::render::mesh::PrimitiveTopology::LineList,
            RenderAssetUsages::RENDER_WORLD,
        ).with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(vec![[-10.0, 0.0, 0.0], [10.0, 0.0, 0.0]]),
        )),
        material: materials.add(Color::RED),
        ..default()
    });

    // Draw y-axis
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::new(
            bevy::render::mesh::PrimitiveTopology::LineList,
            RenderAssetUsages::RENDER_WORLD,
        ).with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(vec![[0.0, -10.0, 0.0], [0.0, 10.0, 0.0]]),
        )),
        material: materials.add(Color::GREEN),
        ..default()
    });

    // Draw z-axis
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::new(
            bevy::render::mesh::PrimitiveTopology::LineList,
            RenderAssetUsages::RENDER_WORLD,
        ).with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(vec![[0.0, 0.0, -10.0], [0.0, 0.0, 10.0]]),
        )),
        material: materials.add(Color::BLUE),
        ..default()
    });
}

