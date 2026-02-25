#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

APP_NAME="Monitor Switch"
BUNDLE_NAME="MonitorSwitch.app"
BUILD_DIR=".build/release"

echo "Building Rust library..."
cd "$ROOT_DIR"
cargo build --release

echo "Copying C header..."
cp "$ROOT_DIR/include/monitor_core.h" "$SCRIPT_DIR/Sources/CMonitorCore/"

echo "Building Swift app..."
cd "$SCRIPT_DIR"
swift build -c release

echo "Creating app bundle..."
rm -rf "$BUNDLE_NAME"
mkdir -p "$BUNDLE_NAME/Contents/MacOS"
mkdir -p "$BUNDLE_NAME/Contents/Resources"

cp "$BUILD_DIR/MonitorSwitch" "$BUNDLE_NAME/Contents/MacOS/"
cp Info.plist "$BUNDLE_NAME/Contents/"

if [ -f "AppIcon.icns" ]; then
    cp AppIcon.icns "$BUNDLE_NAME/Contents/Resources/"
    echo "Added app icon"
fi

echo "App bundle created: $BUNDLE_NAME"
echo ""
echo "To install, run:"
echo "  cp -r \"$BUNDLE_NAME\" /Applications/"

