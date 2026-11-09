# Application architecture

`main.rs` calls the runtime. `runtime.rs` reads startup options and prepares the native presentation environment.
`app.rs` builds the Bevy app. It installs plugins and declares cross-feature schedule order.
`schedule.rs` defines the shared system sets.

| Module | Responsibility |
| --- | --- |
| `assets` | Embed files and load sprite and voice handles. Define the sprite transition table. |
| `window` | Own window settings, native window setup, shortcuts, and window inertia. |
| `rendering` | Select the renderer, create the camera, and apply background and bloom settings. |
| `speaki` | Own speaki state, spawning, pointer interaction, simulation, animation, and effects. |
| `audio` | Own volume settings, produce action voices, and track playback. |
| `settings` | Load the UI font and render the egui settings panel. |

Each feature keeps registration in `plugin.rs`. Module indexes only declare and export modules.
Configuration lives with the feature that owns the behavior.
Messages describe an action or an observed result. Systems apply the change in a declared order.

## Schedule contract

Startup loads the asset handles before spawning speakis.
Each `Update` runs these system sets in order:

1. Window input and settings shortcuts.
2. Pointer input and unconditional drag cleanup.
3. Spawn and despawn requests.
4. Shiny explosions and shockwave animation.
5. Bounce, merge, and idle voice requests.
6. Playback state checks and new voice playback.
7. Expression, blink, glow, and sprite synchronization.
8. Window and camera settings synchronization.

The 60 Hz `FixedUpdate` runs gravity, movement, merge, pair collision, rotation, and wall collision in order.
Wall collision uses the rotated sprite's full extents after rotation. This keeps the visible corners inside the window when the sprite fits.
Bevy runs fixed updates independently of the display frame rate.
The virtual clock caps each frame at 50 ms. A long GPU or startup stall cannot trigger fifteen physics steps in one visible frame.
Pointer sampling uses `Time<Real>`. Simulation, blink, and effect timers use the virtual clock.
Window inertia uses the final primary-window position from each event batch and converts physical pixels to logical pixels.
The first batch sets the anchor without applying an impulse.
The egui primary context pass owns font setup and the settings panel.
The renderer runs the egui 2D pass after `Core2dSystems::PostProcess` and before upscaling.
Post-processing swaps the current view texture. Egui must draw into the final texture.

## Windows transparency

`Window.transparent` requests a transparent native window. The surface also needs an alpha-capable swapchain.
The DX12 HWND swapchain supports opaque output. The DirectComposition Visual swapchain supports premultiplied alpha.
The Windows startup code selects Visual before Bevy initializes the renderer.
The rendering module selects DX12 for transparent mode. The window module selects `PreMultiplied`.
The clear color premultiplies linear RGB by background alpha. At alpha 0, RGB is also 0.
Bloom and tone mapping preserve alpha in the selected Bevy release.

On Windows, `rendering/sprite_shader.rs` corrects the embedded Bevy 0.20.1 sprite vertex shader before `RenderStartup` loads it.
Naga 30.0.1 adds `first_instance` twice when an input builtin is stored in a named `let` expression and then read again.
The shader uses a function-local `var` for `instance_index`. The HLSL writer then applies the offset once.
This preserves the standard sprite materials, batching, MSAA, HDR, and bloom behavior.
The unit tests check the registered shader and compile the index expression through the same Naga HLSL backend used by wgpu.
When Bevy or Naga changes, revalidate this correction. A changed upstream shader fails the registration test with an explicit message.

macOS uses `PostMultiplied`. Other native platforms use `PreMultiplied`.
These platform choices require native validation on each target system.

References: [Bevy Windows transparency report](https://github.com/bevyengine/bevy/issues/25366), [wgpu DX12 swapchains](https://docs.rs/wgpu/30.0.1/wgpu/enum.Dx12SwapchainKind.html).
