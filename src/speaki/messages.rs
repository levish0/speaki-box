use bevy::prelude::*;

/// Event to spawn a new speaki
#[derive(Message)]
pub struct SpawnSpeaki {
    pub position: Vec2,
    pub velocity: Vec2,
    pub grab: bool,
}

/// Event to despawn a speaki
#[derive(Message)]
pub struct DespawnSpeaki {
    pub entity: Entity,
}

/// Event to play a voice
#[derive(Message)]
pub struct PlayVoice {
    pub entity: Option<Entity>, // The speaki entity playing this voice (for mouth animation)
    pub voice_index: usize,
    pub volume: f32,
}

/// Event for wall bounce (to trigger sound)
#[derive(Message)]
pub struct WallBounce {
    pub entity: Entity,
}

/// Event for merging two speakis (Suika game style)
#[derive(Message)]
pub struct SpeakiMerged {
    pub entity: Entity,
}
