fn main() -> bevy::app::AppExit {
    let transparent = std::env::args().any(|argument| argument == "--transparent");
    speaki_box::validation::run_window_smoke_test(transparent)
}
