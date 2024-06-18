pub mod systems;
pub mod components;

use bevy::prelude::*;

use crate::{game::player::systems::{
    movement::move_player, spawn::spawn_player
}, ui::dev::console::ConsoleToggleState};

const PLAYER_WALKING_SPEED: f32 = 1.42;
const PLAYER_RUNNING_SPEED: f32 = 4.16;

// TODO: make a simpler version so that 1.0 sensitivity = 0.00030 and memoize
// that somewhere...
const MOUSE_SENSITIVITY: f32 = 0.00030;
const NOCLIP: bool = false;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                move_player.run_if(in_state(ConsoleToggleState::Hidden))
            );
    }
}

