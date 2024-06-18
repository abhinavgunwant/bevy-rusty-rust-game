/// Module for the developer mode
pub mod console;

use bevy::prelude::*;
use console::ConsolePlugin;

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ConsolePlugin);
    }
}
