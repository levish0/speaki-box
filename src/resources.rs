use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

/// Game configuration
#[derive(Resource)]
pub struct GameConfig {
    pub speaki_count: u32,
    pub speaki_size: f32,
    pub click_to_add: bool,
    pub eye_blink_enabled: bool,
    pub background_color: [f32; 3],
    pub background_alpha: f32,
    pub window_transparent: bool,
    pub window_decorations: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            speaki_count: 3,
            speaki_size: 200.0,
            click_to_add: true,
            eye_blink_enabled: true,
            background_color: [0.1, 0.1, 0.1],
            background_alpha: 1.0,
            window_transparent: false,
            window_decorations: true,
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

/// Drag state tracking
#[derive(Resource, Default)]
pub struct DragState {
    pub is_dragging: bool,
    pub dragged_entity: Option<Entity>,
    pub last_start_pos: Vec2,
    pub last_start_time: f32,
    pub last_click_time: f32, // For double-click detection
}

/// Window position tracking for inertia effect
#[derive(Resource)]
pub struct WindowPositionTracker {
    pub last_position: Option<IVec2>,
    pub enabled: bool,
    pub strength: f32,
}

impl Default for WindowPositionTracker {
    fn default() -> Self {
        Self {
            last_position: None,
            enabled: true,
            strength: 0.1,
        }
    }
}

/// Image state node for animation state machine
#[derive(Clone)]
pub struct ImageStateNode {
    pub handle: Handle<Image>,
    pub eye_open: Option<usize>,
    pub eye_close: Option<usize>,
    pub mouth_open: Option<usize>,
    pub mouth_close: Option<usize>,
}

impl ImageStateNode {
    pub fn new(handle: Handle<Image>) -> Self {
        Self {
            handle,
            eye_open: None,
            eye_close: None,
            mouth_open: None,
            mouth_close: None,
        }
    }
}

/// Sprite assets storage
#[derive(Resource, Default)]
pub struct SpriteAssets {
    pub states: Vec<ImageStateNode>,
    pub loaded: bool,
}

/// Image groups for different situations
#[derive(Resource)]
pub struct ImageGroups {
    pub sad: Vec<usize>,
    pub idle: Vec<usize>,
    pub idle2: Vec<usize>,
}

impl Default for ImageGroups {
    fn default() -> Self {
        Self {
            sad: vec![9, 10], // speaki10, speaki11
            idle: vec![1, 2, 3, 4, 5, 6, 7, 8],
            idle2: vec![11, 12, 13],
        }
    }
}

/// Audio assets storage
#[derive(Resource, Default)]
pub struct AudioAssets {
    pub voices: Vec<Handle<AudioSource>>,
    pub loaded: bool,
}

/// Voice groups for different situations
#[derive(Resource)]
pub struct VoiceGroups {
    pub drag: Vec<usize>,
    pub bounce: Vec<usize>,
    pub create: Vec<usize>,
    pub remove: Vec<usize>,
    pub idle: Vec<usize>,
    pub idle2: Vec<usize>,
}

impl Default for VoiceGroups {
    fn default() -> Self {
        // Based on config.js voice mappings
        Self {
            drag: vec![0, 1, 2, 3],            // dontpress, tryhard, speakifull, speakif
            bounce: vec![16],                  // sc2e
            create: vec![4],                   // speaki
            remove: vec![15, 16],              // sc2s, sc2e (random)
            idle: vec![5, 6, 7, 8, 9, 10, 11], // g1, g2, g3, gs1, gs2, gs3, gs4
            idle2: vec![12, 14],               // sc1, sc2
        }
    }
}
