use bevy::prelude::*;

use crate::assets::EmbeddedGameAssetsPlugin;
use crate::audio::plugin::VoicePlaybackPlugin;
use crate::rendering::plugin::{RenderingPlugin, render_plugin};
use crate::schedule::{StartupSet, UpdateSet};
use crate::settings::plugin::SettingsPlugin;
use crate::speaki::plugin::SpeakiPlugin;
use crate::window::config::WindowConfig;
use crate::window::plugin::{DesktopWindowPlugin, window_plugin};

pub(crate) fn build_app(window_config: WindowConfig) -> App {
    let primary_window = window_plugin(&window_config);
    let renderer = render_plugin(&window_config);
    let mut app = App::new();
    app.configure_sets(Startup, (StartupSet::LoadAssets, StartupSet::Spawn).chain())
        .configure_sets(
            Update,
            (
                UpdateSet::WindowInput,
                UpdateSet::PointerInput,
                UpdateSet::Spawn,
                UpdateSet::Effects,
                UpdateSet::VoiceRequests,
                UpdateSet::Playback,
                UpdateSet::Animation,
                UpdateSet::Presentation,
            )
                .chain(),
        )
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(ClearColor(window_config.clear_color()))
        .insert_resource(window_config)
        .add_plugins(EmbeddedGameAssetsPlugin)
        .add_plugins(DefaultPlugins.set(primary_window).set(renderer))
        .add_plugins((
            DesktopWindowPlugin,
            SpeakiPlugin,
            VoicePlaybackPlugin,
            RenderingPlugin,
            SettingsPlugin,
        ));
    app
}
