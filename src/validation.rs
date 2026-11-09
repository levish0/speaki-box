use std::collections::HashMap;
use std::path::PathBuf;

use bevy::app::AppExit;
use bevy::input::{
    ButtonState,
    keyboard::{Key, KeyboardInput},
};
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, ScreenshotCaptured};
use bevy::window::PrimaryWindow;
use bevy_egui::{EguiPostUpdateSet, EguiRenderOutput, PrimaryEguiContext};

use crate::app::build_app;
use crate::audio::config::AudioConfig;
use crate::settings::ui::SettingsOpen;
use crate::speaki::components::{Speaki, Velocity};
use crate::speaki::config::ShinyConfig;
use crate::window::config::WindowConfig;

#[derive(Resource)]
struct WindowSmokeTest {
    screenshot_path: PathBuf,
    input_sent: bool,
    released: bool,
    capture_started: bool,
    reported: bool,
    max_step: f32,
    previous_positions: HashMap<Entity, Vec2>,
}

/// Exercise the native renderer and capture the final GPU output after a Q key message.
pub fn run_window_smoke_test(transparent: bool) -> AppExit {
    #[cfg(windows)]
    crate::window::native::prepare_transparent_presentation(transparent);

    let mut app = build_app(WindowConfig::new(transparent));
    app.world_mut().resource_mut::<ShinyConfig>().spawn_chance = 0.0;
    app.world_mut().resource_mut::<AudioConfig>().master_volume = 0.0;
    app.world_mut().resource_mut::<AudioConfig>().idle_frequency = 0.0;
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/window-smoke");
    std::fs::create_dir_all(&directory).expect("create smoke-test output directory");
    let screenshot_path = directory.join(if transparent {
        "transparent.png"
    } else {
        "opaque.png"
    });
    app.insert_resource(WindowSmokeTest {
        screenshot_path,
        input_sent: false,
        released: false,
        capture_started: false,
        reported: false,
        max_step: 0.0,
        previous_positions: HashMap::new(),
    })
    .add_systems(Update, inject_settings_shortcut)
    .add_systems(
        PostUpdate,
        capture_window_output.after(EguiPostUpdateSet::ProcessOutput),
    )
    .run()
}

fn inject_settings_shortcut(
    time: Res<Time<Real>>,
    window: Single<Entity, With<PrimaryWindow>>,
    mut state: ResMut<WindowSmokeTest>,
    mut keyboard: MessageWriter<KeyboardInput>,
) {
    if time.elapsed_secs() < 1.0 || state.released {
        return;
    }
    let button_state = if state.input_sent {
        ButtonState::Released
    } else {
        ButtonState::Pressed
    };
    keyboard.write(KeyboardInput {
        key_code: KeyCode::KeyQ,
        logical_key: Key::Character("q".into()),
        state: button_state,
        text: None,
        repeat: false,
        window: *window,
    });
    if state.input_sent {
        state.released = true;
    }
    state.input_sent = true;
}

fn capture_window_output(
    mut commands: Commands,
    time: Res<Time<Real>>,
    settings: Res<SettingsOpen>,
    output: Query<&EguiRenderOutput, With<PrimaryEguiContext>>,
    speakis: Query<(Entity, &Transform, &Velocity), With<Speaki>>,
    mut state: ResMut<WindowSmokeTest>,
    mut exit: MessageWriter<AppExit>,
) {
    for (entity, transform, velocity) in &speakis {
        let position = transform.translation.truncate();
        if let Some(previous) = state.previous_positions.insert(entity, position) {
            let step = previous.distance(position);
            state.max_step = state.max_step.max(step);
            if step > 100.0 {
                println!(
                    "SMOKE discontinuity entity={entity:?} distance={step:.1} position={position:?} velocity=({}, {}) dt={:.4}",
                    velocity.x,
                    velocity.y,
                    time.delta_secs()
                );
            }
        }
    }
    if time.elapsed_secs() >= 3.0 && !state.reported {
        println!(
            "SMOKE settings_open={} primary_contexts={} paint_jobs={} max_position_step={:.2}",
            settings.0,
            output.iter().count(),
            output
                .iter()
                .map(|output| output.paint_jobs.len())
                .sum::<usize>(),
            state.max_step
        );
        state.reported = true;
    }
    if time.elapsed_secs() >= 5.0 && !state.capture_started {
        state.capture_started = true;
        let path = state.screenshot_path.clone();
        commands.spawn(Screenshot::primary_window()).observe(
            move |capture: On<ScreenshotCaptured>, mut exit: MessageWriter<AppExit>| {
                let image = capture
                    .image
                    .clone()
                    .try_into_dynamic()
                    .expect("decode GPU screenshot");
                image.save(&path).expect("save GPU screenshot");
                let pixels = image.to_rgba8();
                let bright_ui_pixels = (20..pixels.height().min(500))
                    .flat_map(|y| {
                        (pixels.width().saturating_sub(400)..pixels.width().saturating_sub(10))
                            .map(move |x| (x, y))
                    })
                    .filter(|&(x, y)| {
                        let pixel = pixels.get_pixel(x, y);
                        pixel[3] > 128 && pixel[0].max(pixel[1]).max(pixel[2]) > 120
                    })
                    .count();
                println!(
                    "SMOKE screenshot={} bright_ui_pixels={bright_ui_pixels}",
                    path.display()
                );
                exit.write(if bright_ui_pixels >= 100 {
                    AppExit::Success
                } else {
                    AppExit::error()
                });
            },
        );
    }
    if time.elapsed_secs() >= 20.0 {
        eprintln!("SMOKE failed: GPU screenshot timed out");
        exit.write(AppExit::error());
    }
}
