use bevy::{
    prelude::*,
    app::AppExit,
    input::{ keyboard::{ Key, KeyboardInput }, ButtonState },
};

use crate::ui::dev::console::{
    components::{ConsoleHistory, ConsoleTextLine}, events::ConsoleCommandEvent,
};

pub fn input_text(
    mut text_query: Query<
        (&mut Text, &mut ConsoleTextLine),
        With<ConsoleTextLine>
    >,
    mut keyboard_er: EventReader<KeyboardInput>,
    mut command_ew: EventWriter<ConsoleCommandEvent>,
) {
    if let Ok(
        (mut text, mut console_text_line)
    ) = text_query.get_single_mut() {
        for e in keyboard_er.read() {
            if e.state == ButtonState::Released {
                continue;
            }

            match &e.logical_key {
                Key::Character(input) => {
                    if input.chars().any(|c| c.is_control() || c == '`') {
                        continue;
                    }

                    console_text_line.text.push_str(&input);
                    // println!("{} pressed: {}", input, console_text_line.text);
                }

                Key::Backspace => {
                    console_text_line.text.pop();
                }

                Key::Space => {
                    console_text_line.text.push(' ');
                }

                // Interpret the command here
                Key::Enter => {
                    command_ew.send(ConsoleCommandEvent {
                        command: console_text_line.text.clone()
                    });
                    console_text_line.text = String::default();
                }

                _ => {}
            }
        }

        text.sections[0].value = console_text_line.text.clone();
    }
}

/// Console command processing happens here...
pub fn process_command(
    mut command_er: EventReader<ConsoleCommandEvent>,
    mut exit_event_writer: EventWriter<AppExit>,
    mut history_query: Query<
        (&mut Text, &mut ConsoleHistory),
        With<ConsoleHistory>
    >,
) {
    for e in command_er.read() {
        if e.command.eq("quit") || e.command.eq("exit") {
            exit_event_writer.send(AppExit);
        }

        let command_pairs = e.command.split(" ").collect::<Vec<&str>>();

        if command_pairs.len() == 2 {
            if command_pairs[0].eq("spawn") {
                println!("Spawning {}", command_pairs[1]);
            }
        }

        if let Ok((mut text, mut console_text_history)) = history_query.get_single_mut() {
            console_text_history.text_vec.push(e.command.clone());
            text.sections.push(TextSection::new(
                format!("\n{}", e.command),
                TextStyle { font_size: 20.0, ..default() },
            ));
        }
    }
}

