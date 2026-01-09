mod components;
mod events;
mod resources;
mod systems;

use bevy::ecs::system::NonSendMarker;
use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass, input::egui_wants_any_pointer_input};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use bevy_kira_audio::prelude::*;

use events::*;
use resources::*;
use systems::*;

fn main() {
    // Check for --transparent flag or SPEAKI_TRANSPARENT env var
    let args: Vec<String> = std::env::args().collect();
    let transparent = args.contains(&"--transparent".to_string())
        || std::env::var("SPEAKI_TRANSPARENT")
            .map(|v| v == "1" || v.to_lowercase() == "true")
            .unwrap_or(false);

    println!("Transparent mode: {}", transparent);

    // Set initial config based on transparent mode
    let mut game_config = GameConfig::default();
    if transparent {
        game_config.background_alpha = 0.0;
        game_config.window_transparent = true;
        game_config.window_decorations = false;
    }

    let clear_color = if transparent {
        ClearColor(Color::NONE)
    } else {
        ClearColor(Color::srgba(
            game_config.background_color[0],
            game_config.background_color[1],
            game_config.background_color[2],
            game_config.background_alpha,
        ))
    };

    App::new()
        .insert_resource(clear_color)
        .add_plugins(EmbeddedAssetPlugin {
            mode: PluginMode::ReplaceDefault,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Speaki Box".to_string(),
                resolution: bevy::window::WindowResolution::new(1280, 720),
                transparent,
                decorations: !transparent, // Hide title bar when transparent
                ..default()
            }),
            ..default()
        }))
        .add_plugins(AudioPlugin)
        .add_plugins(EguiPlugin::default())
        // Resources
        .insert_resource(game_config)
        .init_resource::<PhysicsConfig>()
        .init_resource::<AudioConfig>()
        .init_resource::<BorderConfig>()
        .init_resource::<DragState>()
        .init_resource::<SettingsOpen>()
        .init_resource::<FontLoaded>()
        .init_resource::<SpriteAssets>()
        .init_resource::<AudioAssets>()
        .init_resource::<ImageGroups>()
        .init_resource::<VoiceGroups>()
        .init_resource::<WindowPositionTracker>()
        // Events
        .add_message::<SpawnSpeakiEvent>()
        .add_message::<DespawnSpeakiEvent>()
        .add_message::<PlayVoiceEvent>()
        .add_message::<WallBounceEvent>()
        // Startup
        .add_systems(Startup, (setup_camera, load_assets, set_window_icon))
        .add_systems(Startup, spawn_initial_speakis.after(load_assets))
        // Input systems
        .add_systems(
            Update,
            (
                mouse_input_system,
                drag_update_system,
                drag_release_system,
                right_click_delete_system,
            )
                .chain()
                .run_if(not(egui_wants_any_pointer_input)),
        )
        // Spawn/Despawn
        .add_systems(Update, (spawn_speaki_system, despawn_speaki_system))
        // Physics systems
        .add_systems(
            Update,
            (
                gravity_system,
                movement_system,
                speaki_collision_system,
                wall_collision_system,
                rotation_system,
                window_inertia_system,
            )
                .chain(),
        )
        // Animation systems
        .add_systems(
            Update,
            (
                blink_system,
                sprite_update_system,
                change_to_sad_system,
                change_to_normal_system,
            ),
        )
        // Audio systems
        .add_systems(
            Update,
            (
                play_voice_system,
                bounce_voice_system,
                idle_voice_system,
                mouth_animation_system,
            ),
        )
        // UI systems
        .add_systems(
            Update,
            (
                toggle_settings_system,
                toggle_titlebar_system,
                sync_background_color_system,
                sync_window_settings_system,
                window_drag_system,
            ),
        )
        .add_systems(
            EguiPrimaryContextPass,
            (setup_fonts_system, settings_ui_system).chain(),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn set_window_icon(_marker: NonSendMarker) {
    // Embed logo at compile time
    let icon_bytes = include_bytes!("../assets/logo.png");

    let icon = match image::load_from_memory(icon_bytes) {
        Ok(img) => {
            let rgba = img.into_rgba8();
            let (width, height) = rgba.dimensions();
            let raw = rgba.into_raw();
            winit::window::Icon::from_rgba(raw, width, height).ok()
        }
        Err(_) => None,
    };

    bevy::winit::WINIT_WINDOWS.with_borrow_mut(|winit_windows| {
        for window in winit_windows.windows.values() {
            window.set_window_icon(icon.clone());
        }
    });
}

fn load_assets(
    mut sprites: ResMut<SpriteAssets>,
    mut audio: ResMut<AudioAssets>,
    asset_server: Res<AssetServer>,
) {
    // Load sprite images
    let image_files = [
        "img/speaki1.png",   // 0 - default, eyes open
        "img/speaki1b.png",  // 1 - default, eyes closed
        "img/speaki2.png",   // 2 - idle variation
        "img/speaki3.png",   // 3
        "img/speaki4.png",   // 4
        "img/speaki5.png",   // 5
        "img/speaki6.png",   // 6
        "img/speaki7.png",   // 7
        "img/speaki8.png",   // 8
        "img/speaki9.png",   // 9
        "img/speaki10.png",  // 10 - sad, eyes open
        "img/speaki10b.png", // 11 - sad, eyes closed
        "img/speaki11.png",  // 12
        "img/speaki11b.png", // 13
        "img/speaki12.png",  // 14
        "img/speaki12b.png", // 15
        "img/speaki13.png",  // 16
        "img/speaki13b.png", // 17
        "img/speaki14.png",  // 18
        "img/speaki14b.png", // 19
    ];

    for file in &image_files {
        let handle: Handle<Image> = asset_server.load(*file);
        sprites.states.push(ImageStateNode::new(handle));
    }

    // Set up state machine connections (eye open/close)
    // speaki1 <-> speaki1b
    sprites.states[0].eye_close = Some(1);
    sprites.states[1].eye_open = Some(0);

    // speaki10 <-> speaki10b
    sprites.states[10].eye_close = Some(11);
    sprites.states[11].eye_open = Some(10);

    // speaki11 <-> speaki11b
    sprites.states[12].eye_close = Some(13);
    sprites.states[13].eye_open = Some(12);

    // speaki12 <-> speaki12b
    sprites.states[14].eye_close = Some(15);
    sprites.states[15].eye_open = Some(14);

    // speaki13 <-> speaki13b
    sprites.states[16].eye_close = Some(17);
    sprites.states[17].eye_open = Some(16);

    // speaki14 <-> speaki14b
    sprites.states[18].eye_close = Some(19);
    sprites.states[19].eye_open = Some(18);

    // Set up mouth open/close connections
    sprites.states[0].mouth_close = Some(10);
    sprites.states[1].mouth_close = Some(11);
    sprites.states[2].mouth_close = Some(3);
    sprites.states[3].mouth_open = Some(2);
    sprites.states[4].mouth_close = Some(5);
    sprites.states[5].mouth_open = Some(4);
    sprites.states[6].mouth_open = Some(7);
    sprites.states[7].mouth_close = Some(6);
    sprites.states[8].mouth_close = Some(9);
    sprites.states[9].mouth_open = Some(8);
    sprites.states[10].mouth_open = Some(0);
    sprites.states[11].mouth_open = Some(1);
    // idle2 group
    sprites.states[12].mouth_close = Some(14);
    sprites.states[13].mouth_close = Some(15);
    sprites.states[14].mouth_open = Some(12);
    sprites.states[15].mouth_open = Some(13);
    sprites.states[16].mouth_close = Some(18);
    sprites.states[17].mouth_close = Some(19);
    sprites.states[18].mouth_open = Some(16);
    sprites.states[19].mouth_open = Some(17);

    sprites.loaded = true;

    // Load audio files
    let voice_files = [
        "voice/dontpress.mp3",  // 0 - drag
        "voice/tryhard.mp3",    // 1 - drag
        "voice/speakifull.mp3", // 2 - drag
        "voice/speakif.mp3",    // 3 - drag
        "voice/speaki.mp3",     // 4 - create
        "voice/g1.mp3",         // 5 - idle
        "voice/g2.mp3",         // 6 - idle
        "voice/g3.mp3",         // 7 - idle
        "voice/gs1.mp3",        // 8 - idle
        "voice/gs2.mp3",        // 9 - idle
        "voice/gs3.mp3",        // 10 - idle
        "voice/gs4.mp3",        // 11 - idle
        "voice/sc1.mp3",        // 12 - idle2
        "voice/sc1e.mp3",       // 13
        "voice/sc2.mp3",        // 14 - idle2
        "voice/sc2s.mp3",       // 15 - remove
        "voice/sc2e.mp3",       // 16 - bounce
    ];

    for file in &voice_files {
        let handle: Handle<bevy_kira_audio::AudioSource> = asset_server.load(*file);
        audio.voices.push(handle);
    }

    audio.loaded = true;
}

fn spawn_initial_speakis(
    mut commands: Commands,
    config: Res<GameConfig>,
    sprites: Res<SpriteAssets>,
    window: Single<&Window>,
) {
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

    for _ in 0..config.speaki_count {
        // Random position in top 30% of screen
        let x = (rand::random::<f32>() - 0.5) * 2.0 * half_width;
        let y = half_height * 0.4 + rand::random::<f32>() * half_height * 0.6;

        // Initial velocity
        let vx = (rand::random::<f32>() - 0.5) * 5.0;
        let vy = rand::random::<f32>() * 2.0;

        spawn_speaki(
            &mut commands,
            Vec2::new(x, y),
            Vec2::new(vx, vy),
            config.speaki_size,
            &sprites,
        );
    }
}
