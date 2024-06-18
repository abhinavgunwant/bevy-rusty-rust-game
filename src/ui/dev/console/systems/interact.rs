use bevy::{
    prelude::*,
    app::AppExit,
    input::{ keyboard::{ Key, KeyboardInput }, ButtonState },
};

use crate::ui::dev::console::{components::{ConsoleHistory, ConsoleTextLine}, ConsoleState};

pub fn input_text(
    mut text_query: Query<&mut Text, With<ConsoleTextLine>>,
    mut keyboard_er: EventReader<KeyboardInput>,
    mut console_state: Local<ConsoleState>,
    mut exit_event_writer: EventWriter<AppExit>,
) {
    for e in keyboard_er.read() {
        if e.state == ButtonState::Released {
            continue;
        }

        match &e.logical_key {
            Key::Character(input) => {
                if input.chars().any(|c| c.is_control() || c == '`') {
                    continue;
                }

                console_state.text.push_str(&input);
            }

            Key::Backspace => {
                console_state.text.pop();
            }

            Key::Space => {
                console_state.text.push(' ');
            }

            // Interpret the command here
            Key::Enter => {
                if console_state.text.eq("quit") || console_state.text.eq("exit") {
                    exit_event_writer.send(AppExit);
                }

                let command_pairs = console_state.text.split(" ").collect::<Vec<&str>>();

                if command_pairs.len() == 2 {
                    if command_pairs[0].eq("spawn") {
                        println!("Spawning {}", command_pairs[1]);
                    }
                }


                // Finally, when command has been processed:
                let console_txt = console_state.text.clone();
                console_state.history.push(console_txt);
                console_state.text = String::default();
            }

            _ => {}
        }
    }

    if let Ok(mut text) = text_query.get_single_mut() {
        text.sections[0].value = console_state.text.clone();
    }
}

