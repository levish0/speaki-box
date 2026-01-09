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
