use bevy::core_pipeline::{Core2d, Core2dSystems};
use bevy::prelude::*;
use bevy::render::{RenderApp, RenderPlugin, settings::WgpuSettings};

use super::camera::{setup_camera, sync_background_color_system, sync_bloom_system};
use crate::schedule::UpdateSet;
use crate::window::config::WindowConfig;

pub fn render_plugin(config: &WindowConfig) -> RenderPlugin {
    let mut settings = WgpuSettings::default();
    if cfg!(windows) && config.is_transparent() {
        settings.backends = Some(bevy::render::settings::Backends::DX12);
    }
    RenderPlugin {
        render_creation: settings.into(),
        ..default()
    }
}

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(windows)]
        super::sprite_shader::install_dx12_sprite_shader(app);
        app.add_systems(Startup, setup_camera).add_systems(
            Update,
            (sync_background_color_system, sync_bloom_system).in_set(UpdateSet::Presentation),
        );
        // Post-processing swaps the view's current texture. Egui must use the final target.
        app.sub_app_mut(RenderApp).configure_sets(
            Core2d,
            Core2dSystems::PostProcess.before(bevy_egui::render::egui_pass),
        );
    }
}
