use bevy::prelude::*;
use bevy_egui::input::egui_wants_any_pointer_input;

use super::animation::{
    blink_system, change_to_normal_system, change_to_sad_system, shiny_glow_system,
    sprite_update_system,
};
use super::config::{BorderConfig, MergeConfig, PhysicsConfig, ShinyConfig, SpeakiConfig};
use super::effects::{shiny_explosion_system, shockwave_animation_system};
use super::input::{
    DragState, drag_release_system, drag_update_system, mouse_input_system,
    right_click_delete_system,
};
use super::messages::{DespawnSpeaki, PlayVoice, SpawnSpeaki, SpeakiMerged, WallBounce};
use super::physics::collision::speaki_collision_system;
use super::physics::merge::speaki_merge_system;
use super::physics::simulation::{
    gravity_system, movement_system, rotation_system, wall_collision_system,
};
use super::spawn::{SpawnOrder, despawn_speaki_system, spawn_initial_speakis, spawn_speaki_system};
use crate::schedule::{StartupSet, UpdateSet};

pub struct SpeakiPlugin;

impl Plugin for SpeakiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpawnOrder>()
            .init_resource::<SpeakiConfig>()
            .init_resource::<PhysicsConfig>()
            .init_resource::<BorderConfig>()
            .init_resource::<DragState>()
            .init_resource::<ShinyConfig>()
            .init_resource::<MergeConfig>()
            .add_message::<SpawnSpeaki>()
            .add_message::<DespawnSpeaki>()
            .add_message::<PlayVoice>()
            .add_message::<WallBounce>()
            .add_message::<SpeakiMerged>()
            .add_systems(Startup, spawn_initial_speakis.in_set(StartupSet::Spawn))
            .add_systems(
                Update,
                (
                    (mouse_input_system, right_click_delete_system)
                        .chain()
                        .run_if(not(egui_wants_any_pointer_input)),
                    drag_update_system,
                    drag_release_system,
                )
                    .chain()
                    .in_set(UpdateSet::PointerInput),
            )
            .add_systems(
                Update,
                (spawn_speaki_system, despawn_speaki_system)
                    .chain()
                    .in_set(UpdateSet::Spawn),
            )
            .add_systems(
                FixedUpdate,
                (
                    gravity_system,
                    movement_system,
                    speaki_merge_system,
                    speaki_collision_system,
                    wall_collision_system,
                    rotation_system,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (shiny_explosion_system, shockwave_animation_system)
                    .chain()
                    .in_set(UpdateSet::Effects),
            )
            .add_systems(
                Update,
                (
                    change_to_normal_system,
                    change_to_sad_system,
                    blink_system,
                    shiny_glow_system,
                    sprite_update_system,
                )
                    .chain()
                    .in_set(UpdateSet::Animation),
            );
    }
}
