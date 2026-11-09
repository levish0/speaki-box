use bevy::prelude::*;
use bevy::window::WindowResolution;

use super::config::{WindowConfig, WindowPositionTracker};
use super::systems::{sync_window_settings_system, toggle_titlebar_system, window_inertia_system};
use crate::schedule::UpdateSet;

pub fn window_plugin(config: &WindowConfig) -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Speaki Box".to_owned(),
            resolution: WindowResolution::new(1280, 720),
            transparent: config.is_transparent(),
            composite_alpha_mode: config.composite_alpha_mode(),
            decorations: config.decorations,
            fit_canvas_to_parent: true,
            ..default()
        }),
        ..default()
    }
}

pub struct DesktopWindowPlugin;

impl Plugin for DesktopWindowPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WindowPositionTracker>()
            .add_systems(
                Update,
                (toggle_titlebar_system, window_inertia_system).in_set(UpdateSet::WindowInput),
            )
            .add_systems(
                Update,
                sync_window_settings_system.in_set(UpdateSet::Presentation),
            );
        #[cfg(not(target_arch = "wasm32"))]
        app.add_systems(Startup, super::native::set_window_icon)
            .add_systems(
                Update,
                super::systems::window_drag_system.in_set(UpdateSet::WindowInput),
            );
    }
}
