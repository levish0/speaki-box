# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.11] - 2026-01-10

### Changed
- macOS build improvements:
  - Added `chmod +x` for binary executable permission
  - Added `xattr -cr` to clear quarantine attributes
  - Info.plist version now auto-extracted from git tag (fallback to Cargo.toml)

## [0.1.10] - 2026-01-10

### Changed
- Icon source format changed from .ico to .png (CI auto-converts to .ico for Windows)
- Fixed CI artifact packaging to avoid double-zipping issue

## [0.1.9] - 2026-01-10

### Added
- Application icon support for all platforms:
  - Windows: Executable icon (.ico) embedded via winres
  - macOS: App bundle with Info.plist and icon
  - Linux: .desktop file with icon
- Runtime window icon (title bar and taskbar)

### Changed
- Unified icon format: using .ico for both build-time and runtime icons

## [0.1.8] - 2026-01-10

### Added
- Embedded assets: all images and audio are now bundled into the binary (no separate assets folder needed)
- Multi-platform CI builds: Windows, macOS (x64/ARM64), Linux
- Automated GitHub Releases on version tags
- README Features section with detailed descriptions of physics, audio, and settings

### Removed
- Double-click delete functionality

## [0.1.6] - 2026-01-10

### Added
- Window inertia physics: speakis react when the window is moved/shaken
- Window inertia settings in Physics section (enabled toggle and strength slider)
- `--transparent` command line flag for transparent window mode
- `SPEAKI_TRANSPARENT` environment variable support
- Alt + Left Click to drag window (works even when title bar is hidden)
- Alt + T keyboard shortcut to toggle title bar
- README with controls and usage documentation

### Changed
- BG Alpha slider is now disabled (transparent windows not supported on Windows + NVIDIA)
- Transparent mode disables BG color/alpha settings in UI

## [0.1.2] - 2026-01-09

### Added
- Custom font support (PretendardJP) for egui UI
- Background color picker with hex value display
- Background alpha slider for window transparency
- Window transparency toggle (requires app restart)
- Window decorations (title bar) toggle
- Scrollable settings window with resizable support
- Settings window is now draggable/movable

### Changed
- Reorganized settings UI into collapsible sections: Audio, Physics, Speaki, Window, Border
- Volume now properly converts amplitude to decibels for bevy_kira_audio

### Fixed
- egui widget clicks now work properly (moved UI system to EguiPrimaryContextPass)
- Speaki spawning is blocked when clicking on egui UI