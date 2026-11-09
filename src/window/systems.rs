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
    window: Single<(Entity, &Window), With<PrimaryWindow>>,
) {
    let Some(current_pos) = moved_events
        .read()
        .filter(|event| event.window == window.0)
        .map(|event| event.position)
        .last()
    else {
        return;
    };
    // Native creation can report several positions before the first frame. Only establish an anchor.
    let Some(last_pos) = tracker.last_position.replace(current_pos) else {
        return;
    };
    if !tracker.enabled {
        return;
    }
    let delta = (current_pos - last_pos).as_vec2() / window.1.scale_factor();
    for mut velocity in &mut speaki_query {
        velocity.x -= delta.x * tracker.strength;
        velocity.y += delta.y * tracker.strength;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn app_with_window() -> (App, Entity, Entity) {
        let mut app = App::new();
        app.add_message::<WindowMoved>()
            .init_resource::<WindowPositionTracker>()
            .add_systems(Update, window_inertia_system);
        let window = app
            .world_mut()
            .spawn((Window::default(), PrimaryWindow))
            .id();
        let speaki = app.world_mut().spawn((Speaki, Velocity::default())).id();
        (app, window, speaki)
    }

    fn move_window(app: &mut App, window: Entity, position: IVec2) {
        app.world_mut()
            .write_message(WindowMoved { window, position });
    }

    #[test]
    fn startup_position_burst_does_not_throw_speakis() {
        let (mut app, window, speaki) = app_with_window();
        move_window(&mut app, window, IVec2::ZERO);
        move_window(&mut app, window, IVec2::new(2000, 1000));
        app.update();
        assert_eq!(
            app.world().get::<Velocity>(speaki).unwrap().speed_squared(),
            0.0
        );
        assert_eq!(
            app.world()
                .resource::<WindowPositionTracker>()
                .last_position,
            Some(IVec2::new(2000, 1000))
        );
        move_window(&mut app, window, IVec2::new(2100, 1000));
        app.update();
        assert_eq!(app.world().get::<Velocity>(speaki).unwrap().x, -10.0);
    }

    #[test]
    fn disabled_inertia_keeps_its_anchor_current() {
        let (mut app, window, speaki) = app_with_window();
        app.world_mut()
            .resource_mut::<WindowPositionTracker>()
            .enabled = false;
        move_window(&mut app, window, IVec2::ZERO);
        app.update();
        move_window(&mut app, window, IVec2::new(2000, 1000));
        app.update();
        app.world_mut()
            .resource_mut::<WindowPositionTracker>()
            .enabled = true;
        move_window(&mut app, window, IVec2::new(2010, 1000));
        app.update();
        assert_eq!(app.world().get::<Velocity>(speaki).unwrap().x, -1.0);
    }
}
