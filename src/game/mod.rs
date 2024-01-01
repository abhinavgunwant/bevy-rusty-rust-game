pub mod player;

use bevy::prelude::*;
use crate::game::player::PlayerPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin);
    }
}

