use bevy::prelude::*;

/// Game configuration
#[derive(Resource)]
pub struct SpeakiConfig {
    pub speaki_count: u32,
    pub speaki_size: f32,
    pub click_to_add: bool,
    pub eye_blink_enabled: bool,
}

impl Default for SpeakiConfig {
    fn default() -> Self {
        Self {
            speaki_count: 3,
            speaki_size: 150.0,
            click_to_add: true,
            eye_blink_enabled: true,
        }
    }
}

/// Physics configuration
#[derive(Resource)]
pub struct PhysicsConfig {
    pub gravity: f32,
    pub bounce: f32,
    pub friction: f32,
    pub rotation_speed: f32,
    pub collision_enabled: bool,
    pub collision_damping: f32,
    pub cursor_impulse: f32,
    pub cursor_throwing_power: f32,
    pub bounce_responsiveness: f32,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            gravity: 0.5,
            bounce: 0.7,
            friction: 0.5,
            rotation_speed: 0.3,
            collision_enabled: true,
            collision_damping: 0.99,
            cursor_impulse: 20.0,
            cursor_throwing_power: 1.0,
            bounce_responsiveness: 1.0,
        }
    }
}

/// Border configuration
#[derive(Resource)]
pub struct BorderConfig {
    pub left: f32,
    pub right: f32,
    pub up: f32,
    pub down: f32,
}

impl Default for BorderConfig {
    fn default() -> Self {
        Self {
            left: 0.0,
            right: 0.0,
            up: 0.0,
            down: 0.0,
        }
    }
}

/// Shiny speaki configuration
#[derive(Resource)]
pub struct ShinyConfig {
    pub enabled: bool,
    pub spawn_chance: f32,    // 0.0 to 1.0
    pub glow_color: [f32; 3], // RGB color (0.0 to 1.0)
    pub glow_intensity: f32,  // HDR brightness multiplier
    pub pulse_speed: f32,     // Pulse animation speed
    pub bloom_enabled: bool,  // Toggle bloom effect
    // Explosion settings
    pub explosion_enabled: bool,
    pub explosion_interval_min: f32, // Minimum time between explosions (seconds)
    pub explosion_interval_max: f32, // Maximum time between explosions (seconds)
    pub explosion_radius: f32,       // How far the explosion reaches
    pub explosion_force: f32,        // How hard speakis get pushed
    // Shockwave visual settings
    pub shockwave_enabled: bool,
    pub shockwave_duration: f32, // How long the shockwave animation lasts
}

impl Default for ShinyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            spawn_chance: 0.05,           // 5% chance
            glow_color: [1.0, 0.85, 0.3], // Golden color
            glow_intensity: 3.0,          // HDR brightness
            pulse_speed: 2.0,             // Pulse cycles per second
            bloom_enabled: true,          // Bloom effect on/off
            // Explosion defaults
            explosion_enabled: true,
            explosion_interval_min: 3.0, // Min 3 seconds
            explosion_interval_max: 8.0, // Max 8 seconds
            explosion_radius: 400.0,     // 400 pixels
            explosion_force: 100.0,      // Push force
            // Shockwave defaults
            shockwave_enabled: true,
            shockwave_duration: 0.4, // 400ms animation
        }
    }
}

/// Merge configuration (Suika game style)
#[derive(Resource)]
pub struct MergeConfig {
    pub enabled: bool,
    pub size_tolerance: f32, // How close sizes need to be to merge (percentage)
    pub max_size: f32,       // Maximum size after merging
    pub growth_factor: f32,  // Size multiplier when merging (sqrt(2) = area doubles)
    pub merge_impulse: f32,  // Impulse applied when merging
}

impl Default for MergeConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            size_tolerance: 0.1,                     // 10% tolerance
            max_size: 600.0,                         // Maximum diameter in pixels
            growth_factor: std::f32::consts::SQRT_2, // sqrt(2) - area doubles
            merge_impulse: 5.0,                      // Small upward impulse
        }
    }
}
