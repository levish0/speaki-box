use bevy::prelude::*;

use crate::speaki::components::{Dragged, RotationState, Speaki, SpeakiSize, Velocity};
use crate::speaki::config::PhysicsConfig;

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

            if dist_sq < min_dist_sq {
                let dist = dist_sq.sqrt();
                let overlap = min_dist - dist;
                let (nx, ny) = if dist > f32::EPSILON {
                    (dx / dist, dy / dist)
                } else {
                    (1.0, 0.0)
                };

                // Separate the speakis
                let first_held = Some(e1) == dragged_entity;
                let second_held = Some(e2) == dragged_entity;
                let first_share = if first_held {
                    0.0
                } else if second_held {
                    1.0
                } else {
                    0.5
                };
                let second_share = if second_held {
                    0.0
                } else if first_held {
                    1.0
                } else {
                    0.5
                };
                speakis[i].1.x -= nx * overlap * first_share;
                speakis[i].1.y -= ny * overlap * first_share;
                speakis[j].1.x += nx * overlap * second_share;
                speakis[j].1.y += ny * overlap * second_share;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn held_speaki_stays_at_pointer_during_collision() {
        let mut app = App::new();
        app.init_resource::<PhysicsConfig>()
            .add_systems(Update, speaki_collision_system);
        let held = app
            .world_mut()
            .spawn((
                Speaki,
                Dragged,
                Transform::default(),
                Velocity::default(),
                RotationState::default(),
                SpeakiSize(100.0),
            ))
            .id();
        let other = app
            .world_mut()
            .spawn((
                Speaki,
                Transform::default(),
                Velocity::default(),
                RotationState::default(),
                SpeakiSize(100.0),
            ))
            .id();
        app.update();
        assert_eq!(
            app.world().get::<Transform>(held).unwrap().translation,
            Vec3::ZERO
        );
        assert_eq!(
            app.world()
                .get::<Transform>(other)
                .unwrap()
                .translation
                .x
                .abs(),
            100.0
        );
        assert!(app.world().get::<Velocity>(other).unwrap().x.is_finite());
    }
}
