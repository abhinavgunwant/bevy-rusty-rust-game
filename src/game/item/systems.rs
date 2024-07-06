use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::{ Rng, thread_rng };

use super::{components::GeometryCube, events::{ItemToSpawn, SpawnItemEvent}};

fn random_color() -> Color {
    let mut rng = thread_rng();

    Color::rgb(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
}

pub fn spawn_item(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_item_event: EventReader<SpawnItemEvent>,
) {
    for e in spawn_item_event.read() {
        match e.0 {
            ItemToSpawn::GeometryCube(x, y, z, width, depth, height) => {
                commands.spawn((
                    GeometryCube,
                    PbrBundle {
                        mesh: meshes.add(Cuboid::new(
                            width.to_owned(),
                            depth.to_owned(),
                            height.to_owned()
                        )),
                        material: materials.add(random_color()),
                        transform: Transform::from_xyz(
                            x.to_owned(),
                            y.to_owned(),
                            z.to_owned()
                        ),
                        ..default()
                    },
                    RigidBody::Dynamic,
                    Collider::cuboid(
                        width.to_owned()/2.0,
                        depth.to_owned()/2.0,
                        height.to_owned()/2.0,
                    ),
                    Restitution::coefficient(0.6),
                ));
            }
        }
    }
}

