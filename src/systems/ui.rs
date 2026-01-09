use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_egui::egui;

use crate::resources::*;

#[derive(Resource, Default)]
pub struct SettingsOpen(pub bool);

#[derive(Resource, Default)]
pub struct FontLoaded(pub bool);

pub fn setup_fonts_system(
    mut contexts: EguiContexts,
    mut font_loaded: ResMut<FontLoaded>,
) -> Result {
    if font_loaded.0 {
        return Ok(());
    }

    let ctx = contexts.ctx_mut()?;

    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "pretendard-jp".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/PretendardJP-Regular.otf"
        ))
        .into(),
    );

    // Set as primary font for proportional text
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "pretendard-jp".to_owned());

    // Also set for monospace if desired
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "pretendard-jp".to_owned());

    ctx.set_fonts(fonts);
    font_loaded.0 = true;

    Ok(())
}

/// Toggle settings with Q key
pub fn toggle_settings_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut settings_open: ResMut<SettingsOpen>,
) {
    if keyboard.just_pressed(KeyCode::KeyQ) {
        settings_open.0 = !settings_open.0;
    }
}

/// Toggle title bar with Alt + T
pub fn toggle_titlebar_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_config: ResMut<GameConfig>,
) {
    if keyboard.pressed(KeyCode::AltLeft) && keyboard.just_pressed(KeyCode::KeyT) {
        game_config.window_decorations = !game_config.window_decorations;
    }
}

/// Render settings UI
pub fn settings_ui_system(
    mut contexts: EguiContexts,
    settings_open: Res<SettingsOpen>,
    mut audio_config: ResMut<AudioConfig>,
    mut physics_config: ResMut<PhysicsConfig>,
    mut game_config: ResMut<GameConfig>,
    mut border_config: ResMut<BorderConfig>,
    mut window_tracker: ResMut<WindowPositionTracker>,
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
                        });

                    egui::CollapsingHeader::new("Window")
                        .default_open(false)
                        .show(ui, |ui| {
                            let is_transparent = game_config.window_transparent;

                            // BG color: only disabled in transparent mode
                            ui.add_enabled_ui(!is_transparent, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label("BG");
                                    ui.color_edit_button_rgb(&mut game_config.background_color);
                                    let [r, g, b] = game_config.background_color;
                                    let hex = format!(
                                        "#{:02X}{:02X}{:02X}",
                                        (r * 255.0) as u8,
                                        (g * 255.0) as u8,
                                        (b * 255.0) as u8
                                    );
                                    ui.label(hex);
                                });
                            });

                            // BG Alpha: always disabled (transparent window not supported on this platform)
                            ui.add_enabled(
                                false,
                                egui::Slider::new(&mut game_config.background_alpha, 0.0..=1.0)
                                    .text("BG Alpha"),
                            );

                            ui.checkbox(&mut game_config.window_decorations, "Title Bar");
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

/// Sync background color from config to ClearColor
pub fn sync_background_color_system(
    game_config: Res<GameConfig>,
    mut clear_color: ResMut<ClearColor>,
) {
    // Always use Color::NONE in transparent mode
    if game_config.window_transparent {
        clear_color.0 = Color::NONE;
    } else {
        let [r, g, b] = game_config.background_color;
        clear_color.0 = Color::srgba(r, g, b, game_config.background_alpha);
    }
}

/// Sync window settings from config (only decorations, transparency can't change at runtime)
pub fn sync_window_settings_system(game_config: Res<GameConfig>, mut windows: Query<&mut Window>) {
    for mut window in windows.iter_mut() {
        window.decorations = game_config.window_decorations;
    }
}
