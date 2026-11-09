use bevy::prelude::*;

/// Audio configuration
#[derive(Resource)]
pub struct AudioConfig {
    pub master_volume: f32,
    pub grab_volume: f32,
    pub bounce_volume: f32,
    pub create_volume: f32,
    pub remove_volume: f32,
    pub idle_volume: f32,
    pub idle_frequency: f32,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            master_volume: 0.3,
            grab_volume: 1.0,
            bounce_volume: 0.3,
            create_volume: 1.0,
            remove_volume: 1.0,
            idle_volume: 0.8,
            idle_frequency: 0.5,
        }
    }
}
