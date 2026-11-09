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
use crate::schedule::StartupSet;
use crate::settings::ui::SettingsOpen;
use crate::speaki::components::{Speaki, SpriteState, Velocity};
use crate::speaki::config::{PhysicsConfig, ShinyConfig, SpeakiConfig};
use crate::window::config::WindowConfig;
use crate::window::config::WindowPositionTracker;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowSmokeMode {
    Settings,
    StationarySprites,
    StationaryLive,
}

pub struct WindowSmokeOptions {
    pub transparent: bool,
    pub mode: WindowSmokeMode,
    pub msaa: Msaa,
    pub post_processing: bool,
}

#[derive(Resource)]
struct WindowSmokeTest {
    screenshot_path: PathBuf,
    mode: WindowSmokeMode,
    msaa: Msaa,
    post_processing: bool,
    captures: u32,
    failed_captures: u32,
    input_sent: bool,
    released: bool,
    capture_started: bool,
    reported: bool,
    max_step: f32,
    previous_positions: HashMap<Entity, Vec2>,
}

/// Exercise the native renderer and capture the final GPU output after a Q key message.
pub fn run_window_smoke_test(options: WindowSmokeOptions) -> AppExit {
    let WindowSmokeOptions {
        transparent,
        mode,
        msaa,
        post_processing,
    } = options;
    #[cfg(windows)]
    crate::window::native::prepare_transparent_presentation(transparent);

    let mut app = build_app(WindowConfig::new(transparent));
    app.world_mut().resource_mut::<ShinyConfig>().spawn_chance = 0.0;
    app.world_mut().resource_mut::<AudioConfig>().master_volume = 0.0;
    app.world_mut().resource_mut::<AudioConfig>().idle_frequency = 0.0;
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/window-smoke");
    std::fs::create_dir_all(&directory).expect("create smoke-test output directory");
    let prefix = if transparent { "transparent" } else { "opaque" };
    let suffix = if mode != WindowSmokeMode::Settings {
        "-stationary"
    } else {
        ""
    };
    let screenshot_path = directory.join(format!("{prefix}{suffix}.png"));
    if mode != WindowSmokeMode::Settings {
        let mut physics = app.world_mut().resource_mut::<PhysicsConfig>();
        physics.gravity = 0.0;
        physics.collision_enabled = false;
        physics.rotation_speed = 0.0;
        app.world_mut()
            .resource_mut::<SpeakiConfig>()
            .eye_blink_enabled = false;
        app.world_mut()
            .resource_mut::<WindowPositionTracker>()
            .enabled = false;
        app.add_systems(Startup, place_stationary_speakis.after(StartupSet::Spawn));
    }
    app.insert_resource(WindowSmokeTest {
        screenshot_path,
        mode,
        msaa,
        post_processing,
        captures: 0,
        failed_captures: 0,
        input_sent: false,
        released: false,
        capture_started: false,
        reported: false,
        max_step: 0.0,
        previous_positions: HashMap::new(),
    })
    .add_systems(
        Startup,
        configure_validation_camera.after(crate::rendering::camera::setup_camera),
    )
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
    if state.mode == WindowSmokeMode::StationaryLive {
        if time.elapsed_secs() >= 60.0 {
            exit.write(AppExit::Success);
        }
        return;
    }
    if time.elapsed_secs() >= 5.0 && !state.capture_started {
        state.capture_started = true;
        let path = state.screenshot_path.clone();
        commands.spawn(Screenshot::primary_window()).observe(
            move |capture: On<ScreenshotCaptured>,
                  mut exit: MessageWriter<AppExit>,
                  mut state: ResMut<WindowSmokeTest>| {
                let image = capture
                    .image
                    .clone()
                    .try_into_dynamic()
                    .expect("decode GPU screenshot");

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
                state.captures += 1;
                let valid = if state.mode == WindowSmokeMode::StationarySprites {
                    let counts = sprite_pixel_counts(&pixels);
                    println!(
                        "SMOKE frame={} sprite_pixels={counts:?} ui_pixels={bright_ui_pixels}",
                        state.captures
                    );
                    counts.iter().all(|&count| count >= 200)
                } else {
                    bright_ui_pixels >= 100
                };
                if !valid {
                    state.failed_captures += 1;
                }
                if state.captures == 1 {
                    image.save(&path).expect("save GPU screenshot");
                }
                if !valid {
                    let failure_path = path.with_file_name(format!(
                        "{}-frame-{}.png",
                        path.file_stem().unwrap().to_string_lossy(),
                        state.captures
                    ));
                    image.save(failure_path).expect("save failing GPU frame");
                }
                let target = if state.mode == WindowSmokeMode::StationarySprites {
                    32
                } else {
                    1
                };
                if state.captures >= target {
                    println!(
                        "SMOKE screenshot={} captures={} failed_captures={}",
                        path.display(),
                        state.captures,
                        state.failed_captures
                    );
                    exit.write(if state.failed_captures == 0 {
                        AppExit::Success
                    } else {
                        AppExit::error()
                    });
                } else {
                    state.capture_started = false;
                }
            },
        );
    }
    if time.elapsed_secs() >= 60.0 {
        eprintln!("SMOKE failed: GPU screenshot timed out");
        exit.write(AppExit::error());
    }
}

fn place_stationary_speakis(
    mut query: Query<(&mut Transform, &mut Velocity, &mut SpriteState), With<Speaki>>,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
) {
    window.title = "Speaki Box — stationary renderer test".into();
    for (index, (mut transform, mut velocity, mut sprite_state)) in query.iter_mut().enumerate() {
        transform.translation.x = (index as f32 - 1.0) * 300.0;
        transform.translation.y = 0.0;
        transform.rotation = Quat::IDENTITY;
        velocity.x = 0.0;
        velocity.y = 0.0;
        sprite_state.current_index = [0, 10, 12][index];
    }
}

fn sprite_pixel_counts(image: &image::RgbaImage) -> [u32; 3] {
    let scale = image.width() as f32 / 1280.0;
    let radius = (80.0 * scale).ceil() as u32;
    let center_y = image.height() / 2;
    std::array::from_fn(|index| {
        let center_x =
            (image.width() as f32 * 0.5 + (index as f32 - 1.0) * 300.0 * scale).round() as u32;
        (center_y.saturating_sub(radius)..(center_y + radius).min(image.height()))
            .flat_map(|y| {
                (center_x.saturating_sub(radius)..(center_x + radius).min(image.width()))
                    .map(move |x| (x, y))
            })
            .filter(|&(x, y)| {
                let pixel = image.get_pixel(x, y);
                pixel[3] > 128 && pixel[0].max(pixel[1]).max(pixel[2]) > 120
            })
            .count() as u32
    })
}

fn configure_validation_camera(
    mut commands: Commands,
    cameras: Query<Entity, With<Camera2d>>,
    state: Res<WindowSmokeTest>,
    mut shiny: ResMut<ShinyConfig>,
) {
    for entity in &cameras {
        let mut camera = commands.entity(entity);
        camera.insert(state.msaa);
        if !state.post_processing {
            shiny.bloom_enabled = false;
            camera
                .remove::<(bevy::post_process::bloom::Bloom, bevy::camera::Hdr)>()
                .insert(bevy::render::view::Tonemapping::None);
        }
    }
}
