use bevy::prelude::*;
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use bevy_kira_audio::AudioSource;

use crate::schedule::StartupSet;

pub struct EmbeddedGameAssetsPlugin;

impl Plugin for EmbeddedGameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EmbeddedAssetPlugin {
            mode: PluginMode::ReplaceDefault,
        })
        .init_resource::<SpriteAssets>()
        .init_resource::<AudioAssets>()
        .init_resource::<ImageGroups>()
        .init_resource::<VoiceGroups>()
        .add_systems(Startup, load_assets.in_set(StartupSet::LoadAssets));
    }
}

/// Image state node for animation state machine
#[derive(Clone)]
pub struct ImageStateNode {
    pub handle: Handle<Image>,
    pub eye_open: Option<usize>,
    pub eye_close: Option<usize>,
    pub mouth_open: Option<usize>,
    pub mouth_close: Option<usize>,
}

impl ImageStateNode {
    pub fn new(handle: Handle<Image>) -> Self {
        Self {
            handle,
            eye_open: None,
            eye_close: None,
            mouth_open: None,
            mouth_close: None,
        }
    }
}

/// Sprite assets storage
#[derive(Resource, Default)]
pub struct SpriteAssets {
    pub states: Vec<ImageStateNode>,
}

/// Image groups for different situations
#[derive(Resource)]
pub struct ImageGroups {
    pub sad: Vec<usize>,
    pub idle: Vec<usize>,
    pub idle2: Vec<usize>,
}

impl Default for ImageGroups {
    fn default() -> Self {
        Self {
            sad: vec![10], // Sad expression with open eyes
            idle: vec![0, 2, 3, 4, 5, 6, 7, 8, 9],
            idle2: vec![12, 14, 16, 18],
        }
    }
}

/// Audio assets storage
#[derive(Resource, Default)]
pub struct AudioAssets {
    pub voices: Vec<Handle<AudioSource>>,
}

/// Voice groups for different situations
#[derive(Resource)]
pub struct VoiceGroups {
    pub drag: Vec<usize>,
    pub bounce: Vec<usize>,
    pub create: Vec<usize>,
    pub remove: Vec<usize>,
    pub idle: Vec<usize>,
    pub idle2: Vec<usize>,
}

impl Default for VoiceGroups {
    fn default() -> Self {
        // Based on config.js voice mappings
        Self {
            drag: vec![0, 1, 2, 3],            // dontpress, tryhard, speakifull, speakif
            bounce: vec![16],                  // sc2e
            create: vec![4],                   // speaki
            remove: vec![15, 16],              // sc2s, sc2e (random)
            idle: vec![5, 6, 7, 8, 9, 10, 11], // g1, g2, g3, gs1, gs2, gs3, gs4
            idle2: vec![12, 14],               // sc1, sc2
        }
    }
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
}
