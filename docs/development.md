# Development

Run all commands from the repository root.
Install Rust stable with Rust 1.97.1 or newer. Install [just](https://github.com/casey/just).
The toolchain file includes rustfmt, Clippy, and the `wasm32-unknown-unknown` target.
On Windows, install the Visual Studio C++ build tools and Windows SDK.
On Linux, install the ALSA, Wayland, XKB, and X11 development packages used by CI.
Install [Trunk](https://trunkrs.dev/) only when you need to run or package the browser app.

## Commands

| Command | Expected result |
| --- | --- |
| `just` | List the available commands. |
| `just fmt` | Format Rust source. |
| `just check` | Check native source and test targets. |
| `just verify` | Pass formatting, Clippy, behavior tests, and the browser compile check. |
| `just build` | Build the native debug executable. |
| `just build-release` | Build the native release executable. |
| `just run` | Open the ordinary window. |
| `just transparent` | Open the transparent native window. |
| `just smoke-window` | Inject the Q shortcut and save the ordinary window's GPU output. |
| `just smoke-transparent` | Inject the Q shortcut and save the transparent window's GPU output. |
| `just smoke-stationary` | Count each fixed sprite's pixels in 32 GPU captures. |
| `just probe-renderer` | Show three fixed expressions through the ordinary presentation path for 60 seconds. |
| `just web` | Serve the browser app locally. |
| `just build-web` | Build the GitHub Pages files in `dist`. |

The native executable is `target/debug/speaki-box.exe` on Windows.
The release executable is `target/release/speaki-box.exe`.
Other native platforms omit `.exe`.

## Check a transparent window

1. Put a visible application behind the test window. Use a background with text and contrasting colors.
2. Run `just transparent`. Check that the background application remains visible around the speakis.
3. Press `Q`. Open Window settings. Change BG Alpha and check that only the background opacity changes.
4. Return BG Alpha to 0. Check that the empty area has no black or colored rectangle.
5. Change Bloom and create a shiny speaki. Check sprite edges and the surrounding transparent area.
6. Grab a speaki and release it over the settings panel. Check that it stops following the cursor.
7. Run `just run`. Check that the ordinary background remains opaque.

On Windows, renderer logs must report DX12 for transparent mode.
Logs and compilation alone do not prove that desktop composition works.
Record the GPU, driver, platform, checks, and observations in the validation report.
Do not claim macOS, Linux, or browser UI behavior from a Windows check.

## Capture the native renderer

Run `just smoke-window` or `just smoke-transparent` from an interactive native desktop.
The example injects a Bevy `KeyboardInput` message for Q. It records the settings state, egui paint commands, and the largest per-frame position change.
It disables shiny spawning and idle audio to keep the check focused.
The example saves the final GPU image to `target/window-smoke/opaque.png` or `target/window-smoke/transparent.png` and closes its own window.
Inspect the image for the Settings panel. Compare the log with the image when a shortcut changes state without producing visible UI.
This check does not exercise operating-system keyboard delivery or prove desktop composition.

Use `just smoke-stationary` to compare three different sprite expressions over 32 captured frames.
The check fails if a sprite region loses its bright opaque pixels. It saves the first frame and each failed frame.
Bevy captures through an intermediate texture and copies that texture to the window. This extra copy can mask a direct presentation problem.
Use `just probe-renderer` to inspect the same fixed scene without GPU capture.
Use `just probe-renderer --no-msaa` to compare single-sample rendering, or `just probe-renderer --ldr` to remove HDR post-processing.
