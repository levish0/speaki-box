use bevy::prelude::*;

use super::components::{
    BlinkTimer, CurrentAudio, Dragged, IdleVoiceTimer, RotationState, Shiny, Speaki, SpeakiSize,
    SpriteState, Velocity,
};
use super::config::{ShinyConfig, SpeakiConfig};
use super::input::DragState;
use super::messages::{DespawnSpeaki, PlayVoice, SpawnSpeaki};
use crate::assets::{SpriteAssets, VoiceGroups};
use crate::audio::config::AudioConfig;

#[derive(Resource, Default)]
pub struct SpawnOrder(f32);

impl SpawnOrder {
    fn next_layer(&mut self) -> f32 {
        self.0 += 0.001;
        self.0
    }
}

/// Handle speaki spawn events
pub fn spawn_speaki_system(
    mut commands: Commands,
    mut events: MessageReader<SpawnSpeaki>,
    config: Res<SpeakiConfig>,
    sprites: Res<SpriteAssets>,
    shiny_config: Res<ShinyConfig>,
    mut drag_state: ResMut<DragState>,
    time: Res<Time>,
    voice_groups: Res<VoiceGroups>,
    audio_config: Res<AudioConfig>,
    mut voices: MessageWriter<PlayVoice>,
    mut order: ResMut<SpawnOrder>,
) {
    for event in events.read() {
        let entity = spawn_speaki(
            &mut commands,
            event.position,
            event.velocity,
            config.speaki_size,
            &sprites,
            &shiny_config,
            order.next_layer(),
        );

        // If this was from a click, make it dragged
        if event.grab {
            drag_state.begin(entity, event.position, time.elapsed_secs_f64());
            commands.entity(entity).insert(Dragged);
        }
        if let Some(&voice_index) = voice_groups.create.first() {
            voices.write(PlayVoice {
                entity: Some(entity),
                voice_index,
                volume: audio_config.create_volume,
            });
        }
    }
}

/// Handle speaki despawn events
pub fn despawn_speaki_system(
    mut commands: Commands,
    mut events: MessageReader<DespawnSpeaki>,
    mut drag: ResMut<DragState>,
) {
    for event in events.read() {
        if drag.entity == Some(event.entity) {
            drag.clear();
        }
        if let Ok(mut entity) = commands.get_entity(event.entity) {
            entity.despawn();
        }
    }
}

/// Helper function to spawn a speaki
pub fn spawn_speaki(
    commands: &mut Commands,
    position: Vec2,
    velocity: Vec2,
    size: f32,
    sprites: &SpriteAssets,
    shiny_config: &ShinyConfig,
    layer: f32,
) -> Entity {
    // Check if this speaki should be shiny
    let is_shiny = shiny_config.enabled && rand::random::<f32>() < shiny_config.spawn_chance;

    let mut entity_commands = commands.spawn((
        Speaki,
        Velocity::new(velocity.x, velocity.y),
        RotationState::default(),
        SpriteState::default(),
        BlinkTimer::default(),
        IdleVoiceTimer::default(),
        CurrentAudio::default(),
        SpeakiSize(size),
        Transform::from_translation(position.extend(layer)),
        Visibility::default(),
    ));

    // Add sprite if assets are loaded
    if !sprites.states.is_empty() {
        let sprite_color = if is_shiny {
            // HDR color for bloom effect (values > 1.0 trigger bloom)
            let [r, g, b] = shiny_config.glow_color;
            let intensity = shiny_config.glow_intensity;
            Color::srgb(r * intensity, g * intensity, b * intensity)
        } else {
            Color::WHITE
        };

        entity_commands.insert(Sprite {
            image: sprites.states[0].handle.clone(),
            custom_size: Some(Vec2::splat(size)),
            color: sprite_color,
            ..default()
        });

        if is_shiny {
            let [r, g, b] = shiny_config.glow_color;
            entity_commands.insert(Shiny {
                base_color: Color::srgb(r, g, b),
                ..default()
            });
        }
    }

    entity_commands.id()
}

pub fn spawn_initial_speakis(
    mut commands: Commands,
    config: Res<SpeakiConfig>,
    sprites: Res<SpriteAssets>,
    shiny_config: Res<ShinyConfig>,
    window: Single<&Window>,
    mut order: ResMut<SpawnOrder>,
) {
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

    for _ in 0..config.speaki_count {
        // Random position in top 30% of screen
        let x = (rand::random::<f32>() - 0.5) * 2.0 * half_width;
        let y = half_height * 0.4 + rand::random::<f32>() * half_height * 0.6;

        // Initial velocity
        let vx = (rand::random::<f32>() - 0.5) * 5.0;
        let vy = rand::random::<f32>() * 2.0;

        spawn_speaki(
            &mut commands,
            Vec2::new(x, y),
            Vec2::new(vx, vy),
            config.speaki_size,
            &sprites,
            &shiny_config,
            order.next_layer(),
        );
    }
}
