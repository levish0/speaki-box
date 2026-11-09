use bevy::prelude::*;

use super::components::{Dragged, Shiny, Shockwave, Speaki, Velocity};
use super::config::ShinyConfig;

/// Shiny speakis periodically explode and push nearby speakis away
pub fn shiny_explosion_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut shiny_query: Query<(Entity, &Transform, &mut Shiny)>,
    mut speaki_query: Query<(Entity, &Transform, &mut Velocity), (With<Speaki>, Without<Dragged>)>,
    shiny_config: Res<ShinyConfig>,
    time: Res<Time>,
) {
    if !shiny_config.explosion_enabled {
        return;
    }

    let dt = time.delta_secs();
    let radius_sq = shiny_config.explosion_radius * shiny_config.explosion_radius;

    // Collect shiny positions that are exploding
    let mut explosions: Vec<(Entity, Vec2, Color)> = Vec::new();

    for (entity, transform, mut shiny) in shiny_query.iter_mut() {
        shiny.next_explosion -= dt;

        if shiny.next_explosion <= 0.0 {
            // Explosion triggered!
            explosions.push((entity, transform.translation.truncate(), shiny.base_color));

            // Reset timer with random interval
            let range = shiny_config.explosion_interval_max - shiny_config.explosion_interval_min;
            shiny.next_explosion =
                shiny_config.explosion_interval_min + rand::random::<f32>() * range;
        }
    }

    // Apply explosion force to nearby speakis and spawn shockwave
    for (shiny_entity, shiny_pos, shiny_color) in explosions {
        // Spawn shockwave visual effect
        if shiny_config.shockwave_enabled {
            let base = shiny_color.to_srgba();
            commands.spawn((
                Shockwave {
                    elapsed: 0.0,
                    duration: shiny_config.shockwave_duration,
                    max_radius: shiny_config.explosion_radius,
                    color: shiny_color,
                },
                Mesh2d(meshes.add(Annulus::new(0.9, 1.0))),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgba(
                    base.red * 2.0,
                    base.green * 2.0,
                    base.blue * 2.0,
                    0.8,
                )))),
                Transform::from_translation(shiny_pos.extend(-1.0)).with_scale(Vec3::splat(10.0)),
            ));
        }

        // Apply force to nearby speakis
        for (entity, transform, mut velocity) in speaki_query.iter_mut() {
            // Don't push self
            if entity == shiny_entity {
                continue;
            }

            let pos = transform.translation.truncate();
            let diff = pos - shiny_pos;
            let dist_sq = diff.length_squared();

            if dist_sq < radius_sq && dist_sq > 0.0 {
                let dist = dist_sq.sqrt();
                let direction = diff / dist;

                // Force decreases with distance (inverse linear)
                let force_factor = 1.0 - (dist / shiny_config.explosion_radius);
                let force = shiny_config.explosion_force * force_factor;

                velocity.x += direction.x * force;
                velocity.y += direction.y * force;
            }
        }
    }
}

/// Animate shockwave - expand and fade out
pub fn shockwave_animation_system(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Shockwave,
        &mut Transform,
        &MeshMaterial2d<ColorMaterial>,
    )>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    for (entity, mut shockwave, mut transform, material_handle) in query.iter_mut() {
        shockwave.elapsed += dt;

        let progress = (shockwave.elapsed / shockwave.duration).min(1.0);

        // Expand size
        let current_radius = shockwave.max_radius * progress;
        transform.scale = Vec3::splat(current_radius);

        // Fade out
        let alpha = (1.0 - progress) * 0.8;
        if let Some(mut material) = materials.get_mut(&material_handle.0) {
            let base = shockwave.color.to_srgba();
            material.color = Color::srgba(base.red * 2.0, base.green * 2.0, base.blue * 2.0, alpha);
        }

        // Despawn when done
        if shockwave.elapsed >= shockwave.duration {
            commands.entity(entity).despawn();
        }
    }
}
