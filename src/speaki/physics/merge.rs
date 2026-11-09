use bevy::prelude::*;

use crate::speaki::components::{Dragged, Shiny, Speaki, SpeakiSize, Velocity};
use crate::speaki::config::MergeConfig;
use crate::speaki::messages::SpeakiMerged;

/// Detect and handle speaki merging (Suika game style)
/// When two speakis of the same size collide, they merge into one larger speaki
pub fn speaki_merge_system(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut Transform,
            &mut Velocity,
            &mut SpeakiSize,
            &mut Sprite,
            Option<&Shiny>,
        ),
        With<Speaki>,
    >,
    dragged_query: Query<Entity, With<Dragged>>,
    merge_config: Res<MergeConfig>,
    mut merge_events: MessageWriter<SpeakiMerged>,
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
    let mut to_merge: Vec<(Entity, Entity, Vec2, Vec2, f32)> = Vec::new();
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

            if dist_sq < min_dist_sq {
                // Merge! Use midpoint position and combined velocity
                let mid_pos = Vec2::new((pos1.x + pos2.x) / 2.0, (pos1.y + pos2.y) / 2.0);
                let combined_vel = Vec2::new((vel1.x + vel2.x) / 2.0, (vel1.y + vel2.y) / 2.0);
                let new_size = (avg_size * merge_config.growth_factor).min(merge_config.max_size);
                let (survivor, removed) = if shiny2 && !shiny1 {
                    (e2, e1)
                } else {
                    (e1, e2)
                };
                to_merge.push((survivor, removed, mid_pos, combined_vel, new_size));
                already_merged.insert(e1);
                already_merged.insert(e2);
                break; // Only one merge per entity per frame
            }
        }
    }

    // Execute merges
    for (e1, e2, mid_pos, combined_vel, new_size) in to_merge {
        // Send merge event (for sound effects, etc.)
        merge_events.write(SpeakiMerged { entity: e1 });

        // Despawn one entity
        commands.entity(e2).despawn();

        // Update the remaining entity
        if let Ok((_, mut transform, mut vel, mut size, mut sprite, _)) = query.get_mut(e1) {
            transform.translation.x = mid_pos.x;
            transform.translation.y = mid_pos.y;
            vel.x = combined_vel.x;
            vel.y = combined_vel.y + merge_config.merge_impulse; // Small upward pop

            // Update size
            size.0 = new_size;
            sprite.custom_size = Some(Vec2::splat(new_size));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_preserves_shiny_and_uses_midpoint() {
        let mut app = App::new();
        app.insert_resource(MergeConfig {
            enabled: true,
            ..default()
        })
        .add_message::<SpeakiMerged>()
        .add_systems(Update, speaki_merge_system);
        let first = app
            .world_mut()
            .spawn((
                Speaki,
                Transform::from_xyz(0.0, 0.0, 1.0),
                Velocity::new(2.0, 0.0),
                SpeakiSize(100.0),
                Sprite::default(),
            ))
            .id();
        let second = app
            .world_mut()
            .spawn((
                Speaki,
                Shiny::default(),
                Transform::from_xyz(50.0, 0.0, 2.0),
                Velocity::new(4.0, 0.0),
                SpeakiSize(100.0),
                Sprite::default(),
            ))
            .id();
        app.update();
        assert!(app.world().get_entity(first).is_err());
        assert!(app.world().entity(second).contains::<Shiny>());
        assert_eq!(
            app.world().get::<Transform>(second).unwrap().translation,
            Vec3::new(25.0, 0.0, 2.0)
        );
        assert_eq!(app.world().get::<Velocity>(second).unwrap().x, 3.0);
        assert!(
            (app.world().get::<SpeakiSize>(second).unwrap().0 - 100.0 * std::f32::consts::SQRT_2)
                .abs()
                < 0.001
        );
    }

    #[test]
    fn coincident_speakis_can_merge() {
        let mut app = App::new();
        app.insert_resource(MergeConfig {
            enabled: true,
            ..default()
        })
        .add_message::<SpeakiMerged>()
        .add_systems(Update, speaki_merge_system);
        for _ in 0..2 {
            app.world_mut().spawn((
                Speaki,
                Transform::default(),
                Velocity::default(),
                SpeakiSize(100.0),
                Sprite::default(),
            ));
        }
        app.update();
        assert_eq!(
            app.world_mut()
                .query_filtered::<Entity, With<Speaki>>()
                .iter(app.world())
                .count(),
            1
        );
    }
}
