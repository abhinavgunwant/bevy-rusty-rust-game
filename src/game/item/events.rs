use bevy::prelude::*;

pub enum ItemToSpawn {
    /// Spawn a cube (x, y, z, width, depth, height)
    GeometryCube(f32, f32, f32, f32, f32, f32),

    /// Spawn a sphere with a radius (x, y, z, radius)
    GeometrySphere(f32, f32, f32, f32),

    /// GLB file
    File(f32, f32, f32, String)
}

#[derive(Event)]
pub struct SpawnItemEvent(pub ItemToSpawn);

