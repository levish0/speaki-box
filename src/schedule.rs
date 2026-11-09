use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum StartupSet {
    LoadAssets,
    Spawn,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateSet {
    WindowInput,
    PointerInput,
    Spawn,
    Effects,
    VoiceRequests,
    Playback,
    Animation,
    Presentation,
}
