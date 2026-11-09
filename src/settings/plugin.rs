use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};

use super::fonts::{FontLoaded, setup_fonts_system};
use super::ui::{SettingsOpen, settings_ui_system, toggle_settings_system};
use crate::schedule::UpdateSet;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .init_resource::<SettingsOpen>()
            .init_resource::<FontLoaded>()
            .add_systems(
                Update,
                toggle_settings_system.in_set(UpdateSet::WindowInput),
            )
            .add_systems(
                EguiPrimaryContextPass,
                (setup_fonts_system, settings_ui_system).chain(),
            );
    }
}
