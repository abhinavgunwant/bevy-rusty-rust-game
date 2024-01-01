use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub id: u16,
    pub name: String,
    pub pitch: f32,
    pub yaw: f32,
    pub health: u8,
    pub food: u8,
    pub water: u8,
    pub poison: u8,
    pub bleeding: u8,
    pub radiation: u8,
    pub ore_boost: u8,
    pub health_boost: u8,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            id: 0,
            name: String::from("Unknown Player"),
            pitch: 0.0,
            yaw: 0.0,
            health: 70,
            food: 64,
            water: 120,
            poison: 0,
            bleeding: 0,
            radiation: 0,
            ore_boost: 0,
            health_boost: 0,
        }
    }
}

