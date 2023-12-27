pub mod systems;
pub mod components;

use bevy::{ prelude::*, time::Fixed };
use systems::{ layout::spawn_fps_counter, update::update_fps_text };

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_fps_counter)
            .add_systems(FixedUpdate, update_fps_text)
            .insert_resource(Time::<Fixed>::from_seconds(1.0));
    }
}

