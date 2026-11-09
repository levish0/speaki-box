use speaki_box::validation::{WindowSmokeMode, WindowSmokeOptions, run_window_smoke_test};

fn main() -> bevy::app::AppExit {
    let argument = |flag: &str| std::env::args().any(|value| value == flag);
    run_window_smoke_test(WindowSmokeOptions {
        transparent: argument("--transparent"),
        mode: if argument("--stationary-live") {
            WindowSmokeMode::StationaryLive
        } else if argument("--stationary") {
            WindowSmokeMode::StationarySprites
        } else {
            WindowSmokeMode::Settings
        },
        msaa: if argument("--no-msaa") {
            bevy::prelude::Msaa::Off
        } else {
            bevy::prelude::Msaa::default()
        },
        post_processing: !argument("--ldr"),
    })
}
