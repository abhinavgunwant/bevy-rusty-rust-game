use bevy::prelude::*;

use std::{fs::{ metadata, File, write, create_dir }, io::Read};

use crate::ui::dev::console::{
    components::{ Console, ConsoleHistory, ConsoleTextLine, Scrollable },
    CONSOLE_HEIGHT, CONSOLE_TEXT_LINE_HEIGHT, CONSOLE_TITLE_HEIGHT,
};

pub fn spawn_console(mut commands: Commands) {
    let console_backdrop_style = Style {
        width: Val::Percent(100.0),
        height: Val::Px(CONSOLE_HEIGHT),
        flex_direction: FlexDirection::Column,
        ..default()
    };

    let mut console_history_style = console_backdrop_style.clone();
    console_history_style.height = Val::Px(
        CONSOLE_HEIGHT - CONSOLE_TEXT_LINE_HEIGHT - CONSOLE_TITLE_HEIGHT
    );
    console_history_style.overflow = Overflow::clip_y();

    let mut history_lines: Vec<String> = vec![];

    match metadata("logs/console.log") {
        Ok(_) => {
            match File::open("logs/console.log") {
                Ok(mut file) => {
                    let mut content: String = String::default();

                    match file.read_to_string(&mut content) {
                        Ok(_) => {
                            history_lines = content.lines()
                                .map(|s| s.to_string()).collect();
                        }
                        Err(_) => {}
                    };
                }
                Err(_) => {
                    println!("Could not read logs file");
                }
            }
        }

        Err(_) => {
            println!("Could not open logs file");
        }
    }

    commands.spawn((
        NodeBundle {
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
            z_index: ZIndex::Global(i32::MAX - 1),
            style: console_backdrop_style,
            ..default()
        },
        Console,
    )).with_children(|parent| {
        let style = TextStyle {
            font_size: 16.0,
            color: Color::WHITE,
            ..default()
        };

        let title_style = Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            height: Val::Px(CONSOLE_TITLE_HEIGHT),
            ..default()
        };

        parent.spawn(NodeBundle {
            background_color: Color::rgba(0.0, 0.0, 0.0, 1.0).into(),
            z_index: ZIndex::Global(i32::MAX),
            style: title_style,
            ..default()
        }).with_children(|title| {
            title.spawn((
                TextBundle {
                    text: Text::from_sections([
                        TextSection {
                            value: "Console".into(),
                            style: style.clone(),
                        },
                    ]),
                    ..default()
                },
            ));
        });

        // Command history
        parent.spawn((
            NodeBundle {
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.9).into(),
                z_index: ZIndex::Global(i32::MAX - 1),
                style: console_history_style,
                ..default()
            },
            Scrollable { pos: 0.0 },
        )).with_children(|history| {
            let position_top;

            if history_lines.len() > 12 {
                position_top = -20.0 * (history_lines.len() as f32 - 12.0);
            } else {
                position_top = 0.0;
            }

            let text_sections = history_lines.clone().into_iter().map(
                |line| TextSection {
                    value: format!("\n{}", line),
                    style: TextStyle {
                        font_size: 20.0,
                        ..default()
                    }
                }
            ).collect();

            history.spawn((
                TextBundle {
                    text: Text {
                        sections: text_sections,
                        ..default()
                    },
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        top: Val::Px(position_top),
                        ..default()
                    },
                    ..default()
                },
                ConsoleHistory {
                    text_vec: history_lines.clone(),
                    position: position_top,
                },
            ));
        });

        // Command prompt
        parent.spawn((
            NodeBundle {
                background_color: Color::rgba(0.0, 0.0, 0.0, 1.0).into(),
                z_index: ZIndex::Global(i32::MAX - 1),
                style: Style::default(),
                ..default()
            },
        )).with_children(|text_line| {
            let text_style = TextStyle {
                font_size: 24.0,
                color: Color::WHITE,
                ..default()
            };

            text_line.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new("", text_style.clone()),
                            TextSection::new("|", text_style),
                        ],
                        ..default()
                    },
                    ..default()
                },
                ConsoleTextLine { text: String::default() },
            ));
        });
    });
}

pub fn despawn_console(
    mut commands: Commands,
    console_query: Query<Entity, With<Console>>,
    console_hist_query: Query<&mut ConsoleHistory, With<ConsoleHistory>>,
) {
    if let Ok(console_history) = console_hist_query.get_single() {
        match metadata("logs/console.log") {
            Ok(_) => {}
            Err(_) => {
                match create_dir("logs") {
                    Ok(_) => {
                        println!("Created logs directory");
                    }
                    Err(e) => {
                        println!("Could not create logs directory: {}", e);
                    }
                }
            }
        }

        match write("logs/console.log", console_history.text_vec.join("\n")) {
            Ok(_) => {}
            Err(e) => {
                println!("could not write console log to file: {}", e);
            }
        };
    }

    if let Ok(console_entity) = console_query.get_single() {
        commands.entity(console_entity).despawn_recursive();
    }
}

