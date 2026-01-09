use bevy::prelude::*;
use bevy_kira_audio::AudioInstance;

/// Speaki entity marker
#[derive(Component)]
pub struct Speaki;

/// Velocity component for physics
#[derive(Component, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn speed_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
}

/// Rotation state for spinning
#[derive(Component)]
pub struct RotationState {
    pub speed: f32,
}

impl Default for RotationState {
    fn default() -> Self {
        Self {
            speed: (rand::random::<f32>() - 0.5),
        }
    }
}

/// Current sprite state index (for animation state machine)
#[derive(Component, Default)]
pub struct SpriteState {
    pub current_index: usize,
}

/// Blink timer for eye animation
#[derive(Component)]
pub struct BlinkTimer {
    pub last_blink_time: f32,
    pub blink_cooldown: f32,
    pub blink_open_time: f32,
    pub is_blinking: bool,
    pub double_blink: bool,
}

impl Default for BlinkTimer {
    fn default() -> Self {
        Self {
            last_blink_time: 0.0,
            blink_cooldown: 5.0 + 5.0 * rand::random::<f32>(),
            blink_open_time: 0.0,
            is_blinking: false,
            double_blink: false,
        }
    }
}

/// Idle voice timer
#[derive(Component)]
pub struct IdleVoiceTimer {
    pub last_idle_time: f32,
    pub idle_cooldown: f32,
}

impl Default for IdleVoiceTimer {
    fn default() -> Self {
        Self {
            last_idle_time: 0.0,
            idle_cooldown: rand::random::<f32>(),
        }
    }
}

/// Marker for dragged speaki
#[derive(Component)]
pub struct Dragged;

/// Speaki size
#[derive(Component)]
pub struct SpeakiSize(pub f32);

impl Default for SpeakiSize {
    fn default() -> Self {
        Self(150.0)
    }
}

/// Current audio being played by this speaki
#[derive(Component, Default)]
pub struct CurrentAudio {
    pub handle: Option<Handle<AudioInstance>>,
}
