use bevy::prelude::*;

#[derive(Event)]
pub struct ConsoleCommandEvent {
    pub command: String,
}

