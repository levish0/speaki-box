#[cfg(windows)]
#[allow(unsafe_code)]
pub fn prepare_transparent_presentation(transparent: bool) {
    if transparent {
        // SAFETY: Windows environment mutation is thread-safe. Call before Bevy creates its renderer.
        // DX12 HWND swapchains cannot composite alpha. DirectComposition uses the Visual swapchain.
        unsafe { std::env::set_var("WGPU_DX12_PRESENTATION_SYSTEM", "visual") };
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn set_window_icon(_marker: bevy::ecs::system::NonSendMarker) {
    // Embed logo at compile time
    let icon_bytes = include_bytes!("../../assets/logo.png");

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
