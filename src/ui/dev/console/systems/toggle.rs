use bevy::prelude::*;

use super::super::ConsoleToggleState;

pub fn toggle_console(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    console_state: Res<State<ConsoleToggleState>>,
) {
    if keyboard_input.just_released(KeyCode::Backquote) {
        match console_state.get() {
            ConsoleToggleState::Shown => {
                commands.insert_resource(
                    NextState(Some(ConsoleToggleState::Hidden))
                );
            }

            ConsoleToggleState::Hidden => {
                commands.insert_resource(
                    NextState(Some(ConsoleToggleState::Shown))
                );
            }
        }
    }
}

