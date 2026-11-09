# Native window validation — 2026-11-09

## Result

The Windows transparent background works. The user independently confirmed desktop transparency.
The settings panel renders after 2D post-processing. The user confirmed that Q opens it.
The DX12 sprite visibility defect is reproduced and corrected in the native GPU check.
The user confirmed that the final interactive build renders newly created speakis normally and no longer shows intermittent disappearance.

## Environment

- Windows 11 Pro, kernel 26200.
- NVIDIA GeForce RTX 4060 Laptop GPU, driver 591.74.
- Rust 1.99.1. Bevy 0.20.1. wgpu 30.0.1. Naga 30.0.1. bevy_egui 0.43.1.
- Native debug build. The ordinary window used Vulkan. The transparent window used DX12.

## Automated checks

`just verify` passed formatting, native Clippy with warnings denied, sixteen behavior and shader tests, and the WASM compile check.
`cargo build --locked --bin speaki-box --example window_smoke` passed.
The tests include drag cleanup, fixed-step timing, long-frame catch-up, merge state, rotated bounds, startup inertia, the registered Bevy shader, and Naga HLSL output.

## Settings panel regression

The user reported that Q did not show settings in either window mode.
The validation example injected a Bevy `KeyboardInput` message for Q and captured the final GPU image.
Before the render-order fix, `SettingsOpen` was true and egui produced four paint jobs, but the final image contained no panel.
After the fix, the panel appeared in both modes. Each capture contained 5,605 bright UI pixels in the panel region.
The fix orders the egui pass after `Core2dSystems::PostProcess` and before upscaling.

## DX12 sprite visibility regression

The user reported that newly created speakis could be invisible while their creation voices played.
The user also saw sprites disappear, reappear, and appear at incorrect positions in transparent mode.
Disabling Bloom did not remove the defect.

A fixed scene with one expression for all three sprites produced 32 stable frames.
A stronger scene used sprite states 0, 10, and 12 at three fixed, separate positions.
Before the shader correction, all 32 frames missed the middle sprite. The pixel counts were `[31707, 0, 31925]`.
The right sprite displayed the middle sprite's expression. CPU positions did not move.
Disabling texture binding arrays or automatic batching did not correct the scene. Those changes were removed.

A standalone reproduction used the same Naga 30.0.1 library as the app.
For `let instance_index = vertex.instance_index;`, Naga's HLSL writer added `first_instance` when it declared the local and again when it read the cached named expression.
For a function-local `var`, the writer added the offset once.
The app corrects this one token in the registered Bevy sprite shader before the renderer loads it.
The compiler regression test verifies that the HLSL output contains one offset addition.

After the correction, all 32 frames contained all three sprites at the correct locations and with the correct expressions.
Every frame produced pixel counts `[31831, 32177, 32026]`, 5,605 UI pixels, and zero CPU position change.
The native smoke test exited successfully.
The normal sprite material, automatic batching, MSAA, HDR, and Bloom remain enabled.

Images and logs are generated under `target/window-smoke/`.
Use `just smoke-stationary` to repeat the three-expression test.
Bevy captures through an intermediate texture before copying to the real back buffer. Capture alone does not prove desktop composition.
Here, the incorrect sprite output was present in that capture before the shader change and absent after it.

## Motion and boundary guards

An earlier native trace recorded a 0.3246-second startup frame and a maximum position change of 139.65 pixels.
The app now caps virtual frame time at 50 ms and samples pointer motion with the real clock.
The first batch of native window positions establishes the inertia anchor without applying an impulse.
These guards did not fix the sprite visibility defect; the HLSL correction fixes that defect.

Wall collision now uses the full rotated sprite bounds after rotation.
A regression test checks all four corners of a rotated sprite against the window edges.

## Scope

The user stopped Computer Use with Escape. No later Computer Use actions ran.
Subsequent GPU checks used the app's own validation example and did not inject operating-system input.
Background alpha values between 0 and 1 still need desktop composition checks.
macOS, Linux, and browser UI behavior were not tested on this Windows machine.
The audio decoder logged metadata skip warnings. One earlier interactive run logged `SoundLimitReached` after repeated activity.
Remote CI, publication, and deployment were not run.
