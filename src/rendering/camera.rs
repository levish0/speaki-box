use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;

use crate::speaki::config::ShinyConfig;
use crate::window::config::WindowConfig;

fn speaki_bloom() -> Bloom {
    Bloom {
        intensity: 0.5,
        low_frequency_boost: 0.7,
        low_frequency_boost_curvature: 0.5,
        high_pass_frequency: 2.0,
        ..Bloom::OLD_SCHOOL
    }
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, speaki_bloom()));
}

pub fn sync_background_color_system(
    config: Res<WindowConfig>,
    mut clear_color: ResMut<ClearColor>,
) {
    if config.is_changed() {
        clear_color.0 = config.clear_color();
    }
}

pub fn sync_bloom_system(
    mut commands: Commands,
    config: Res<ShinyConfig>,
    cameras: Query<(Entity, Has<Bloom>), With<Camera2d>>,
) {
    if !config.is_changed() {
        return;
    }
    for (entity, has_bloom) in &cameras {
        if config.bloom_enabled && !has_bloom {
            commands.entity(entity).insert(speaki_bloom());
        } else if !config.bloom_enabled && has_bloom {
            commands.entity(entity).remove::<Bloom>();
        }
    }
}
