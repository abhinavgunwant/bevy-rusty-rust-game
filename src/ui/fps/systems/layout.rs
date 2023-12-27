use bevy::prelude::*;

use crate::ui::fps::components::{ FpsRoot, FpsText };

pub fn spawn_fps_counter(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
            z_index: ZIndex::Global(i32::MAX),
            style: Style::default(),
            ..default()
        },
        FpsRoot {},
    )).with_children(|parent| {
        let style = TextStyle {
            font_size: 16.0,
            color: Color::WHITE,
            ..default()
        };

        parent.spawn((
            TextBundle {
                text: Text::from_sections([
                    TextSection {
                        value: "FPS: ".into(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "N/A".into(),
                        style,
                    }
                ]),
                ..default()
            },
            FpsText {},
        ));
    });
}

