use bevy::prelude::*;

use crate::speaki::components::{Dragged, RotationState, Speaki, SpeakiSize, Velocity};
use crate::speaki::config::{BorderConfig, PhysicsConfig};
use crate::speaki::messages::WallBounce;

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
pub fn movement_system(
    mut query: Query<(&mut Transform, &Velocity), (With<Speaki>, Without<Dragged>)>,
) {
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
        (With<Speaki>, Without<Dragged>),
    >,
    window: Single<&Window>,
    physics: Res<PhysicsConfig>,
    border: Res<BorderConfig>,
    mut bounce_events: MessageWriter<WallBounce>,
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
        let x_axis = transform.rotation * Vec3::X * transform.scale.x;
        let y_axis = transform.rotation * Vec3::Y * transform.scale.y;
        let half_extent = Vec2::new(
            x_axis.x.abs() + y_axis.x.abs(),
            x_axis.y.abs() + y_axis.y.abs(),
        ) * (size.0 * 0.5);
        // A window can shrink below a speaki's diameter. Center that axis and stop it.
        if right_bound - left_bound <= half_extent.x * 2.0 {
            transform.translation.x = (left_bound + right_bound) * 0.5;
            vel.x = 0.0;
        }
        if top_bound - bottom_bound <= half_extent.y * 2.0 {
            transform.translation.y = (bottom_bound + top_bound) * 0.5;
            vel.y = 0.0;
        }
        let vel_sq = vel.speed_squared();
        let should_update_rotation = rot.speed.abs() < 0.01 && vel_sq > 100.0;

        // Bottom collision
        if top_bound - bottom_bound > half_extent.y * 2.0
            && transform.translation.y - half_extent.y < bottom_bound
        {
            transform.translation.y = bottom_bound + half_extent.y;
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
                bounce_events.write(WallBounce { entity });
            }
        }

        // Top collision
        if top_bound - bottom_bound > half_extent.y * 2.0
            && transform.translation.y + half_extent.y > top_bound
        {
            transform.translation.y = top_bound - half_extent.y;
            vel.y *= -physics.bounce;
            vel.x *= physics.friction;
            rot.speed *= physics.friction;

            if should_update_rotation {
                rot.speed = rand::random::<f32>() - 0.5;
            }

            if vel_sq > bounce_threshold {
                bounce_events.write(WallBounce { entity });
            }
        }

        // Left collision
        if right_bound - left_bound > half_extent.x * 2.0
            && transform.translation.x - half_extent.x < left_bound
        {
            transform.translation.x = left_bound + half_extent.x;
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
                bounce_events.write(WallBounce { entity });
            }
        }

        // Right collision
        if right_bound - left_bound > half_extent.x * 2.0
            && transform.translation.x + half_extent.x > right_bound
        {
            transform.translation.x = right_bound - half_extent.x;
            vel.x *= -physics.bounce;
            vel.y *= physics.friction;
            rot.speed *= physics.friction;

            if should_update_rotation {
                rot.speed = rand::random::<f32>() - 0.5;
            }

            if vel_sq > bounce_threshold {
                bounce_events.write(WallBounce { entity });
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::MAX_SIMULATION_FRAME_DELTA;
    use bevy::time::TimeUpdateStrategy;
    use std::time::Duration;

    fn simulate(frame_millis: u64) -> Vec3 {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(Time::<Fixed>::from_hz(60.0))
            .insert_resource(Time::<Virtual>::from_max_delta(MAX_SIMULATION_FRAME_DELTA))
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(
                frame_millis,
            )))
            .init_resource::<PhysicsConfig>()
            .add_systems(FixedUpdate, (gravity_system, movement_system).chain());
        let entity = app
            .world_mut()
            .spawn((Speaki, Velocity::default(), Transform::default()))
            .id();
        app.update();
        for _ in 0..(2000 / frame_millis) {
            app.update();
        }
        app.world().get::<Transform>(entity).unwrap().translation
    }

    #[test]
    fn physics_is_independent_of_render_frame_rate() {
        let at_50_fps = simulate(20);
        let at_100_fps = simulate(10);
        assert!(at_50_fps.y < -1000.0);
        assert_eq!(at_50_fps, at_100_fps);
    }

    #[test]
    fn rotated_sprite_corners_stay_inside_the_window() {
        let mut app = App::new();
        app.init_resource::<PhysicsConfig>()
            .init_resource::<BorderConfig>()
            .add_message::<WallBounce>()
            .add_systems(Update, wall_collision_system);
        app.world_mut().spawn(Window {
            resolution: bevy::window::WindowResolution::new(300, 300),
            ..default()
        });
        let entity = app
            .world_mut()
            .spawn((
                Speaki,
                Transform::from_xyz(100.0, -100.0, 0.0)
                    .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_4)),
                Velocity::default(),
                RotationState::default(),
                SpeakiSize(100.0),
            ))
            .id();
        app.update();
        let transform = app.world().get::<Transform>(entity).unwrap();
        for x in [-50.0, 50.0] {
            for y in [-50.0, 50.0] {
                let corner = transform.transform_point(Vec3::new(x, y, 0.0));
                assert!(
                    corner.x.abs() <= 150.001 && corner.y.abs() <= 150.001,
                    "clipped corner: {corner:?}"
                );
            }
        }
    }

    #[test]
    fn long_render_stall_does_not_advance_fifteen_physics_ticks_at_once() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(Time::<Fixed>::from_hz(60.0))
            .insert_resource(Time::<Virtual>::from_max_delta(MAX_SIMULATION_FRAME_DELTA))
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs(1)))
            .init_resource::<PhysicsConfig>()
            .add_systems(FixedUpdate, (gravity_system, movement_system).chain());
        let entity = app
            .world_mut()
            .spawn((Speaki, Velocity::new(10.0, 0.0), Transform::default()))
            .id();
        app.update();
        app.update();
        let position = app.world().get::<Transform>(entity).unwrap().translation;
        assert!(position.x > 0.0);
        assert!(
            position.x <= 30.0,
            "a stalled frame advanced too far: {position:?}"
        );
    }
}
