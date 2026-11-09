use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use crate::audio::config::AudioConfig;
use crate::speaki::config::{BorderConfig, MergeConfig, PhysicsConfig, ShinyConfig, SpeakiConfig};
use crate::window::config::{WindowConfig, WindowPositionTracker};

#[derive(Resource, Default)]
pub struct SettingsOpen(pub bool);

/// Toggle settings with Q key
pub fn toggle_settings_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut settings_open: ResMut<SettingsOpen>,
) {
    if keyboard.just_pressed(KeyCode::KeyQ) {
        settings_open.0 = !settings_open.0;
    }
}

/// Render settings UI
pub fn settings_ui_system(
    mut contexts: EguiContexts,
    settings_open: Res<SettingsOpen>,
    mut audio_config: ResMut<AudioConfig>,
    mut physics_config: ResMut<PhysicsConfig>,
    mut game_config: ResMut<SpeakiConfig>,
    mut window_config: ResMut<WindowConfig>,
    mut border_config: ResMut<BorderConfig>,
    mut window_tracker: ResMut<WindowPositionTracker>,
    mut shiny_config: ResMut<ShinyConfig>,
    mut merge_config: ResMut<MergeConfig>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    if !settings_open.0 {
        return Ok(());
    }

    let content_rect = ctx.content_rect();

    egui::Window::new("Settings")
        .default_width(220.0)
        .default_pos([content_rect.right() - 230.0, 10.0])
        .resizable(true)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .max_height(500.0)
                .show(ui, |ui| {
                    egui::CollapsingHeader::new("Audio")
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.add(
                                egui::Slider::new(&mut audio_config.master_volume, 0.0..=1.0)
                                    .text("Master"),
                            );
                            ui.add(
                                egui::Slider::new(&mut audio_config.grab_volume, 0.0..=1.0)
                                    .text("Grab"),
                            );
                            ui.add(
                                egui::Slider::new(&mut audio_config.bounce_volume, 0.0..=1.0)
                                    .text("Bounce"),
                            );
                            ui.add(
                                egui::Slider::new(&mut audio_config.create_volume, 0.0..=1.0)
                                    .text("Create"),
                            );
                            ui.add(
                                egui::Slider::new(&mut audio_config.remove_volume, 0.0..=1.0)
                                    .text("Remove"),
                            );
                            ui.add(
                                egui::Slider::new(&mut audio_config.idle_volume, 0.0..=1.0)
                                    .text("Idle"),
                            );
                            ui.add(
                                egui::Slider::new(&mut audio_config.idle_frequency, 0.0..=1.0)
                                    .text("Idle Freq"),
                            );
                        });

                    egui::CollapsingHeader::new("Physics")
                        .default_open(false)
                        .show(ui, |ui| {
                            ui.add(
                                egui::Slider::new(&mut physics_config.gravity, 0.0..=2.0)
                                    .text("Gravity"),
                            );
                            ui.add(
                                egui::Slider::new(&mut physics_config.bounce, 0.0..=1.0)
                                    .text("Bounce"),
                            );
                            ui.add(
                                egui::Slider::new(&mut physics_config.friction, 0.0..=1.0)
                                    .text("Friction"),
                            );
                            ui.add(
                                egui::Slider::new(&mut physics_config.rotation_speed, 0.0..=1.0)
                                    .text("Rotation"),
                            );
                            ui.checkbox(&mut physics_config.collision_enabled, "Collision");
                            ui.add(
                                egui::Slider::new(&mut physics_config.collision_damping, 0.0..=1.0)
                                    .text("Col Damp"),
                            );
                            ui.add(
                                egui::Slider::new(&mut physics_config.cursor_impulse, 0.0..=50.0)
                                    .text("Impulse"),
                            );
                            ui.add(
                                egui::Slider::new(
                                    &mut physics_config.cursor_throwing_power,
                                    0.0..=3.0,
                                )
                                .text("Throw"),
                            );
                            ui.add(
                                egui::Slider::new(
                                    &mut physics_config.bounce_responsiveness,
                                    0.0..=2.0,
                                )
                                .text("Bounce Resp"),
                            );
                            ui.separator();
                            ui.checkbox(&mut window_tracker.enabled, "Window Inertia");
                            ui.add(
                                egui::Slider::new(&mut window_tracker.strength, 0.0..=1.0)
                                    .text("Inertia"),
                            );
                        });

                    egui::CollapsingHeader::new("スピキ")
                        .default_open(false)
                        .show(ui, |ui| {
                            ui.add(
                                egui::Slider::new(&mut game_config.speaki_size, 50.0..=400.0)
                                    .text("Size"),
                            );
                            ui.checkbox(&mut game_config.click_to_add, "Click to Add");
                            ui.checkbox(&mut game_config.eye_blink_enabled, "Eye Blink");
                            ui.separator();
                            ui.label("Shiny");
                            ui.checkbox(&mut shiny_config.enabled, "Enabled");
                            ui.checkbox(&mut shiny_config.bloom_enabled, "Bloom");
                            ui.add(
                                egui::Slider::new(&mut shiny_config.spawn_chance, 0.0..=1.0)
                                    .text("Chance")
                                    .custom_formatter(|v, _| format!("{:.0}%", v * 100.0)),
                            );
                            ui.horizontal(|ui| {
                                ui.label("Color");
                                ui.color_edit_button_rgb(&mut shiny_config.glow_color);
                            });
                            ui.add(
                                egui::Slider::new(&mut shiny_config.glow_intensity, 1.0..=10.0)
                                    .text("Glow"),
                            );
                            ui.add(
                                egui::Slider::new(&mut shiny_config.pulse_speed, 0.0..=5.0)
                                    .text("Pulse"),
                            );
                            ui.separator();
                            ui.label("Explosion");
                            ui.checkbox(&mut shiny_config.explosion_enabled, "Enabled");
                            ui.checkbox(&mut shiny_config.shockwave_enabled, "Shockwave");
                            ui.add(
                                egui::Slider::new(
                                    &mut shiny_config.explosion_radius,
                                    100.0..=800.0,
                                )
                                .text("Radius"),
                            );
                            ui.add(
                                egui::Slider::new(&mut shiny_config.explosion_force, 10.0..=150.0)
                                    .text("Force"),
                            );
                            ui.add(
                                egui::Slider::new(
                                    &mut shiny_config.explosion_interval_min,
                                    1.0..=10.0,
                                )
                                .text("Min Interval"),
                            );
                            ui.add(
                                egui::Slider::new(
                                    &mut shiny_config.explosion_interval_max,
                                    2.0..=20.0,
                                )
                                .text("Max Interval"),
                            );
                            shiny_config.explosion_interval_max = shiny_config
                                .explosion_interval_max
                                .max(shiny_config.explosion_interval_min);
                            ui.separator();
                            ui.label("Merge (Suika)");
                            ui.checkbox(&mut merge_config.enabled, "Enabled");
                            ui.add(
                                egui::Slider::new(&mut merge_config.size_tolerance, 0.0..=0.5)
                                    .text("Tolerance")
                                    .custom_formatter(|v, _| format!("{:.0}%", v * 100.0)),
                            );
                            ui.add(
                                egui::Slider::new(&mut merge_config.growth_factor, 1.0..=2.0)
                                    .text("Growth"),
                            );
                            ui.add(
                                egui::Slider::new(&mut merge_config.max_size, 200.0..=800.0)
                                    .text("Max Size"),
                            );
                            ui.add(
                                egui::Slider::new(&mut merge_config.merge_impulse, 0.0..=20.0)
                                    .text("Pop Force"),
                            );
                        });

                    egui::CollapsingHeader::new("Window")
                        .default_open(false)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label("BG");
                                ui.color_edit_button_rgb(&mut window_config.background_color);
                                let [r, g, b] = window_config.background_color;
                                let hex = format!(
                                    "#{:02X}{:02X}{:02X}",
                                    (r * 255.0) as u8,
                                    (g * 255.0) as u8,
                                    (b * 255.0) as u8
                                );
                                ui.label(hex);
                            });

                            #[cfg(not(target_arch = "wasm32"))]
                            {
                                let is_transparent = window_config.is_transparent();
                                // Window transparency is selected before the native window is created.
                                ui.add_enabled(
                                    is_transparent,
                                    egui::Slider::new(
                                        &mut window_config.background_alpha,
                                        0.0..=1.0,
                                    )
                                    .text("BG Alpha"),
                                );

                                if !is_transparent {
                                    ui.label("Restart with --transparent to enable BG Alpha.");
                                }
                                ui.checkbox(&mut window_config.decorations, "Title Bar");
                                ui.checkbox(&mut window_config.fullscreen, "Fullscreen");
                            }
                        });

                    egui::CollapsingHeader::new("Border")
                        .default_open(false)
                        .show(ui, |ui| {
                            ui.add(
                                egui::Slider::new(&mut border_config.left, 0.0..=0.5).text("Left"),
                            );
                            ui.add(
                                egui::Slider::new(&mut border_config.right, 0.0..=0.5)
                                    .text("Right"),
                            );
                            ui.add(egui::Slider::new(&mut border_config.up, 0.0..=0.5).text("Up"));
                            ui.add(
                                egui::Slider::new(&mut border_config.down, 0.0..=0.5).text("Down"),
                            );
                        });
                });
        });

    Ok(())
}
