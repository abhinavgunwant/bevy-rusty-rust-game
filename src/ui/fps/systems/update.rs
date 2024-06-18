use bevy::{
    prelude::*,
    diagnostic::{ DiagnosticsStore, FrameTimeDiagnosticsPlugin },
};
use crate::ui::fps::components::FpsText;

pub fn update_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            text.sections[1].value = format!("{}", value as u16);
        } else {
            text.sections[1].value = "N/A".into();
        }
    }
}

