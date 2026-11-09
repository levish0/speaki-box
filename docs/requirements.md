# Behavior requirements

## Native window

The app opens a 1280 × 720 logical-pixel window and spawns three speakis.
The ordinary window has an opaque background and a title bar.
The user selects transparent mode with `--transparent` or `SPEAKI_TRANSPARENT=1` before startup.
The environment variable also accepts `true`, without case sensitivity.
Transparent mode starts without a title bar and with background alpha 0.
The user can change background color and alpha in the settings panel.
The opacity control changes the background. Speakis and settings keep their own opacity.

On Windows, transparent mode selects DX12, the DirectComposition Visual swapchain, and premultiplied alpha.
The app does not change graphics driver settings or Windows settings.
Window transparency is fixed at window creation. Background alpha remains adjustable.

## Interaction and simulation

The left mouse button creates or grabs a speaki. Releasing the button throws a held speaki.
The right mouse button removes the speaki under the cursor.
The visible top speaki receives the pointer action when speakis overlap.
The app releases a held speaki when the window loses focus or the left button is no longer held.
Settings input must not block drag cleanup.
Either Alt key with the left mouse button moves the native window.
Either Alt key with `T` toggles the title bar. `Q` toggles settings.

Gravity, movement, collisions, merge, and rotation run at 60 simulation ticks per second.
Display refresh rate must not change simulation speed while frames stay within the 50 ms budget.
After a long frame, the simulation advances by at most 50 ms. It discards additional elapsed time.
If the frame rate stays below 20 fps, virtual gameplay time slows instead of jumping forward.
Pointer throw sampling uses real time so a render stall cannot amplify the throw.
The first batch of native window positions establishes the inertia anchor and does not push speakis.
Held speakis follow the cursor. Collisions and walls must not displace them.
Merge mode combines nearby speakis of similar size. It preserves a shiny participant and places the result at the midpoint.

## Assets, audio, and browser

The executable embeds the sprite images, voices, and font. It does not need an adjacent asset directory.
Voice playback follows action volume and master volume settings.
A queued voice remains associated with its speaki until playback stops.
The browser build uses the same game behavior and embedded assets.
Native window transparency, icon changes, and native window dragging do not apply to the browser.
