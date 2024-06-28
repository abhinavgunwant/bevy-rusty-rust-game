use bevy::prelude::*;

#[derive(Component)]
pub struct Console;

#[derive(Component)]
pub struct ConsoleTextLine {
    pub text: String,
}

#[derive(Component)]
pub struct ConsoleHistory {
    pub text_vec: Vec<String>,
}

#[derive(Component)]
pub struct Scrollable {
    /// scroll offset position
    pub pos: f32,
}

