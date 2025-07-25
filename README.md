# OrbicWifiQR

WiFi QR Code Display App for Orbic devices with enhanced button timing and exit functionality.

## Features

- **WiFi QR Code Generation**: Dynamically reads WiFi configuration and generates QR codes for easy device connection
- **Enhanced Button Timing**: Press and hold the menu button for **1.5-6 seconds** to display QR code (extended from 1.5-3 seconds)
- **Triple-Press Exit**: Quickly press the menu button **3 times within 2 seconds** to exit the app cleanly
- **Cross-Platform Build**: Docker-based cross-compilation for ARM devices
- **Auto-Start Support**: Configurable startup scripts for automatic launch on boot
- **Root Permissions**: Runs with full system access for framebuffer and input device control

## Quick Start

### Prerequisites
- Docker
- ADB (Android Debug Bridge)
- Orbic device with root access via `/bin/rootshell`

### Build and Deploy
```bash
# Build the app
./build.sh

# Install with auto-start
./install.sh
```

### Manual Usage
```bash
# Start the service
./orbic_service.sh start

# Check status
./orbic_service.sh status

# View logs
./orbic_service.sh logs
```

## Button Controls

| Action | Button Press | Result |
|--------|-------------|--------|
| **Display QR Code** | Hold 1.5-6 seconds | Shows WiFi QR for 30 seconds |
| **Exit App** | Triple-press within 2s | App exits cleanly |
| **Ignored** | <1.5s or >6s press | No action taken |

## Configuration

The app automatically reads WiFi credentials from:
- `/usrdata/data/usr/wlan/wlan_conf_6174.xml` (primary)
- Common system configuration files (fallback)
- Environment variables (fallback)

## Architecture

- **Target**: ARMv7 devices (Orbic hotspots)
- **Display**: 128x128 framebuffer (`/dev/fb0`)
- **Input**: Menu button (`/dev/input/event1`)
- **Permissions**: Root access required for device access

## Development

### Project Structure
```
OrbicWifiQR/
├── src/main.rs              # Main application code
├── build.sh                 # Cross-compilation script
├── install.sh               # Auto-start installation
├── orbic_service.sh         # Service management
└── devenv.dockerfile        # Docker build environment
```

### Building Locally
```bash
# Build for ARM
cargo build --release --target=armv7-unknown-linux-gnueabihf

# Deploy to device
adb push target/armv7-unknown-linux-gnueabihf/release/orbic-wifi-qr /data/orbic-wifi-qr/
```

## Troubleshooting

### App Not Starting on Boot
1. Check if startup script exists: `/data/orbic-wifi-qr/start_orbic_wifi_qr.sh`
2. Verify rcS entry: `/etc/init.d/rcS` should contain the startup line
3. Check logs: `cat /data/orbic-wifi-qr/orbic-wifi-qr.log`

### Button Not Responding
1. Verify input device: `ls -l /dev/input/event1`
2. Check permissions: App must run as root
3. Test manually: `/bin/rootshell /data/orbic-wifi-qr/orbic-wifi-qr`

## License

MIT License - see LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## Support

For issues and questions, please open an issue on GitHub.
