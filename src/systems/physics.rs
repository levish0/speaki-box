use bevy::prelude::*;
use bevy::window::WindowMoved;

use crate::components::*;
use crate::events::*;
use crate::resources::*;

/// Apply gravity to all non-dragged speakis
pub fn gravity_system(
    mut query: Query<&mut Velocity, (With<Speaki>, Without<Dragged>)>,
    physics: Res<PhysicsConfig>,
) {
    for mut vel in query.iter_mut() {
        // Bevy Y is up, so gravity decreases Y
        vel.y -= physics.gravity;
    }
}

/// Update positions based on velocity
pub fn movement_system(mut query: Query<(&mut Transform, &Velocity), With<Speaki>>) {
    for (mut transform, vel) in query.iter_mut() {
        transform.translation.x += vel.x;
        transform.translation.y += vel.y;
    }
}

/// Handle wall collisions
pub fn wall_collision_system(
    mut query: Query<
        (
            &mut Transform,
            &mut Velocity,
            &mut RotationState,
            &SpeakiSize,
            Entity,
        ),
        With<Speaki>,
    >,
    window: Single<&Window>,
    physics: Res<PhysicsConfig>,
    border: Res<BorderConfig>,
    mut bounce_events: MessageWriter<WallBounceEvent>,
) {
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

    // Calculate bounds
    let left_bound = -half_width + half_width * border.left;
    let right_bound = half_width - half_width * border.right;
    let top_bound = half_height - half_height * border.up;
    let bottom_bound = -half_height + half_height * border.down;

    let bounce_threshold = 1000.0 * physics.bounce_responsiveness;

    for (mut transform, mut vel, mut rot, size, entity) in query.iter_mut() {
        let half_size = size.0 / 2.0;
        let vel_sq = vel.speed_squared();
        let should_update_rotation = rot.speed.abs() < 0.01 && vel_sq > 100.0;

        // Bottom collision
        if transform.translation.y - half_size < bottom_bound {
            transform.translation.y = bottom_bound + half_size;
            vel.y *= -physics.bounce;
            vel.x *= physics.friction;
            rot.speed *= physics.friction;

            if vel.y.abs() < 0.5 {
                vel.y = 0.0;
            }

            if should_update_rotation {
                rot.speed = rand::random::<f32>() - 0.5;
            }

            if vel_sq > bounce_threshold {
                bounce_events.write(WallBounceEvent { entity });
            }
        }

        // Top collision
        if transform.translation.y + half_size > top_bound {
            transform.translation.y = top_bound - half_size;
            vel.y *= -physics.bounce;
            vel.x *= physics.friction;
            rot.speed *= physics.friction;

            if should_update_rotation {
                rot.speed = rand::random::<f32>() - 0.5;
            }

            if vel_sq > bounce_threshold {
                bounce_events.write(WallBounceEvent { entity });
            }
        }

        // Left collision
        if transform.translation.x - half_size < left_bound {
            transform.translation.x = left_bound + half_size;
            vel.x *= -physics.bounce;
            vel.y *= physics.friction;
            rot.speed *= physics.friction;

            if vel.x.abs() < 0.5 {
                vel.x = 0.0;
            }

            if should_update_rotation {
                rot.speed = rand::random::<f32>() - 0.5;
            }

            if vel_sq > bounce_threshold {
                bounce_events.write(WallBounceEvent { entity });
            }
        }

        // Right collision
        if transform.translation.x + half_size > right_bound {
            transform.translation.x = right_bound - half_size;
            vel.x *= -physics.bounce;
            vel.y *= physics.friction;
            rot.speed *= physics.friction;

            if should_update_rotation {
                rot.speed = rand::random::<f32>() - 0.5;
            }

            if vel_sq > bounce_threshold {
                bounce_events.write(WallBounceEvent { entity });
            }
        }
    }
}

/// Handle speaki-to-speaki collisions
pub fn speaki_collision_system(
    mut query: Query<
        (
            Entity,
            &mut Transform,
            &mut Velocity,
            &mut RotationState,
            &SpeakiSize,
        ),
        With<Speaki>,
    >,
    dragged_query: Query<Entity, With<Dragged>>,
    physics: Res<PhysicsConfig>,
) {
    if !physics.collision_enabled {
        return;
    }

    let dragged_entity = dragged_query.iter().next();

    // Collect all speaki data
    let mut speakis: Vec<(Entity, Vec3, Vec2, f32)> = query
        .iter()
        .map(|(e, t, v, _, s)| (e, t.translation, Vec2::new(v.x, v.y), s.0))
        .collect();

    let len = speakis.len();

    // Check all pairs
    for i in 0..len {
        for j in (i + 1)..len {
            let (e1, pos1, vel1, size1) = speakis[i];
            let (e2, pos2, vel2, size2) = speakis[j];

            let dx = pos2.x - pos1.x;
            let dy = pos2.y - pos1.y;
            let dist_sq = dx * dx + dy * dy;
            let min_dist = (size1 + size2) / 2.0;
            let min_dist_sq = min_dist * min_dist;

            if dist_sq < min_dist_sq && dist_sq > 0.0 {
                let dist = dist_sq.sqrt();
                let overlap = min_dist - dist;
                let nx = dx / dist;
                let ny = dy / dist;

                // Separate the speakis
                let sep = overlap * 0.5;
                speakis[i].1.x -= nx * sep;
                speakis[i].1.y -= ny * sep;
                speakis[j].1.x += nx * sep;
                speakis[j].1.y += ny * sep;

                // Calculate relative velocity
                let dvx = vel2.x - vel1.x;
                let dvy = vel2.y - vel1.y;

                // Apply cursor impulse if one is being dragged
                let dvn = if Some(e1) == dragged_entity || Some(e2) == dragged_entity {
                    -physics.cursor_impulse
                } else {
                    dvx * nx + dvy * ny
                };

                // Only process if approaching
                if dvn > 0.0 {
                    continue;
                }

                // Calculate impulse (assuming equal mass)
                let impulse = dvn;

                // Update velocities
                let is_e1_dragged = Some(e1) == dragged_entity;
                let is_e2_dragged = Some(e2) == dragged_entity;

                if !is_e1_dragged {
                    speakis[i].2.x += impulse * nx * physics.collision_damping;
                    speakis[i].2.y += impulse * ny * physics.collision_damping;
                }

                if !is_e2_dragged {
                    speakis[j].2.x -= impulse * nx * physics.collision_damping;
                    speakis[j].2.y -= impulse * ny * physics.collision_damping;
                }
            }
        }
    }

    // Apply changes back to entities
    for (entity, new_pos, new_vel, _) in speakis {
        if let Ok((_, mut transform, mut vel, mut rot, _)) = query.get_mut(entity) {
            transform.translation.x = new_pos.x;
            transform.translation.y = new_pos.y;
            vel.x = new_vel.x;
            vel.y = new_vel.y;
            rot.speed *= physics.collision_damping;
        }
    }
}

/// Update rotation
pub fn rotation_system(
    mut query: Query<(&mut Transform, &RotationState), With<Speaki>>,
    physics: Res<PhysicsConfig>,
) {
    for (mut transform, rot) in query.iter_mut() {
        let angle = rot.speed * physics.rotation_speed;
        transform.rotate_z(angle);
    }
}

/// Apply inertia when window moves - speakis react to window movement
pub fn window_inertia_system(
    mut moved_events: MessageReader<WindowMoved>,
    mut tracker: ResMut<WindowPositionTracker>,
    mut speaki_query: Query<&mut Velocity, (With<Speaki>, Without<Dragged>)>,
) {
    if !tracker.enabled {
        return;
    }

    for event in moved_events.read() {
        let current_pos = event.position;

        if let Some(last_pos) = tracker.last_position {
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

/// Shiny speakis periodically explode and push nearby speakis away
pub fn shiny_explosion_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut shiny_query: Query<(Entity, &Transform, &mut Shiny)>,
    mut speaki_query: Query<(Entity, &Transform, &mut Velocity), With<Speaki>>,
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
            shiny.next_explosion = shiny_config.explosion_interval_min + rand::random::<f32>() * range;
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
                Mesh2d(meshes.add(bevy::math::primitives::Annulus::new(0.9, 1.0))),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(
                    Color::srgba(base.red * 2.0, base.green * 2.0, base.blue * 2.0, 0.8)
                ))),
                Transform::from_translation(shiny_pos.extend(-1.0))
                    .with_scale(Vec3::splat(10.0)),
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
    mut query: Query<(Entity, &mut Shockwave, &mut Transform, &MeshMaterial2d<ColorMaterial>)>,
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
        if let Some(material) = materials.get_mut(&material_handle.0) {
            let base = shockwave.color.to_srgba();
            material.color = Color::srgba(base.red * 2.0, base.green * 2.0, base.blue * 2.0, alpha);
        }

        // Despawn when done
        if shockwave.elapsed >= shockwave.duration {
            commands.entity(entity).despawn();
        }
    }
}

/// Detect and handle speaki merging (Suika game style)
/// When two speakis of the same size collide, they merge into one larger speaki
pub fn speaki_merge_system(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &Transform,
            &mut Velocity,
            &mut SpeakiSize,
            &mut Sprite,
            Option<&Shiny>,
        ),
        With<Speaki>,
    >,
    dragged_query: Query<Entity, With<Dragged>>,
    merge_config: Res<MergeConfig>,
    mut merge_events: MessageWriter<MergeSpeakiEvent>,
) {
    if !merge_config.enabled {
        return;
    }

    let dragged_entity = dragged_query.iter().next();

    // Collect all speaki data
    let speakis: Vec<(Entity, Vec3, Vec2, f32, bool)> = query
        .iter()
        .map(|(e, t, v, s, _, shiny)| (e, t.translation, Vec2::new(v.x, v.y), s.0, shiny.is_some()))
        .collect();

    let len = speakis.len();
    let mut to_merge: Vec<(Entity, Entity, Vec2, Vec2, f32, bool)> = Vec::new();
    let mut already_merged: std::collections::HashSet<Entity> = std::collections::HashSet::new();

    // Check all pairs for merge candidates
    for i in 0..len {
        if already_merged.contains(&speakis[i].0) {
            continue;
        }

        for j in (i + 1)..len {
            if already_merged.contains(&speakis[j].0) {
                continue;
            }

            let (e1, pos1, vel1, size1, shiny1) = speakis[i];
            let (e2, pos2, vel2, size2, shiny2) = speakis[j];

            // Skip if either is being dragged
            if Some(e1) == dragged_entity || Some(e2) == dragged_entity {
                continue;
            }

            // Check if sizes are similar (within tolerance)
            let size_diff = (size1 - size2).abs();
            let avg_size = (size1 + size2) / 2.0;
            if size_diff / avg_size > merge_config.size_tolerance {
                continue;
            }

            // Skip if either is already at max size
            if size1 >= merge_config.max_size || size2 >= merge_config.max_size {
                continue;
            }

            // Check if colliding
            let dx = pos2.x - pos1.x;
            let dy = pos2.y - pos1.y;
            let dist_sq = dx * dx + dy * dy;
            let min_dist = (size1 + size2) / 2.0;
            let min_dist_sq = min_dist * min_dist;

            if dist_sq < min_dist_sq && dist_sq > 0.0 {
                // Merge! Use midpoint position and combined velocity
                let mid_pos = Vec2::new((pos1.x + pos2.x) / 2.0, (pos1.y + pos2.y) / 2.0);
                let combined_vel = Vec2::new((vel1.x + vel2.x) / 2.0, (vel1.y + vel2.y) / 2.0);
                let new_size = (avg_size * merge_config.growth_factor).min(merge_config.max_size);
                let keep_shiny = shiny1 || shiny2;

                to_merge.push((e1, e2, mid_pos, combined_vel, new_size, keep_shiny));
                already_merged.insert(e1);
                already_merged.insert(e2);
                break; // Only one merge per entity per frame
            }
        }
    }

    // Execute merges
    for (e1, e2, mid_pos, combined_vel, new_size, _keep_shiny) in to_merge {
        // Send merge event (for sound effects, etc.)
        merge_events.write(MergeSpeakiEvent {
            entity1: e1,
            entity2: e2,
            position: mid_pos,
            combined_velocity: combined_vel,
            new_size,
        });

        // Despawn one entity
        commands.entity(e2).despawn();

        // Update the remaining entity
        if let Ok((_, _, mut vel, mut size, mut sprite, _)) = query.get_mut(e1) {
            // Move to midpoint - we can't mutate Transform here so we use velocity
            vel.x = combined_vel.x;
            vel.y = combined_vel.y + merge_config.merge_impulse; // Small upward pop

            // Update size
            size.0 = new_size;
            sprite.custom_size = Some(Vec2::splat(new_size));
        }
    }
}
