use bevy::prelude::*;

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
            history.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![],
                        ..default()
                    },
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                },
                ConsoleHistory { text_vec: vec![], position: 0.0 },
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
    console_query: Query<Entity, With<Console>>
) {
    if let Ok(console_entity) = console_query.get_single() {
        commands.entity(console_entity).despawn_recursive();
    }

    // TODO: store the command history in a file or a Local<...> here...
}

