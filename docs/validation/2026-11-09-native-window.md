# Native window validation — 2026-11-09

## Environment

- Windows 11 Pro, kernel 26200.
- NVIDIA GeForce RTX 4060 Laptop GPU, driver 591.74.
- Rust 1.99.1. Bevy 0.20.1. wgpu 30.0.1. bevy_egui 0.43.1.
- Native debug build. The ordinary window used Vulkan. The transparent window used DX12.

## Automated checks

`just verify` passed formatting, native Clippy with warnings denied, ten behavior tests, and the WASM compile check.
`just build` passed.
The later native GPU checks added a validation example and corrected the 2D UI render order.
After the motion guards, `just fmt verify build` passed with thirteen behavior tests and no Clippy or WASM warnings.

## Desktop transparency

The native transparent window showed the Zed editor and desktop through the empty background.
The user independently confirmed that transparency worked.
The user stopped Computer Use with Escape. No further Computer Use actions ran.

## Settings panel regression

The user reported that Q did not show settings in either window mode.
The validation example injected a Bevy `KeyboardInput` message for Q and captured the final GPU image.
Before the render-order fix, `SettingsOpen` was true and egui produced four paint jobs, but the final image contained no panel.
After the fix, the panel appeared in both modes. Each capture contained 5,605 bright UI pixels in the panel region.
The fix orders the egui pass after `Core2dSystems::PostProcess` and before upscaling.

Images are generated under `target/window-smoke/`. Use `just smoke-window` and `just smoke-transparent` to repeat the check.
This test exercises Bevy input processing and GPU rendering. It does not inject operating-system keyboard input.

## Motion report under investigation

The user reported large position jumps in transparent mode while the window was stationary.
The user first reported that restarting removed the symptom, then confirmed that it occurred in transparent mode.
The native trace recorded long startup frames followed by multiple fixed simulation steps.
One DX12 run recorded a 0.3246-second frame and a maximum per-frame position change of 139.65 pixels.
That test disabled shiny spawning and pointer interaction.
The UI render-order fix is verified. The reported motion problem requires a separate change and acceptance check.

The follow-up change caps virtual frame time at 50 ms and samples pointer motion with the real clock.
It also treats the first batch of native positions as an anchor and converts later position deltas to logical pixels.
Regression tests cover a one-second render stall, startup position bursts, and inertia re-enabling.
These guards address recorded long-frame catch-up and a reproducible startup event case.
They do not establish that every reported transparent-window jump has the same cause.

The user then confirmed that Q worked but described intermittent invisible sprites, including newly created speakis with audible creation voices.
The user reported that disabling Bloom did not remove this symptom.
This is an unresolved render visibility problem. The earlier time and startup guards did not resolve it.

A fixed-scene DX12 test captured 32 frames with no CPU movement and no missing sprite region.
All frames contained the same per-sprite bright opaque pixel counts: 31,822, 32,022, and 31,947.
That first fixed scene used one expression for all three speakis. Later probes use three different expressions.
Bevy captures through a temporary render target before copying to the real back buffer. These captures do not prove that the uncaptured presentation path is stable.

## Remaining checks

Background alpha values between 0 and 1 need desktop composition checks.
macOS, Linux, and browser UI behavior were not tested on this Windows machine.
The audio decoder logged metadata skip warnings. One earlier interactive run also logged `SoundLimitReached` after repeated activity.
Remote CI, publication, and deployment were not run.
