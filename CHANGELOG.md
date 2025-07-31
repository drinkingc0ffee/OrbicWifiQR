# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Centered QR code display with optimal sizing
- Static linking with musl target for better compatibility
- Comprehensive documentation and contributing guidelines
- Service management scripts for easy deployment
- Auto-start configuration for boot-time launch

### Changed
- QR code size optimized to 120x120 pixels with 4px margins
- Improved error handling and logging
- Enhanced button timing (1.5-6 seconds for QR display)
- Updated build system for ARMv7 musl target

### Fixed
- App startup issues with dynamic linking
- QR code positioning and centering
- Service installation and management
- Button event handling reliability

## [0.1.0] - 2024-07-31

### Added
- Initial release of OrbicWifiQR
- WiFi QR code generation from device configuration
- Framebuffer display interface for 128x128 screens
- Button input handling for menu button
- Service management with auto-start capability
- Cross-compilation support for ARMv7 devices
- Docker-based build environment

### Features
- Automatic WiFi credential reading from XML configuration
- QR code generation following ZXing WiFi specification
- Long-press button detection (1.5-6 seconds)
- Triple-press exit functionality
- Background service operation
- Root permission handling for device access

### Technical Details
- Rust-based implementation
- ARMv7 target support (gnueabihf and musleabihf)
- Direct framebuffer access via `/dev/fb0`
- Input event processing via `/dev/input/event1`
- Systemd-style init script integration

---

## Version History

### v0.1.0 (Initial Release)
- Basic WiFi QR code functionality
- Button-controlled display
- Service management
- ARM cross-compilation

---

## Release Notes

### Known Issues
- Requires root access for device control
- Specific to Orbic devices with 128x128 displays
- WiFi configuration must be in specific XML format

### Compatibility
- **Architecture**: ARMv7 (armv7l)
- **Display**: 128x128 framebuffer
- **Input**: Menu button via event1
- **System**: Android/Linux with root access

### Dependencies
- **Rust**: 1.86+
- **Docker**: For cross-compilation
- **ADB**: For device deployment
- **Target Device**: Orbic hotspot with 128x128 display 