use bevy::prelude::*;
use rand::Rng;

use crate::components::*;
use crate::events::*;
use crate::resources::*;

/// Handle mouse input for clicking/grabbing speakis
pub fn mouse_input_system(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    window: Single<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    speaki_query: Query<(Entity, &Transform, &SpeakiSize), With<Speaki>>,
    mut drag_state: ResMut<DragState>,
    config: Res<GameConfig>,
    time: Res<Time>,
    mut spawn_events: MessageWriter<SpawnSpeakiEvent>,
    mut voice_events: MessageWriter<PlayVoiceEvent>,
    voice_groups: Res<VoiceGroups>,
    audio_config: Res<AudioConfig>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    // Skip if Alt is pressed (window drag mode)
    if keyboard.pressed(KeyCode::AltLeft) {
        return;
    }

    let Ok((camera, camera_transform)) = camera_q.single() else {
        return;
    };

    // Get cursor position in world coordinates
    let Some(cursor_pos) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(camera_transform, p).ok())
    else {
        return;
    };

    let current_time = time.elapsed_secs();

    // Check if clicking on existing speaki (reverse order for topmost first)
    let mut speakis: Vec<_> = speaki_query.iter().collect();
    speakis.reverse();

    for (entity, transform, size) in speakis {
        let dx = cursor_pos.x - transform.translation.x;
        let dy = cursor_pos.y - transform.translation.y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist < size.0 / 2.0 {
            // Start dragging
            drag_state.is_dragging = true;
            drag_state.dragged_entity = Some(entity);
            drag_state.last_start_pos = cursor_pos;
            drag_state.last_start_time = current_time;

            // Add Dragged marker
            commands.entity(entity).insert(Dragged);

            // Play drag voice
            if !voice_groups.drag.is_empty() {
                let idx = voice_groups.drag[rand::rng().random_range(0..voice_groups.drag.len())];
                voice_events.write(PlayVoiceEvent {
                    entity: Some(entity),
                    voice_index: idx,
                    volume: audio_config.grab_volume,
                });
            }

            return;
        }
    }

    // Clicked on empty space: create new speaki
    if config.click_to_add {
        drag_state.is_dragging = true;
        drag_state.last_start_pos = cursor_pos;
        drag_state.last_start_time = current_time;
        drag_state.last_click_time = current_time;

        spawn_events.write(SpawnSpeakiEvent {
            position: cursor_pos,
            velocity: Vec2::ZERO,
        });

        // Play create voice (entity will be set after spawn)
        if let Some(&idx) = voice_groups.create.first() {
            voice_events.write(PlayVoiceEvent {
                entity: None, // New speaki, will be spawned separately
                voice_index: idx,
                volume: audio_config.create_volume,
            });
        }
    }
}

/// Update dragged speaki position
pub fn drag_update_system(
    window: Single<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Dragged>>,
    mut drag_state: ResMut<DragState>,
    time: Res<Time>,
) {
    if !drag_state.is_dragging {
        return;
    }

    let Ok((camera, camera_transform)) = camera_q.single() else {
        return;
    };

    let Some(cursor_pos) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(camera_transform, p).ok())
    else {
        return;
    };

    let current_time = time.elapsed_secs();

    // Update reference point every 100ms
    if current_time - drag_state.last_start_time > 0.1 {
        drag_state.last_start_time = current_time;
        drag_state.last_start_pos = cursor_pos;
    }

    // Move dragged speaki to cursor
    for (mut transform, mut vel) in query.iter_mut() {
        transform.translation.x = cursor_pos.x;
        transform.translation.y = cursor_pos.y;
        vel.x = 0.0;
        vel.y = 0.0;
    }
}

/// Handle drag release (throwing)
pub fn drag_release_system(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut query: Query<(Entity, &mut Velocity, &mut IdleVoiceTimer), With<Dragged>>,
    mut drag_state: ResMut<DragState>,
    physics: Res<PhysicsConfig>,
    time: Res<Time>,
) {
    if !mouse_button.just_released(MouseButton::Left) {
        return;
    }

    if !drag_state.is_dragging {
        return;
    }

    let Ok((camera, camera_transform)) = camera_q.single() else {
        return;
    };

    let Some(cursor_pos) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(camera_transform, p).ok())
    else {
        return;
    };

    let current_time = time.elapsed_secs();
    let drag_time = current_time - drag_state.last_start_time;

    // Calculate throw velocity
    let delta = cursor_pos - drag_state.last_start_pos;
    let time_factor = (drag_time / 0.05).max(0.001);
    let throw_vel = delta / time_factor * physics.cursor_throwing_power;

    for (entity, mut vel, mut idle_timer) in query.iter_mut() {
        vel.x = throw_vel.x;
        vel.y = throw_vel.y;

        // Reset idle timer
        idle_timer.last_idle_time = current_time;

        // Remove Dragged marker
        commands.entity(entity).remove::<Dragged>();
    }

    drag_state.is_dragging = false;
    drag_state.dragged_entity = None;
}

/// Handle speaki spawn events
pub fn spawn_speaki_system(
    mut commands: Commands,
    mut events: MessageReader<SpawnSpeakiEvent>,
    config: Res<GameConfig>,
    sprites: Res<SpriteAssets>,
    mut drag_state: ResMut<DragState>,
) {
    for event in events.read() {
        let entity = spawn_speaki(
            &mut commands,
            event.position,
            event.velocity,
            config.speaki_size,
            &sprites,
        );

        // If this was from a click, make it dragged
        if drag_state.is_dragging && drag_state.dragged_entity.is_none() {
            drag_state.dragged_entity = Some(entity);
            commands.entity(entity).insert(Dragged);
        }
    }
}

/// Handle right-click to delete speaki
pub fn right_click_delete_system(
    mouse_button: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    speaki_query: Query<(Entity, &Transform, &SpeakiSize), With<Speaki>>,
    mut despawn_events: MessageWriter<DespawnSpeakiEvent>,
    mut voice_events: MessageWriter<PlayVoiceEvent>,
    voice_groups: Res<VoiceGroups>,
    audio_config: Res<AudioConfig>,
) {
    if !mouse_button.just_pressed(MouseButton::Right) {
        return;
    }

    let Ok((camera, camera_transform)) = camera_q.single() else {
        return;
    };

    let Some(cursor_pos) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(camera_transform, p).ok())
    else {
        return;
    };

    // Check if clicking on existing speaki (reverse order for topmost first)
    let mut speakis: Vec<_> = speaki_query.iter().collect();
    speakis.reverse();

    for (entity, transform, size) in speakis {
        let dx = cursor_pos.x - transform.translation.x;
        let dy = cursor_pos.y - transform.translation.y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist < size.0 / 2.0 {
            // Delete speaki
            despawn_events.write(DespawnSpeakiEvent { entity });

            // Play remove voice (random)
            if !voice_groups.remove.is_empty() {
                let idx =
                    voice_groups.remove[rand::rng().random_range(0..voice_groups.remove.len())];
                voice_events.write(PlayVoiceEvent {
                    entity: Some(entity),
                    voice_index: idx,
                    volume: audio_config.remove_volume,
                });
            }
            return;
        }
    }
}

/// Handle speaki despawn events
pub fn despawn_speaki_system(
    mut commands: Commands,
    mut events: MessageReader<DespawnSpeakiEvent>,
) {
    for event in events.read() {
        commands.entity(event.entity).despawn();
    }
}

/// Helper function to spawn a speaki
pub fn spawn_speaki(
    commands: &mut Commands,
    position: Vec2,
    velocity: Vec2,
    size: f32,
    sprites: &SpriteAssets,
) -> Entity {
    let mut entity_commands = commands.spawn((
        Speaki,
        Velocity::new(velocity.x, velocity.y),
        RotationState::default(),
        SpriteState::default(),
        BlinkTimer::default(),
        IdleVoiceTimer::default(),
        CurrentAudio::default(),
        SpeakiSize(size),
        Transform::from_translation(position.extend(0.0)),
        Visibility::default(),
    ));

    // Add sprite if assets are loaded
    if sprites.loaded && !sprites.states.is_empty() {
        entity_commands.insert(Sprite {
            image: sprites.states[0].handle.clone(),
            custom_size: Some(Vec2::splat(size)),
            ..default()
        });
    }

    entity_commands.id()
}

/// Allow dragging window with Alt + Left Click
pub fn window_drag_system(
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window>,
) {
    if keyboard.pressed(KeyCode::AltLeft) && mouse.just_pressed(MouseButton::Left) {
        for mut window in windows.iter_mut() {
            window.start_drag_move();
        }
    }
}
