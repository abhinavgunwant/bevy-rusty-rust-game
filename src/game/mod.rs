pub mod player;
pub mod item;

use bevy::prelude::*;
use crate::game::{ player::PlayerPlugin, item::ItemPlugin };

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin)
            .add_plugins(ItemPlugin);
    }
}

