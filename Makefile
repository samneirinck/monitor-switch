.PHONY: all macos linux clean

all: macos linux

# macOS app bundle
macos:
	cargo build --release
	cd apps/macos && swift build -c release
	cd apps/macos && ./build-app.sh

# Linux binary
linux:
	cd apps/linux && cargo build --release

# Install macOS app
install-macos: macos
	cp -r apps/macos/MonitorSwitch.app /Applications/

# Install Linux binary
install-linux: linux
	install -Dm755 apps/linux/target/release/monitor-switch ~/.local/bin/monitor-switch

# Development builds
dev-macos:
	cargo build
	cd apps/macos && swift build

dev-linux:
	cd apps/linux && cargo build

# Run development builds
run-macos: dev-macos
	apps/macos/.build/debug/MonitorSwitch

run-linux: dev-linux
	apps/linux/target/debug/monitor-switch

# Clean build artifacts
clean:
	cargo clean
	cd apps/linux && cargo clean
	cd apps/macos && swift package clean
	rm -rf apps/macos/MonitorSwitch.app
	rm -rf apps/macos/.build

