pub mod fps;
pub mod systems;
pub mod dev;

use bevy::prelude::*;
use fps::FpsPlugin;
use dev::DevPlugin;

use self::systems::spawn_crosshair;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(FpsPlugin)
            .add_plugins(DevPlugin)
            .add_systems(Startup, spawn_crosshair);
    }
}
