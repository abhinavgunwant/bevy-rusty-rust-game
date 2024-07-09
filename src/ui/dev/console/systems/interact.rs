use bevy::{
    app::AppExit, input::{ keyboard::{ Key, KeyboardInput }, mouse::{MouseScrollUnit, MouseWheel}, ButtonState }, prelude::*
};

use crate::{
    game::item::events::{ SpawnItemEvent, ItemToSpawn },
    ui::dev::console::{
        components::{ConsoleHistory, ConsoleTextLine},
        events::ConsoleCommandEvent,
    },
    lang::{ parser::Parser, types::{ AST, Literal } },
};

/// Gets f32 vector from the command parameter literals
pub fn get_f32_param_values(params: Vec<Literal>) -> Result<Vec<f32>, String> {
    let mut result = vec![];

    for (indx, param) in params.iter().enumerate() {
        match param {
            Literal::Number(num) => { result.push(num.clone()); }
            _ => {
                return Err(format!(
                    "Unexpected value, expecting number at position: {}",
                    indx,
                ));
            }
        }
    }

    Ok(result)
}

pub fn interpret(
    source: String,
    exit_event_writer: &mut EventWriter<AppExit>,
    spawn_item_ew: &mut EventWriter<SpawnItemEvent>,
) {
    let parser = Parser::new(source);
    let ast = parser.parse();

    println!("interpreting {}", ast);

    match ast {
        AST::Command(command_ast) => {
            println!("interpreting command");
            match command_ast.name.as_str() {
                "spawn" => {
                    println!("Behold! the spawn command!");
                    if command_ast.identifier.eq("box") {
                        if command_ast.parameters.is_empty() {
                            spawn_item_ew.send(SpawnItemEvent(
                                ItemToSpawn::GeometryCube(
                                    0.0, 0.0, 0.0, 1.0, 1.0, 1.0
                                )
                            ));
                        } else if command_ast.parameters.len() == 3 {
                            let mut error = false;

                            let params = match get_f32_param_values(
                                command_ast.parameters
                            ) {
                                Ok(p) => p,
                                Err(err_text) => {
                                    println!("> spawn -> box: {}", err_text);
                                    error = true;
                                    vec![]
                                }
                            };

                            if !error {
                                spawn_item_ew.send(SpawnItemEvent(
                                    ItemToSpawn::GeometryCube(
                                        params[0],
                                        params[1],
                                        params[2],
                                        1.0,
                                        1.0,
                                        1.0
                                    )
                                ));
                            }
                        } else if command_ast.parameters.len() == 6 {
                            let mut error = false;
                            let params = match get_f32_param_values(
                                command_ast.parameters
                            ) {
                                Ok(p) => p,
                                Err(err_text) => {
                                    println!("> spawn -> box: {}", err_text);
                                    error = true;
                                    vec![]
                                }
                            };

                            if !error {
                                spawn_item_ew.send(SpawnItemEvent(
                                    ItemToSpawn::GeometryCube(
                                        params[0],
                                        params[1],
                                        params[2],
                                        params[3],
                                        params[4],
                                        params[5],
                                    )
                                ));
                            }
                        } else {
                            println!("> spawn -> box: unexpected number of parameters");
                        }
                    }
                }

                "quit" => {
                    exit_event_writer.send(AppExit);
                }

                _ => {
                    println!("> Unknown command: {}", command_ast.name);
                }
            }
        }

        AST::Assignment(assignment_ast) => {
            // TODO: Implement!
        }

        AST::None => {},
    }
}

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

/// Console command processing system
pub fn process_command(
    mut command_er: EventReader<ConsoleCommandEvent>,
    mut exit_event_writer: EventWriter<AppExit>,
    mut history_query: Query<
        (&mut Text, &mut ConsoleHistory, &mut Style),
        With<ConsoleHistory>
    >,
    mut spawn_item_ew: EventWriter<SpawnItemEvent>,
) {
    for e in command_er.read() {
        interpret(
            e.command.clone(), &mut exit_event_writer, &mut spawn_item_ew,
        );

        if let Ok((mut text, mut console_text_history, mut style)) = history_query.get_single_mut() {
            console_text_history.text_vec.push(e.command.clone());
            text.sections.push(TextSection::new(
                format!("\n{}", e.command),
                TextStyle { font_size: 20.0, ..default() },
            ));

            if console_text_history.text_vec.len() > 12 {
                console_text_history.position -= 20.0;
                style.top = Val::Px(console_text_history.position);
            }
        }
    }
}

pub fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut history_query: Query<
        (&mut ConsoleHistory, &mut Style, &Parent, &Node),
        With<ConsoleHistory>
    >,
    query_node: Query<&Node>,
) {
    for e in mouse_wheel_events.read() {
        for (mut console_history, mut style, parent, node) in &mut history_query {
            let dy = match e.unit {
                MouseScrollUnit::Line => e.y * 80.0,
                MouseScrollUnit::Pixel => e.y,
            };

            let node_height = node.size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (node_height - container_height).max(0.0);

            console_history.position += dy;
            console_history.position = console_history
                .position.clamp(-max_scroll, 0.0);

            style.top = Val::Px(console_history.position);
        }
    }
}

