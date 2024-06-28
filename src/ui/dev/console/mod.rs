/// Developer console module
pub mod systems;
pub mod components;
pub mod events;

use bevy::prelude::*;
use systems::{
    toggle::toggle_console,
    interact::{ input_text, process_command },
    layout::{ spawn_console, despawn_console },
};

pub struct ConsolePlugin;

const CONSOLE_HEIGHT: f32 = 300.0;
const CONSOLE_TITLE_HEIGHT: f32 = 16.0;
const CONSOLE_TEXT_LINE_HEIGHT: f32 = 24.0;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<ConsoleToggleState>()
            .add_systems(
                Update,
                (
                    toggle_console,
                    (input_text, process_command).run_if(
                        in_state(ConsoleToggleState::Shown)
                    )
                )
            )
            .add_systems(OnEnter(ConsoleToggleState::Shown), spawn_console)
            .add_systems(OnExit(ConsoleToggleState::Shown), despawn_console);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum ConsoleToggleState {
    #[default]
    Shown,
    Hidden,
}

