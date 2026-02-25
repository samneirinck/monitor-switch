# Monitor Switch

A cross-platform system tray application to switch monitor inputs via DDC/CI.

Perfect for multi-computer setups where you want to quickly switch your monitor between different sources (laptop, desktop, dock, etc.) without reaching for physical buttons.

## Features

- **System tray / menu bar app** - Lives quietly in your system tray
- **Quick Switch favorites** - Pin your most-used inputs for one-click switching
- **Input aliases** - Name your inputs (e.g., "HDMI 1" → "Work Laptop")
- **Multi-monitor support** - Control multiple monitors independently
- **Cross-platform** - Native apps for macOS and Linux

## Supported Platforms

| Platform | Status | UI Framework |
|----------|--------|--------------|
| macOS (Apple Silicon) | ✅ | Native Swift/AppKit |
| Linux (Wayland) | ✅ | ksni (StatusNotifierItem) |
| Linux (X11) | ✅ | ksni (StatusNotifierItem) |
| Windows | ❌ | Not planned |

## Building

### Prerequisites

- Rust 1.70+
- macOS: Xcode Command Line Tools
- Linux: Development headers for i2c

### macOS

```bash
make macos
make install-macos
```

### Linux

```bash
make linux
make install-linux
```

**Note:** On Linux, you need permission to access I2C devices:
```bash
sudo usermod -aG i2c $USER
# Log out and back in
```

## Configuration

Config file location: `~/.config/monitor-switch/config.json`

```json
{
  "monitors": {
    "DELL U2515H-0": {
      "input_aliases": {
        "17": "Work Laptop",
        "18": "Personal Mac"
      }
    }
  },
  "favorites": [
    { "monitor_id": "DELL U2515H-0", "input_value": 17 },
    { "monitor_id": "DELL U2515H-1", "input_value": 15 }
  ]
}
```

### Input VCP Values

| Input | VCP Value |
|-------|-----------|
| VGA 1 | 1 |
| DVI 1 | 3 |
| DisplayPort 1 | 15 |
| DisplayPort 2 | 16 |
| HDMI 1 | 17 |
| HDMI 2 | 18 |
| USB-C 1 | 21 |
| USB-C 2 | 22 |

## Project Structure

```
monitor-switch/
├── src/              # Core Rust library (DDC/CI + config)
├── apps/
│   ├── macos/        # macOS Swift menu bar app
│   └── linux/        # Linux Rust tray app (ksni)
├── Cargo.toml        # Core library manifest
└── Makefile          # Build commands
```

## How It Works

The app uses the DDC/CI (Display Data Channel Command Interface) protocol to communicate with monitors over the display cable. This is the same protocol monitors use for their on-screen display menus.

VCP (Virtual Control Panel) code `0x60` controls the input source selection.

## License

MIT

