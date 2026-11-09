use std::collections::VecDeque;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::RngExt;

use super::components::{Dragged, IdleVoiceTimer, Speaki, SpeakiSize, Velocity};
use super::config::{PhysicsConfig, SpeakiConfig};
use super::messages::{DespawnSpeaki, PlayVoice, SpawnSpeaki};
use crate::assets::VoiceGroups;
use crate::audio::config::AudioConfig;

#[derive(Resource, Default)]
pub struct DragState {
    pub entity: Option<Entity>,
    samples: VecDeque<(f64, Vec2)>,
}

impl DragState {
    pub fn begin(&mut self, entity: Entity, position: Vec2, time: f64) {
        self.entity = Some(entity);
        self.samples.clear();
        self.sample(position, time);
    }

    fn sample(&mut self, position: Vec2, time: f64) {
        self.samples.push_back((time, position));
        while self.samples.len() > 2 && self.samples[1].0 < time - 0.1 {
            self.samples.pop_front();
        }
    }

    fn throw_velocity(&self, now: f64) -> Vec2 {
        let (Some(&(start, from)), Some(&(end, to))) = (self.samples.front(), self.samples.back())
        else {
            return Vec2::ZERO;
        };
        if now - end > 0.1 {
            return Vec2::ZERO;
        }
        let duration = (end - start) as f32;
        if duration < 0.001 {
            return Vec2::ZERO;
        }
        // Physics uses pixels per 60 Hz tick.
        (to - from) / (duration * 60.0)
    }

    pub fn clear(&mut self) {
        self.entity = None;
        self.samples.clear();
    }
}

fn cursor_world_position(
    window: &Window,
    cameras: &Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) -> Option<Vec2> {
    let (camera, transform) = cameras.single().ok()?;
    camera
        .viewport_to_world_2d(transform, window.cursor_position()?)
        .ok()
}

fn topmost_speaki(
    position: Vec2,
    speakis: &Query<(Entity, &Transform, &SpeakiSize), With<Speaki>>,
) -> Option<Entity> {
    speakis
        .iter()
        .filter(|(_, transform, size)| {
            position.distance_squared(transform.translation.truncate()) < (size.0 * 0.5).powi(2)
        })
        .max_by(|(a, ta, _), (b, tb, _)| {
            ta.translation
                .z
                .total_cmp(&tb.translation.z)
                .then_with(|| a.to_bits().cmp(&b.to_bits()))
        })
        .map(|(entity, _, _)| entity)
}

pub fn mouse_input_system(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    window: Single<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    speakis: Query<(Entity, &Transform, &SpeakiSize), With<Speaki>>,
    mut drag: ResMut<DragState>,
    config: Res<SpeakiConfig>,
    time: Res<Time>,
    mut spawn: MessageWriter<SpawnSpeaki>,
    mut voices: MessageWriter<PlayVoice>,
    voice_groups: Res<VoiceGroups>,
    audio_config: Res<AudioConfig>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    // A release and another press can arrive in the same rendered frame.
    if let Some(previous) = drag.entity
        && let Ok(mut entity) = commands.get_entity(previous)
    {
        entity.remove::<Dragged>();
    }
    drag.clear();
    if keyboard.any_pressed([KeyCode::AltLeft, KeyCode::AltRight]) {
        return;
    }
    let Some(position) = cursor_world_position(&window, &cameras) else {
        return;
    };
    if let Some(entity) = topmost_speaki(position, &speakis) {
        drag.begin(entity, position, time.elapsed_secs_f64());
        commands.entity(entity).insert(Dragged);
        if let Some(&voice_index) = voice_groups
            .drag
            .get(rand::rng().random_range(0..voice_groups.drag.len().max(1)))
        {
            voices.write(PlayVoice {
                entity: Some(entity),
                voice_index,
                volume: audio_config.grab_volume,
            });
        }
    } else if config.click_to_add {
        spawn.write(SpawnSpeaki {
            position,
            velocity: Vec2::ZERO,
            grab: mouse.pressed(MouseButton::Left),
        });
    }
}

pub fn drag_update_system(
    window: Single<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut speakis: Query<(&mut Transform, &mut Velocity), With<Dragged>>,
    mut drag: ResMut<DragState>,
    time: Res<Time>,
) {
    let Some(entity) = drag.entity else {
        return;
    };
    let Some(position) = cursor_world_position(&window, &cameras) else {
        return;
    };
    let Ok((mut transform, mut velocity)) = speakis.get_mut(entity) else {
        drag.clear();
        return;
    };
    drag.sample(position, time.elapsed_secs_f64());
    transform.translation.x = position.x;
    transform.translation.y = position.y;
    velocity.x = 0.0;
    velocity.y = 0.0;
}

pub fn drag_release_system(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut speakis: Query<(Entity, &mut Velocity, &mut IdleVoiceTimer), With<Dragged>>,
    mut drag: ResMut<DragState>,
    physics: Res<PhysicsConfig>,
    time: Res<Time>,
) {
    let focused = windows.single().is_ok_and(|window| window.focused);
    if mouse.pressed(MouseButton::Left) && focused {
        return;
    }
    let throw = if focused {
        drag.throw_velocity(time.elapsed_secs_f64()) * physics.cursor_throwing_power
    } else {
        Vec2::ZERO
    };
    for (entity, mut velocity, mut idle) in &mut speakis {
        velocity.x = throw.x;
        velocity.y = throw.y;
        idle.last_idle_time = time.elapsed_secs();
        commands.entity(entity).remove::<Dragged>();
    }
    drag.clear();
}

pub fn right_click_delete_system(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    speakis: Query<(Entity, &Transform, &SpeakiSize), With<Speaki>>,
    mut despawn: MessageWriter<DespawnSpeaki>,
    mut voices: MessageWriter<PlayVoice>,
    voice_groups: Res<VoiceGroups>,
    audio_config: Res<AudioConfig>,
) {
    if !mouse.just_pressed(MouseButton::Right) {
        return;
    }
    let Some(position) = cursor_world_position(&window, &cameras) else {
        return;
    };
    if let Some(entity) = topmost_speaki(position, &speakis) {
        despawn.write(DespawnSpeaki { entity });
        if let Some(&voice_index) = voice_groups
            .remove
            .get(rand::rng().random_range(0..voice_groups.remove.len().max(1)))
        {
            voices.write(PlayVoice {
                entity: None,
                voice_index,
                volume: audio_config.remove_volume,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_without_cursor_or_camera_clears_drag() {
        let mut app = App::new();
        app.init_resource::<ButtonInput<MouseButton>>()
            .init_resource::<DragState>()
            .init_resource::<PhysicsConfig>()
            .init_resource::<Time>()
            .add_systems(Update, drag_release_system);
        let entity = app
            .world_mut()
            .spawn((Dragged, Velocity::default(), IdleVoiceTimer::default()))
            .id();
        app.world_mut().resource_mut::<DragState>().entity = Some(entity);
        app.update();
        assert!(!app.world().entity(entity).contains::<Dragged>());
        assert!(app.world().resource::<DragState>().entity.is_none());
    }

    #[test]
    fn lost_focus_releases_even_while_button_is_down() {
        let mut app = App::new();
        app.init_resource::<ButtonInput<MouseButton>>()
            .init_resource::<DragState>()
            .init_resource::<PhysicsConfig>()
            .init_resource::<Time>()
            .add_systems(Update, drag_release_system);
        app.world_mut()
            .resource_mut::<ButtonInput<MouseButton>>()
            .press(MouseButton::Left);
        app.world_mut().spawn((
            Window {
                focused: false,
                ..default()
            },
            PrimaryWindow,
        ));
        let entity = app
            .world_mut()
            .spawn((
                Dragged,
                Velocity::new(10.0, 20.0),
                IdleVoiceTimer::default(),
            ))
            .id();
        app.world_mut().resource_mut::<DragState>().entity = Some(entity);
        app.update();
        assert!(!app.world().entity(entity).contains::<Dragged>());
        assert_eq!(
            app.world().get::<Velocity>(entity).unwrap().speed_squared(),
            0.0
        );
    }

    #[test]
    fn throw_uses_recent_motion_and_stops_after_pause() {
        let mut drag = DragState::default();
        drag.sample(Vec2::ZERO, 0.0);
        drag.sample(Vec2::new(60.0, 0.0), 0.1);
        assert!((drag.throw_velocity(0.1).x - 10.0).abs() < 0.001);
        assert_eq!(drag.throw_velocity(1.0), Vec2::ZERO);
        drag.sample(Vec2::new(60.0, 0.0), 0.2);
        drag.sample(Vec2::new(60.0, 0.0), 0.3);
        assert_eq!(drag.throw_velocity(0.3), Vec2::ZERO);
    }
}
