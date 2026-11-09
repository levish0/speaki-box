use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMoved};

use super::config::{WindowConfig, WindowPositionTracker};
use crate::speaki::components::{Dragged, Speaki, Velocity};

/// Toggle title bar with Alt + T
pub fn toggle_titlebar_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_config: ResMut<WindowConfig>,
) {
    if keyboard.any_pressed([KeyCode::AltLeft, KeyCode::AltRight])
        && keyboard.just_pressed(KeyCode::KeyT)
    {
        game_config.decorations = !game_config.decorations;
    }
}

/// Sync window settings from config (only decorations, transparency can't change at runtime)
pub fn sync_window_settings_system(
    game_config: Res<WindowConfig>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    use bevy::window::WindowMode;
    if !game_config.is_changed() {
        return;
    }

    for mut window in windows.iter_mut() {
        window.decorations = game_config.decorations;

        let target_mode = if game_config.fullscreen {
            WindowMode::BorderlessFullscreen(bevy::window::MonitorSelection::Current)
        } else {
            WindowMode::Windowed
        };

        if window.mode != target_mode {
            window.mode = target_mode;
        }
    }
}

/// Apply inertia when window moves - speakis react to window movement
pub fn window_inertia_system(
    mut moved_events: MessageReader<WindowMoved>,
    mut tracker: ResMut<WindowPositionTracker>,
    mut speaki_query: Query<&mut Velocity, (With<Speaki>, Without<Dragged>)>,
) {
    for event in moved_events.read() {
        let current_pos = event.position;

        if let Some(last_pos) = tracker.last_position.filter(|_| tracker.enabled) {
            let delta_x = (current_pos.x - last_pos.x) as f32;
            let delta_y = (current_pos.y - last_pos.y) as f32;

            // Only apply if there's significant movement
            if delta_x.abs() > 0.5 || delta_y.abs() > 0.5 {
                let strength = tracker.strength;
                for mut vel in speaki_query.iter_mut() {
                    // Apply opposite impulse (inertia effect)
                    vel.x -= delta_x * strength;
                    vel.y += delta_y * strength; // Y is inverted (screen vs bevy coords)
                }
            }
        }

        tracker.last_position = Some(current_pos);
    }
}

/// Allow dragging window with Alt + Left Click
#[cfg(not(target_arch = "wasm32"))]
pub fn window_drag_system(
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if keyboard.any_pressed([KeyCode::AltLeft, KeyCode::AltRight])
        && mouse.just_pressed(MouseButton::Left)
    {
        for mut window in windows.iter_mut() {
            window.start_drag_move();
        }
    }
}
