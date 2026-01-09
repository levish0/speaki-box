use bevy::prelude::*;
use rand::Rng;

use crate::components::*;
use crate::resources::*;

/// Handle eye blinking animation
pub fn blink_system(
    mut query: Query<(&mut SpriteState, &mut BlinkTimer), With<Speaki>>,
    sprites: Res<SpriteAssets>,
    config: Res<GameConfig>,
    time: Res<Time>,
) {
    if !config.eye_blink_enabled || !sprites.loaded {
        return;
    }

    let current_time = time.elapsed_secs();

    for (mut sprite_state, mut blink) in query.iter_mut() {
        if blink.is_blinking {
            // Currently blinking, check if should open eyes
            if current_time - blink.last_blink_time > blink.blink_open_time {
                // Open eyes
                blink.is_blinking = false;

                if let Some(state) = sprites.states.get(sprite_state.current_index) {
                    if let Some(open_idx) = state.eye_open {
                        sprite_state.current_index = open_idx;
                    }
                }

                if blink.double_blink {
                    // Double blink: short cooldown then blink again
                    blink.double_blink = false;
                    blink.blink_cooldown = 0.07; // 70ms
                } else {
                    // Normal: wait 5-10 seconds
                    blink.blink_cooldown = 5.0 + 5.0 * rand::random::<f32>();
                }

                blink.last_blink_time = current_time;
            }
        } else {
            // Not blinking, check if should close eyes
            if current_time - blink.last_blink_time > blink.blink_cooldown {
                // Close eyes
                blink.is_blinking = true;
                blink.blink_open_time = 0.1; // 100ms

                if let Some(state) = sprites.states.get(sprite_state.current_index) {
                    if let Some(close_idx) = state.eye_close {
                        sprite_state.current_index = close_idx;
                    }
                }

                blink.last_blink_time = current_time;

                // 20% chance of double blink (only if cooldown was long)
                if blink.blink_cooldown > 0.1 && rand::random::<f32>() > 0.8 {
                    blink.double_blink = true;
                }
            }
        }
    }
}

/// Update sprite images based on state
pub fn sprite_update_system(
    mut query: Query<
        (&SpriteState, &SpeakiSize, &mut Sprite),
        (With<Speaki>, Changed<SpriteState>),
    >,
    sprites: Res<SpriteAssets>,
) {
    if !sprites.loaded {
        return;
    }

    for (state, size, mut sprite) in query.iter_mut() {
        if let Some(image_state) = sprites.states.get(state.current_index) {
            sprite.image = image_state.handle.clone();
            sprite.custom_size = Some(Vec2::splat(size.0));
        }
    }
}

/// Change speaki to sad expression (used when dragging)
pub fn change_to_sad_system(
    mut query: Query<&mut SpriteState, Added<Dragged>>,
    image_groups: Res<ImageGroups>,
) {
    for mut state in query.iter_mut() {
        if !image_groups.sad.is_empty() {
            let idx = image_groups.sad[rand::rng().random_range(0..image_groups.sad.len())];
            state.current_index = idx;
        }
    }
}

/// Change speaki back to normal when released
pub fn change_to_normal_system(
    mut removed: RemovedComponents<Dragged>,
    mut query: Query<&mut SpriteState, With<Speaki>>,
) {
    for entity in removed.read() {
        if let Ok(mut state) = query.get_mut(entity) {
            state.current_index = 0; // Back to default
        }
    }
}
