use bevy::prelude::*;

pub enum ItemToSpawn {
    /// Spawn a cube (x, y, z, width, depth, height)
    GeometryCube(f32, f32, f32, f32, f32, f32),
}

#[derive(Event)]
pub struct SpawnItemEvent(pub ItemToSpawn);

