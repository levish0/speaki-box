use bevy::prelude::*;
use bevy::window::CompositeAlphaMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransparencyMode {
    Opaque,
    Transparent,
}

#[derive(Resource)]
pub struct WindowConfig {
    pub transparency: TransparencyMode,
    pub background_color: [f32; 3],
    pub background_alpha: f32,
    pub decorations: bool,
    pub fullscreen: bool,
}

impl WindowConfig {
    pub fn new(transparent: bool) -> Self {
        Self {
            transparency: if transparent {
                TransparencyMode::Transparent
            } else {
                TransparencyMode::Opaque
            },
            background_color: [0.1; 3],
            background_alpha: if transparent { 0.0 } else { 1.0 },
            decorations: !transparent,
            fullscreen: false,
        }
    }

    pub fn is_transparent(&self) -> bool {
        self.transparency == TransparencyMode::Transparent
    }

    pub fn composite_alpha_mode(&self) -> CompositeAlphaMode {
        if !self.is_transparent() {
            return CompositeAlphaMode::Auto;
        }
        if cfg!(target_os = "macos") {
            CompositeAlphaMode::PostMultiplied
        } else {
            CompositeAlphaMode::PreMultiplied
        }
    }

    pub fn clear_color(&self) -> Color {
        let [r, g, b] = self.background_color;
        if !self.is_transparent() {
            return Color::srgb(r, g, b);
        }
        let alpha = self.background_alpha;
        if alpha == 0.0 {
            return Color::NONE;
        }
        if self.composite_alpha_mode() == CompositeAlphaMode::PreMultiplied {
            let linear = Color::srgb(r, g, b).to_linear();
            Color::linear_rgba(
                linear.red * alpha,
                linear.green * alpha,
                linear.blue * alpha,
                alpha,
            )
        } else {
            Color::srgba(r, g, b, alpha)
        }
    }
}

#[derive(Resource)]
pub struct WindowPositionTracker {
    pub last_position: Option<IVec2>,
    pub enabled: bool,
    pub strength: f32,
}

impl Default for WindowPositionTracker {
    fn default() -> Self {
        Self {
            last_position: None,
            enabled: true,
            strength: 0.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transparent_background_has_no_residual_color() {
        let config = WindowConfig::new(true);
        assert_eq!(config.clear_color().to_linear(), LinearRgba::NONE);
        assert!(!config.decorations);
        assert_ne!(config.composite_alpha_mode(), CompositeAlphaMode::Opaque);
    }

    #[test]
    fn opaque_window_ignores_background_alpha() {
        let mut config = WindowConfig::new(false);
        config.background_alpha = 0.0;
        assert_eq!(config.clear_color().alpha(), 1.0);
        assert_eq!(config.composite_alpha_mode(), CompositeAlphaMode::Auto);
    }

    #[test]
    fn translucent_background_matches_surface_alpha_mode() {
        let mut config = WindowConfig::new(true);
        config.background_color = [1.0; 3];
        config.background_alpha = 0.5;
        let color = config.clear_color().to_linear();
        assert_eq!(color.alpha, 0.5);
        assert_eq!(color.red, if cfg!(target_os = "macos") { 1.0 } else { 0.5 });
    }
}
