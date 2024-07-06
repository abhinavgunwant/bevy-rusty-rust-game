use bevy::prelude::*;
use systems::spawn_item;

pub mod components;
pub mod events;
pub mod systems;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_item);
    }
}

