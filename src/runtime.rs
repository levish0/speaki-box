use crate::app::build_app;
use crate::window::config::WindowConfig;

pub fn run() {
    #[cfg(not(target_arch = "wasm32"))]
    let transparent = std::env::args().any(|arg| arg == "--transparent")
        || std::env::var("SPEAKI_TRANSPARENT")
            .is_ok_and(|value| value == "1" || value.eq_ignore_ascii_case("true"));

    #[cfg(target_arch = "wasm32")]
    let transparent = false;

    #[cfg(windows)]
    crate::window::native::prepare_transparent_presentation(transparent);

    build_app(WindowConfig::new(transparent)).run();
}
