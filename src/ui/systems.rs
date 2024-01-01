use bevy::prelude::*;

pub fn spawn_crosshair(
    mut commands: Commands,
) {
    commands.spawn(NodeBundle {
        background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
        z_index: ZIndex::Global(i32::MAX),
        style: Style {
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        parent.spawn(NodeBundle {
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
            z_index: ZIndex::Global(i32::MAX),
            style: Style {
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                column_gap: Val::Px(4.0),
                ..default()
            },
            ..default()
        }).with_children(|parent| {
            let node = NodeBundle {
                background_color: Color::rgba(255.0, 100.0, 100.0, 1.0).into(),
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    height: Val::Px(1.0),
                    width: Val::Px(5.0),
                    ..default()
                },
                ..default()
            };

            parent.spawn(node.clone());
            parent.spawn(node);
        });
    });

    commands.spawn(NodeBundle {
        background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
        z_index: ZIndex::Global(i32::MAX),
        style: Style {
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        parent.spawn(NodeBundle {
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
            z_index: ZIndex::Global(i32::MAX),
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                row_gap: Val::Px(4.0),
                ..default()
            },
            ..default()
        }).with_children(|parent| {
            let node = NodeBundle {
                background_color: Color::rgba(255.0, 100.0, 100.0, 1.0).into(),
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    height: Val::Px(5.0),
                    width: Val::Px(1.0),
                    ..default()
                },
                ..default()
            };

            parent.spawn(node.clone());
            parent.spawn(node);
        });
    });
}

