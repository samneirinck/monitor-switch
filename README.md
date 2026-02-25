# Monitor Switch

A cross-platform application to switch monitor inputs via DDC/CI.

Perfect for multi-computer setups where you want to quickly switch your monitor between different sources (laptop, desktop, dock, etc.) without reaching for physical buttons.

## Features

- **Quick Switch favorites** - Pin your most-used inputs for one-click switching
- **Input aliases** - Name your inputs (e.g., "HDMI 1" → "Work Laptop")
- **Multi-monitor support** - Control multiple monitors independently
- **Cross-platform** - Native apps for macOS and Linux
- **Wayland native** - GTK4/libadwaita on Linux, perfect for Hyprland/Sway

## Supported Platforms

| Platform | UI Framework |
|----------|--------------|
| macOS | SwiftUI (menu bar app) |
| Linux | GTK4 + libadwaita |

## Installation

### macOS

Download `MonitorSwitch-macos.zip` from [Releases](../../releases), unzip, and drag to Applications.

Or build from source:
```bash
make macos
cp -r apps/macos/MonitorSwitch.app /Applications/
```

### Linux (Hyprland / Wayland)

Download `monitor-switch` from [Releases](../../releases) and place in your PATH:
```bash
chmod +x monitor-switch
sudo mv monitor-switch /usr/local/bin/
```

Or build from source:
```bash
# Install dependencies (Arch)
sudo pacman -S gtk4 libadwaita

# Install dependencies (Ubuntu/Debian)
sudo apt install libgtk-4-dev libadwaita-1-dev libi2c-dev

# Build
make linux
sudo cp apps/linux/target/release/monitor-switch /usr/local/bin/
```

**Required:** Add yourself to the `i2c` group for DDC/CI access:
```bash
sudo usermod -aG i2c $USER
# Log out and back in
```

## Hyprland Integration

Add a keybind to launch the app:

```bash
# ~/.config/hypr/hyprland.conf
bind = $mod, M, exec, monitor-switch
```

### Waybar

```json
// ~/.config/waybar/config
"custom/monitor": {
    "format": "󰍹",
    "on-click": "monitor-switch",
    "tooltip": "Switch monitor input"
}
```

The app opens as a popup window. Click an input to switch, then close the window.

## Usage

1. **Launch the app** - Click the menu bar icon (macOS) or run `monitor-switch` (Linux)
2. **Switch inputs** - Click any input to switch immediately
3. **Set favorites** - Open Preferences, check ⭐ next to frequently used inputs
4. **Create aliases** - In Preferences, name your inputs (e.g., "Work Laptop")
5. **Launch at login** - Enable in the app to start automatically

## Configuration

Config is managed through the Preferences UI, but you can also edit directly:

**Location:** `~/.config/monitor-switch/config.json`

```json
{
  "monitors": {
    "DELL U2720Q-ABC123": {
      "input_aliases": {
        "17": "Work Laptop",
        "21": "MacBook"
      }
    }
  },
  "favorites": [
    { "monitor_id": "DELL U2720Q-ABC123", "input_value": 17 },
    { "monitor_id": "DELL U2720Q-ABC123", "input_value": 21 }
  ]
}
```

<details>
<summary>Input VCP Values Reference</summary>

| Input | VCP Value |
|-------|-----------|
| VGA 1/2 | 1, 2 |
| DVI 1/2 | 3, 4 |
| DisplayPort 1/2 | 15, 16 |
| HDMI 1/2/3/4 | 17, 18, 19, 20 |
| USB-C 1/2/3 | 21, 22, 23 |

</details>

## How It Works

The app uses DDC/CI (Display Data Channel Command Interface) to communicate with monitors over the display cable. VCP code `0x60` controls input source selection.

## Building from Source

### Prerequisites

- Rust 1.70+
- macOS: Xcode Command Line Tools, Swift 5.9+
- Linux: `gtk4`, `libadwaita`, `libi2c-dev`

```bash
# macOS
make macos

# Linux
make linux
```

## License

MIT

