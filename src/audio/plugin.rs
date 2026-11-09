use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;

use super::config::AudioConfig;
use super::systems::{
    bounce_voice_system, idle_voice_system, merge_voice_system, mouth_animation_system,
    play_voice_system,
};
use crate::schedule::UpdateSet;

pub struct VoicePlaybackPlugin;

impl Plugin for VoicePlaybackPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .init_resource::<AudioConfig>()
            .add_systems(
                Update,
                (bounce_voice_system, merge_voice_system, idle_voice_system)
                    .chain()
                    .in_set(UpdateSet::VoiceRequests),
            )
            .add_systems(
                Update,
                (mouth_animation_system, play_voice_system)
                    .chain()
                    .in_set(UpdateSet::Playback),
            );
    }
}
