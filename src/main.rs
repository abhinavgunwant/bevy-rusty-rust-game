mod systems;
mod ui;

use bevy::{
    prelude::*, window::PresentMode, diagnostic::FrameTimeDiagnosticsPlugin,
};

use ui::UIPlugin;
use systems::setup;

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "Rusty Rust".into(),
            present_mode: PresentMode::Immediate,
            ..default()
        }),
        ..default()
    };

    App::new()
        .add_plugins(DefaultPlugins.set(window_plugin))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(UIPlugin)
        .add_systems(Startup, setup)
        .run();
}

