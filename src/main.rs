mod systems;
mod ui;
mod game;

use bevy::{
    prelude::*, window::PresentMode, diagnostic::FrameTimeDiagnosticsPlugin,
};
use bevy_rapier3d::prelude::*;
use game::item::events::SpawnItemEvent;

use crate::{ 
    systems::setup, game::GamePlugin,
    ui::{ UIPlugin, dev::console::events::ConsoleCommandEvent },
};

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "Rusty Rust".into(),
            present_mode: PresentMode::Immediate,
            ..default()
        }),
        ..default()
    };

    let mut rapier_config = RapierConfiguration::new(0.0);
    rapier_config.gravity = Vect::new(0.0, 0.0, -9.8);

    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(window_plugin))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(GamePlugin)
        .add_plugins(UIPlugin)
        .add_systems(Startup, setup)
        .add_event::<ConsoleCommandEvent>()
        .add_event::<SpawnItemEvent>()
        .insert_resource(rapier_config);

    #[cfg(debug_assertions)]
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.run();
}

