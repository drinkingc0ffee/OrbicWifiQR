# OrbicWifiQR

A Rust application that displays WiFi QR codes on Orbic devices with 128x128 displays. The app runs continuously in the background and displays a scannable QR code when the menu button is held down.

## Features

- **WiFi QR Code Generation**: Automatically reads WiFi credentials from the device and generates QR codes
- **Centered Display**: QR codes are perfectly centered and sized for optimal scanning
- **Button Control**: Long press menu button (1.5-6 seconds) to display QR code
- **Auto-start**: Configured to start automatically on device boot
- **Background Service**: Runs continuously as a system service

## Hardware Requirements

- Orbic device with 128x128 display
- ARMv7 architecture (`armv7l`)
- Android/Linux-based system with framebuffer support

## Prerequisites

- Docker (for building)
- ADB (Android Debug Bridge)
- Device connected via USB with USB debugging enabled

## Installation

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/OrbicWifiQR.git
cd OrbicWifiQR
```

### 2. Install on Device

```bash
# Make scripts executable
chmod +x install.sh build.sh

# Install the app (this will build and install automatically)
./install.sh
```

The installation script will:
- Build the ARM binary using Docker
- Install the app to `/data/orbic-wifi-qr/`
- Configure auto-start service
- Start the app immediately

### 3. Verify Installation

```bash
# Check if the service is running
adb shell "ps aux | grep orbic-wifi-qr | grep -v grep"

# Check service status
adb shell "/bin/rootshell /etc/init.d/orbic-wifi-qr status"
```

## Usage

### Display WiFi QR Code

1. **Long press the menu button** (event1) for 1.5-6 seconds
2. The QR code will appear on the display for 30 seconds
3. Scan the QR code with any mobile device to connect to WiFi

### Exit the App

**Triple-press the menu button** within 2 seconds to exit the app.

## Service Management

### Start Service
```bash
adb shell "/bin/rootshell /etc/init.d/orbic-wifi-qr start"
```

### Stop Service
```bash
adb shell "/bin/rootshell /etc/init.d/orbic-wifi-qr stop"
```

### Restart Service
```bash
adb shell "/bin/rootshell /etc/init.d/orbic-wifi-qr restart"
```

### Check Status
```bash
adb shell "/bin/rootshell /etc/init.d/orbic-wifi-qr status"
```

### View Logs
```bash
adb shell "cat /data/orbic-wifi-qr/orbic-wifi-qr.log"
```

## Development

### Building Locally

```bash
# Build for ARM using Docker
./build.sh

# Or build manually
docker build -t orbic-wifi-qr-devenv -f devenv.dockerfile .
docker run --user $UID:$GID -v ./:/workdir -w /workdir -it orbic-wifi-qr-devenv sh -c 'cargo build --release --target="armv7-unknown-linux-musleabihf"'
```

### Project Structure

```
OrbicWifiQR/
├── src/
│   └── main.rs              # Main application code
├── build.sh                 # Build script for ARM target
├── install.sh               # Installation and service setup
├── devenv.dockerfile        # Docker environment for building
├── Cargo.toml              # Rust dependencies
└── README.md               # This file
```

### Key Components

- **Framebuffer Interface**: Direct display access via `/dev/fb0`
- **Input Event Handling**: Button press detection via `/dev/input/event1`
- **WiFi Configuration**: Reads from `/usrdata/data/usr/wlan/wlan_conf_6174.xml`
- **QR Code Generation**: Uses `qrcode` crate with WiFi format specification
- **Service Management**: Systemd-style init scripts for auto-start

## Configuration

### WiFi Credentials

The app automatically reads WiFi credentials from:
```
/usrdata/data/usr/wlan/wlan_conf_6174.xml
```

Supported encryption types:
- WPA2 (default)
- WPA
- WEP
- Open networks

### Display Settings

- **Resolution**: 128x128 pixels
- **Color Format**: RGB565
- **QR Code Size**: 120x120 pixels (centered with 4px margins)
- **Error Correction**: Low (L) for easier scanning

## Troubleshooting

### App Not Starting

1. Check if the binary exists:
   ```bash
   adb shell "ls -la /data/orbic-wifi-qr/orbic-wifi-qr"
   ```

2. Check logs:
   ```bash
   adb shell "cat /data/orbic-wifi-qr/orbic-wifi-qr.log"
   ```

3. Verify device architecture:
   ```bash
   adb shell "uname -m"
   ```

### QR Code Not Displaying

1. Check if WiFi credentials exist:
   ```bash
   adb shell "cat /usrdata/data/usr/wlan/wlan_conf_6174.xml"
   ```

2. Verify button events:
   ```bash
   adb shell "cat /dev/input/event1"
   ```

### Service Issues

1. Restart the service:
   ```bash
   adb shell "/bin/rootshell /etc/init.d/orbic-wifi-qr restart"
   ```

2. Check service status:
   ```bash
   adb shell "/bin/rootshell /etc/init.d/orbic-wifi-qr status"
   ```

## Technical Details

### Dependencies

- **Rust**: 1.86+
- **qrcode**: QR code generation
- **image**: Image processing
- **Static Linking**: Musl target for portability

### Build Targets

- **Primary**: `armv7-unknown-linux-musleabihf` (static linking)
- **Alternative**: `armv7-unknown-linux-gnueabihf` (dynamic linking)

### System Integration

- **Init Script**: `/etc/init.d/orbic-wifi-qr`
- **Startup Script**: `/data/orbic-wifi-qr/start_orbic_wifi_qr.sh`
- **Binary Location**: `/data/orbic-wifi-qr/orbic-wifi-qr`
- **Logs**: `/data/orbic-wifi-qr/orbic-wifi-qr.log`

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- QR code generation using the `qrcode` crate
- Image processing with the `image` crate
- WiFi QR code format specification from ZXing

## Support

For issues and questions:
- Create an issue on GitHub
- Check the troubleshooting section above
- Review the logs for error messages

---

**Note**: This app is specifically designed for Orbic devices with 128x128 displays. It may not work on other hardware configurations.
