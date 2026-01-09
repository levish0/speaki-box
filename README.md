# speaki-box

<p align="center">
  <img src="assets/img/speaki1.png" alt="Speaki" width="200" style="border-radius: 20px;">
</p>

A pet application featuring Speaki(스피키) characters with physics simulation.

## Features

### Physics Simulation
- **Gravity**: Speaki(s) fall naturally with configurable gravity strength
- **Bounce**: Wall collisions with adjustable bounce coefficient
- **Friction**: Ground friction affects sliding behavior
- **Speaki-to-Speaki Collision**: Impulse-based collision between multiple speakis with damping

### Drag & Throw Mechanics
- Grab speakis by clicking on them
- Throw speakis with velocity based on mouse movement
- Throwing power is configurable

### Window Inertia
- Speakis react when you move or shake the window
- Creates a "snow globe" effect where speakis bounce around when the window is dragged
- Adjustable inertia strength (or disable completely)

### Animation
- **Eye Blink**: Random blinking with double-blink variations
- **Mouth Animation**: Mouth moves in sync with voice playback
- **Sprite State Machine**: Multiple sprite states with smooth transitions

### Audio
- Voice sounds for various actions (grab, throw, bounce, idle)
- Per-action volume controls
- Random voice selection for variety

### Configurable Settings
All parameters can be adjusted in real-time via the settings panel (`Q` key):

| Category | Parameters |
|----------|------------|
| Audio | Master volume, grab/bounce/create/remove/idle volumes, idle frequency |
| Physics | Gravity, bounce, friction, rotation speed, collision damping, throwing power, window inertia |
| Speaki | Size, click-to-add toggle, eye blink toggle |
| Window | Background color, title bar visibility |
| Border | Left/right/up/down margins |

## Controls

### Mouse
| Action | Description |
|--------|-------------|
| Left Click (empty space) | Create new Speaki |
| Left Click (on Speaki) | Grab Speaki |
| Drag + Release | Throw Speaki |
| Right Click | Delete Speaki |

### Keyboard
| Shortcut | Description |
|----------|-------------|
| `Q` | Toggle settings window |
| `Alt + T` | Toggle title bar |
| `Alt + Left Click` | Drag window (useful when title bar is hidden) |

## Running

```bash
# Normal mode
cargo run

# Transparent window mode (platform-dependent support)
cargo run -- --transparent

# Or using environment variable
SPEAKI_TRANSPARENT=1 cargo run
```

> Note: Transparent window may not work on all platforms. Known to have issues on Windows 11 with NVIDIA GPUs.

## Settings

Press `Q` to open settings. Available options:

- **Audio**: Volume controls for various sounds
- **Physics**: Gravity, bounce, friction, collision settings, window inertia
- **Speaki**: Size, click-to-add toggle, eye blink
- **Window**: Background color, title bar toggle
- **Border**: Boundary margins

## Building

```bash
cargo build --release
```

## License

MIT