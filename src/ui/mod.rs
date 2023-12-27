pub mod fps;

use bevy::prelude::*;
use fps::FpsPlugin;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FpsPlugin);
    }
}
