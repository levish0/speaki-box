use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

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
