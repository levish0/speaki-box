use bevy::prelude::*;

/// Event to spawn a new speaki
#[derive(Message)]
pub struct SpawnSpeakiEvent {
    pub position: Vec2,
    pub velocity: Vec2,
}

/// Event to despawn a speaki
#[derive(Message)]
pub struct DespawnSpeakiEvent {
    pub entity: Entity,
}

/// Event to play a voice
#[derive(Message)]
pub struct PlayVoiceEvent {
    pub entity: Option<Entity>, // The speaki entity playing this voice (for mouth animation)
    pub voice_index: usize,
    pub volume: f32,
}

/// Event for wall bounce (to trigger sound)
#[derive(Message)]
pub struct WallBounceEvent {
    pub entity: Entity,
}

/// Event for merging two speakis (Suika game style)
#[derive(Message)]
pub struct MergeSpeakiEvent {
    pub entity1: Entity,
    pub entity2: Entity,
    pub position: Vec2,
    pub combined_velocity: Vec2,
    pub new_size: f32,
}
