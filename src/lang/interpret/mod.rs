pub mod utils;

use bevy::{ app::AppExit, prelude::* };

use crate::{
    game::item::events::{ SpawnItemEvent, ItemToSpawn },
    lang::{
        parser::Parser, types::AST,
        interpret::utils::{
            get_f32_param_values, get_string_param_values,
            get_string_param_value, get_f32_param_value,
        },
    },
};

pub fn interpret(
    source: String,
    exit_event_writer: &mut EventWriter<AppExit>,
    spawn_item_ew: &mut EventWriter<SpawnItemEvent>,
) {
    let parser = Parser::new(source);
    let ast = parser.parse();

    // println!("interpreting {}", ast);

    match ast {
        AST::Command(command_ast) => {
            match command_ast.name.as_str() {
                "spawn" => {
                    match command_ast.identifier.as_str() {
                        "box" => {
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

                        "sphere" => {
                            if command_ast.parameters.is_empty() {
                                spawn_item_ew.send(SpawnItemEvent(
                                    ItemToSpawn::GeometrySphere(
                                        0.0, 0.0, 0.0, 1.0
                                    )
                                ));
                            } else if command_ast.parameters.len() == 3 {
                                let mut error = false;

                                let params = match get_f32_param_values(
                                    command_ast.parameters
                                ) {
                                    Ok(p) => p,
                                    Err(err_text) => {
                                        println!("> spawn -> sphere: {}", err_text);
                                        error = true;
                                        vec![]
                                    }
                                };

                                if !error {
                                    spawn_item_ew.send(SpawnItemEvent(
                                        ItemToSpawn::GeometrySphere(
                                            params[0],
                                            params[1],
                                            params[2],
                                            1.0,
                                        )
                                    ));
                                }
                            } else if command_ast.parameters.len() == 4 {
                                let mut error = false;

                                let params = match get_f32_param_values(
                                    command_ast.parameters
                                ) {
                                    Ok(p) => p,
                                    Err(err_text) => {
                                        println!("> spawn -> sphere: {}", err_text);
                                        error = true;
                                        vec![]
                                    }
                                };

                                if !error {
                                    spawn_item_ew.send(SpawnItemEvent(
                                        ItemToSpawn::GeometrySphere(
                                            params[0],
                                            params[1],
                                            params[2],
                                            params[3],
                                        )
                                    ));
                                }
                            } else {
                                println!("> spawn -> sphere: unexpected number of parameters");
                            }
                        }

                        "file" => {
                            if command_ast.parameters.len() == 1 {
                                let mut error = false;

                                let params = match get_string_param_values(
                                    command_ast.parameters
                                ) {
                                    Ok(p) => p,
                                    Err(err_text) => {
                                        println!("> spawn -> file: {}", err_text);
                                        error = true;
                                        vec![]
                                    }
                                };

                                if !error {
                                    spawn_item_ew.send(SpawnItemEvent(
                                        ItemToSpawn::File(
                                            0.0, 0.0, 0.0,
                                            params[0].clone()
                                        )
                                    ));
                                }
                            } else if command_ast.parameters.len() == 4 {
                                let mut error = false;

                                let param0 = match get_f32_param_value(
                                    &command_ast.parameters,
                                    0
                                ) {
                                    Ok(p) => p,
                                    Err(err_text) => {
                                        println!("> spawn -> file: {}", err_text);
                                        error = true;
                                        0.0
                                    }
                                };

                                let param1 = match get_f32_param_value(
                                    &command_ast.parameters,
                                    1
                                ) {
                                    Ok(p) => p,
                                    Err(err_text) => {
                                        println!("> spawn -> file: {}", err_text);
                                        error = true;
                                        0.0
                                    }
                                };

                                let param2 = match get_f32_param_value(
                                    &command_ast.parameters,
                                    2
                                ) {
                                    Ok(p) => p,
                                    Err(err_text) => {
                                        println!("> spawn -> file: {}", err_text);
                                        error = true;
                                        0.0
                                    }
                                };

                                let param3 = match get_string_param_value(
                                    &command_ast.parameters,
                                    3
                                ) {
                                    Ok(p) => p,
                                    Err(err_text) => {
                                        println!("> spawn -> file: {}", err_text);
                                        error = true;
                                        String::default()
                                    }
                                };

                                if !error {
                                    spawn_item_ew.send(SpawnItemEvent(
                                        ItemToSpawn::File(
                                            param0,
                                            param1,
                                            param2,
                                            param3,
                                        )
                                    ));
                                }
                            } else {
                                println!("> spawn -> file: file name not present");
                            }
                        }

                        _ => {
                            println!(
                                "> spawn: invalid primitive \"{}\"",
                                command_ast.identifier
                            );
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

        AST::Assignment(_assignment_ast) => {
            // TODO: Implement!
        }

        AST::None => {},
    }
}

