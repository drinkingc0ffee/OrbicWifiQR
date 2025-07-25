#!/bin/bash

echo "🔧 Installing OrbicWifiQR with Auto-Start"
echo "========================================="

# Check if adb is available
if ! command -v adb &> /dev/null; then
    echo "❌ Error: adb not found. Please install Android Debug Bridge (adb)"
    exit 1
fi

# Check if device is connected
echo "📱 Checking device connection..."
if ! adb devices | grep -q "device$"; then
    echo "❌ Error: No device connected or device not authorized"
    echo "   Please connect your device and ensure USB debugging is enabled"
    exit 1
fi

echo "✅ Device connected"

# Build the app first
echo "🔨 Building OrbicWifiQR..."
if ! ./build.sh; then
    echo "❌ Error: Failed to build OrbicWifiQR"
    exit 1
fi

echo "✅ Build successful"

# Create startup script with proper PATH
echo "🔧 Creating startup script on device..."
adb shell "/bin/rootshell -c 'cat > /data/orbic-wifi-qr/start_orbic_wifi_qr.sh << \"EOF\"
#!/bin/sh
# OrbicWifiQR Startup Script

# Set PATH for embedded systems including /data/orbic-wifi-qr
export PATH=/bin:/sbin:/usr/bin:/usr/sbin:/data/orbic-wifi-qr

# Wait for system to be fully ready
sleep 20

# Ensure we are in the right directory
cd /data/orbic-wifi-qr

# Check if binary exists
if [ ! -f ./orbic-wifi-qr ]; then
    echo \"ERROR: orbic-wifi-qr binary not found\"
    exit 1
fi

# Make sure it is executable
chmod +x ./orbic-wifi-qr

# Start the OrbicWifiQR app with logging
echo \"Starting OrbicWifiQR App...\"
./orbic-wifi-qr > /data/orbic-wifi-qr/orbic-wifi-qr.log 2>&1 &

# Save PID for management
echo \$! > /data/orbic-wifi-qr/orbic-wifi-qr.pid
echo \"OrbicWifiQR app started with PID: \$!\"
EOF'"

# Make startup script executable
echo "🔧 Making startup script executable..."
adb shell "/bin/rootshell -c 'chmod +x /data/orbic-wifi-qr/start_orbic_wifi_qr.sh'"

# Create init.d script for auto-start
echo "📝 Creating init.d script for auto-start..."
adb shell "/bin/rootshell -c 'cat > /etc/init.d/orbic-wifi-qr << \"EOF\"
#!/bin/sh
### BEGIN INIT INFO
# Provides:          orbic-wifi-qr
# Required-Start:    \$network \$remote_fs
# Required-Stop:     \$network \$remote_fs
# Default-Start:     2 3 4 5
# Default-Stop:      0 1 6
# Short-Description: OrbicWifiQR Display Service
# Description:       Starts the OrbicWifiQR app on boot
### END INIT INFO

case \"\$1\" in
  start)
    echo \"Starting OrbicWifiQR Display Service...\"
    /data/orbic-wifi-qr/start_orbic_wifi_qr.sh &
    ;;
  stop)
    echo \"Stopping OrbicWifiQR Display Service...\"
    if [ -f /data/orbic-wifi-qr/orbic-wifi-qr.pid ]; then
        kill \$(cat /data/orbic-wifi-qr/orbic-wifi-qr.pid) 2>/dev/null
        rm -f /data/orbic-wifi-qr/orbic-wifi-qr.pid
    fi
    pkill -f orbic-wifi-qr
    ;;
  restart)
    \$0 stop
    sleep 2
    \$0 start
    ;;
  status)
    if [ -f /data/orbic-wifi-qr/orbic-wifi-qr.pid ] && kill -0 \$(cat /data/orbic-wifi-qr/orbic-wifi-qr.pid) 2>/dev/null; then
      echo \"OrbicWifiQR service is running (PID: \$(cat /data/orbic-wifi-qr/orbic-wifi-qr.pid))\"
    elif pgrep -f orbic-wifi-qr > /dev/null; then
      echo \"OrbicWifiQR service is running\"
    else
      echo \"OrbicWifiQR service is not running\"
    fi
    ;;
  *)
    echo \"Usage: \$0 {start|stop|restart|status}\"
    exit 1
    ;;
esac

exit 0
EOF'"

# Make init.d script executable
echo "🔧 Making init.d script executable..."
adb shell "/bin/rootshell -c 'chmod +x /etc/init.d/orbic-wifi-qr'"

# Add to multiple startup locations for maximum compatibility
echo "📝 Adding to startup locations..."

# Method 1: /etc/init.d/rcS (most common for embedded)
adb shell "/bin/rootshell -c 'if ! grep -q \"orbic-wifi-qr\" /etc/init.d/rcS; then
    echo \"# Start OrbicWifiQR Display App\" >> /etc/init.d/rcS
    echo \"/data/orbic-wifi-qr/start_orbic_wifi_qr.sh &\" >> /etc/init.d/rcS
fi'"

# Method 2: /etc/rc.local
adb shell "/bin/rootshell -c 'if [ -f /etc/rc.local ] && ! grep -q \"orbic-wifi-qr\" /etc/rc.local; then
    echo \"# Start OrbicWifiQR Display App\" >> /etc/rc.local
    echo \"/data/orbic-wifi-qr/start_orbic_wifi_qr.sh &\" >> /etc/rc.local
fi'"

# Start the service now
echo "▶️  Starting OrbicWifiQR service..."
adb shell "/bin/rootshell -c '/etc/init.d/orbic-wifi-qr start'"

# Wait a moment and check if it started
echo "📊 Checking service status..."
sleep 5
if adb shell "pgrep -f orbic-wifi-qr" | grep -q .; then
    echo "✅ OrbicWifiQR service started successfully"
else
    echo "⚠️  Service may not have started, checking logs..."
    adb shell "cat /data/orbic-wifi-qr/orbic-wifi-qr.log 2>/dev/null || echo 'No log file found'"
fi

echo ""
echo "✅ OrbicWifiQR installed and configured for auto-start!"
echo ""
echo "📋 Service Information:"
echo "   - Service Name: orbic-wifi-qr"
echo "   - Binary Location: /data/orbic-wifi-qr/orbic-wifi-qr"
echo "   - Startup Script: /data/orbic-wifi-qr/start_orbic_wifi_qr.sh"
echo "   - Init Script: /etc/init.d/orbic-wifi-qr"
echo "   - Auto-start: Enabled"
echo ""
echo "🔧 Service Control Commands:"
echo "   Start:   adb shell \"/bin/rootshell /etc/init.d/orbic-wifi-qr start\""
echo "   Stop:    adb shell \"/bin/rootshell /etc/init.d/orbic-wifi-qr stop\""
echo "   Status:  adb shell \"/bin/rootshell /etc/init.d/orbic-wifi-qr status\""
echo "   Restart: adb shell \"/bin/rootshell /etc/init.d/orbic-wifi-qr restart\""
echo ""
echo "🎯 Usage:"
echo "   Long press the menu button (event1) for 1.5-3 seconds to display WiFi QR code"
echo "   The app runs continuously in the background and auto-starts on boot"
echo ""
echo "📝 Logs: adb shell \"cat /data/orbic-wifi-qr/orbic-wifi-qr.log\""
echo "🔄 To test auto-start: adb shell \"reboot\""
