use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::Rng;

use crate::components::*;
use crate::events::*;
use crate::resources::*;

/// Handle voice play events
pub fn play_voice_system(
    mut events: MessageReader<PlayVoiceEvent>,
    audio_assets: Res<AudioAssets>,
    audio_config: Res<AudioConfig>,
    audio: Res<Audio>,
    mut speaki_query: Query<(&mut CurrentAudio, &mut SpriteState), With<Speaki>>,
    sprites: Res<SpriteAssets>,
) {
    if !audio_assets.loaded {
        return;
    }

    for event in events.read() {
        if let Some(source_handle) = audio_assets.voices.get(event.voice_index) {
            // Convert amplitude (0.0-1.0) to decibels
            // dB = 20 * log10(amplitude), with minimum of -80dB for silence
            let amplitude = audio_config.master_volume * event.volume;
            let volume_db: f32 = if amplitude > 0.001 {
                20.0 * amplitude.log10()
            } else {
                -80.0 // Effectively silent
            };
            let instance_handle = audio
                .play(source_handle.clone())
                .with_volume(volume_db)
                .handle();

            // If this voice is for a specific speaki, track it and open mouth
            if let Some(entity) = event.entity {
                if let Ok((mut current_audio, mut sprite_state)) = speaki_query.get_mut(entity) {
                    current_audio.handle = Some(instance_handle);

                    // Open mouth if current image has mouth_open state
                    if let Some(state) = sprites.states.get(sprite_state.current_index) {
                        if let Some(mouth_open_idx) = state.mouth_open {
                            sprite_state.current_index = mouth_open_idx;
                        }
                    }
                }
            }
        }
    }
}

/// Check for finished audio and close mouth
pub fn mouth_animation_system(
    mut query: Query<(&mut CurrentAudio, &mut SpriteState), With<Speaki>>,
    audio_instances: Res<Assets<AudioInstance>>,
    sprites: Res<SpriteAssets>,
) {
    for (mut current_audio, mut sprite_state) in query.iter_mut() {
        if let Some(handle) = &current_audio.handle {
            let should_close_mouth;

            // Check if audio is still playing
            if let Some(instance) = audio_instances.get(handle) {
                let state = instance.state();
                should_close_mouth = state == PlaybackState::Stopped;
            } else {
                // Handle is invalid (audio finished and removed), close mouth
                should_close_mouth = true;
            }

            if should_close_mouth {
                // Audio finished, close mouth (40% chance like JS)
                if rand::random::<f32>() < 0.4 {
                    if let Some(state_node) = sprites.states.get(sprite_state.current_index) {
                        if let Some(mouth_close_idx) = state_node.mouth_close {
                            sprite_state.current_index = mouth_close_idx;
                        }
                    }
                }
                current_audio.handle = None;
            }
        }
    }
}

/// Handle wall bounce events and play sound
pub fn bounce_voice_system(
    mut bounce_events: MessageReader<WallBounceEvent>,
    mut voice_events: MessageWriter<PlayVoiceEvent>,
    voice_groups: Res<VoiceGroups>,
    audio_config: Res<AudioConfig>,
    mut query: Query<(&mut SpriteState, &mut IdleVoiceTimer), With<Speaki>>,
    image_groups: Res<ImageGroups>,
    time: Res<Time>,
) {
    for event in bounce_events.read() {
        // Play bounce voice
        if !voice_groups.bounce.is_empty() {
            let idx = voice_groups.bounce[rand::rng().random_range(0..voice_groups.bounce.len())];
            voice_events.write(PlayVoiceEvent {
                entity: Some(event.entity),
                voice_index: idx,
                volume: audio_config.bounce_volume,
            });
        }

        // Change to sad expression
        if let Ok((mut sprite_state, mut idle_timer)) = query.get_mut(event.entity) {
            if !image_groups.sad.is_empty() {
                sprite_state.current_index =
                    image_groups.sad[rand::rng().random_range(0..image_groups.sad.len())];
            }
            idle_timer.last_idle_time = time.elapsed_secs();
        }
    }
}

/// Handle idle voice (random sounds when not interacting)
pub fn idle_voice_system(
    mut query: Query<
        (&mut IdleVoiceTimer, &mut SpriteState, Entity),
        (With<Speaki>, Without<Dragged>),
    >,
    mut voice_events: MessageWriter<PlayVoiceEvent>,
    voice_groups: Res<VoiceGroups>,
    image_groups: Res<ImageGroups>,
    audio_config: Res<AudioConfig>,
    time: Res<Time>,
) {
    if audio_config.idle_frequency <= 0.0 {
        return;
    }

    let current_time = time.elapsed_secs();

    for (mut timer, mut sprite_state, entity) in query.iter_mut() {
        // Calculate interval based on frequency
        // Original: (30000 / frequency - 29000) * cooldown + 3000 ms
        let interval = (30.0 / audio_config.idle_frequency - 29.0) * timer.idle_cooldown + 3.0;

        if current_time - timer.last_idle_time > interval {
            // Play idle voice
            if rand::random::<f32>() > 0.8 {
                // 20% chance for idle2
                if !voice_groups.idle2.is_empty() {
                    let idx =
                        voice_groups.idle2[rand::rng().random_range(0..voice_groups.idle2.len())];
                    voice_events.write(PlayVoiceEvent {
                        entity: Some(entity),
                        voice_index: idx,
                        volume: audio_config.idle_volume,
                    });
                }
                if !image_groups.idle2.is_empty() {
                    sprite_state.current_index =
                        image_groups.idle2[rand::rng().random_range(0..image_groups.idle2.len())];
                }
            } else {
                // 80% chance for idle
                if !voice_groups.idle.is_empty() {
                    let idx =
                        voice_groups.idle[rand::rng().random_range(0..voice_groups.idle.len())];
                    voice_events.write(PlayVoiceEvent {
                        entity: Some(entity),
                        voice_index: idx,
                        volume: audio_config.idle_volume,
                    });
                }
                if !image_groups.idle.is_empty() {
                    sprite_state.current_index =
                        image_groups.idle[rand::rng().random_range(0..image_groups.idle.len())];
                }
            }

            timer.last_idle_time = current_time;
            timer.idle_cooldown = rand::random::<f32>();
        }
    }
}
