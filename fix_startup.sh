#!/bin/bash

echo "🔧 Fixing OrbicWifiQR Auto-Start Issues"
echo "======================================"

# Create an improved startup script with better error handling and logging
adb shell "/bin/rootshell -c 'cat > /data/orbic-wifi-qr/start_orbic_wifi_qr.sh << \"SCRIPT_EOF\"
#!/bin/sh
# OrbicWifiQR Startup Script - Enhanced Version

# Create log directory if it doesn't exist
mkdir -p /data/orbic-wifi-qr/logs

# Set PATH for embedded systems
export PATH=/bin:/sbin:/usr/bin:/usr/sbin:/data/orbic-wifi-qr

# Log startup attempt
echo \"\$(date): OrbicWifiQR startup script started\" >> /data/orbic-wifi-qr/logs/startup.log

# Wait for system to be fully ready
echo \"\$(date): Waiting 20 seconds for system readiness...\" >> /data/orbic-wifi-qr/logs/startup.log
sleep 20

# Ensure we are in the right directory
cd /data/orbic-wifi-qr
echo \"\$(date): Changed to directory: \$(pwd)\" >> /data/orbic-wifi-qr/logs/startup.log

# Check if binary exists
if [ ! -f ./orbic-wifi-qr ]; then
    echo \"\$(date): ERROR: orbic-wifi-qr binary not found\" >> /data/orbic-wifi-qr/logs/startup.log
    exit 1
fi

# Make sure it is executable
chmod +x ./orbic-wifi-qr
echo \"\$(date): Binary permissions set\" >> /data/orbic-wifi-qr/logs/startup.log

# Check if input device is accessible
if [ ! -c /dev/input/event1 ]; then
    echo \"\$(date): ERROR: /dev/input/event1 not accessible\" >> /data/orbic-wifi-qr/logs/startup.log
    exit 1
fi

# Kill any existing instances
pkill -f orbic-wifi-qr 2>/dev/null
sleep 2

# Start the OrbicWifiQR app with rootshell, proper environment, and comprehensive logging
echo \"\$(date): Starting OrbicWifiQR App with rootshell...\" >> /data/orbic-wifi-qr/logs/startup.log

# Use nohup to ensure the process doesn't get killed when the parent exits
# Redirect all output to logs and run in background
nohup /bin/rootshell -c \"cd /data/orbic-wifi-qr && ./orbic-wifi-qr\" > /data/orbic-wifi-qr/logs/orbic-wifi-qr.log 2>&1 &

# Get the actual PID
APP_PID=\$!
echo \$APP_PID > /data/orbic-wifi-qr/orbic-wifi-qr.pid
echo \"\$(date): OrbicWifiQR app started with PID: \$APP_PID\" >> /data/orbic-wifi-qr/logs/startup.log

# Wait a moment and verify it's still running
sleep 3
if kill -0 \$APP_PID 2>/dev/null; then
    echo \"\$(date): SUCCESS: Process \$APP_PID is running\" >> /data/orbic-wifi-qr/logs/startup.log
else
    echo \"\$(date): ERROR: Process \$APP_PID exited immediately\" >> /data/orbic-wifi-qr/logs/startup.log
    # Try to capture any error output
    if [ -f /data/orbic-wifi-qr/logs/orbic-wifi-qr.log ]; then
        echo \"\$(date): Last few lines of app log:\" >> /data/orbic-wifi-qr/logs/startup.log
        tail -5 /data/orbic-wifi-qr/logs/orbic-wifi-qr.log >> /data/orbic-wifi-qr/logs/startup.log
    fi
fi

echo \"\$(date): Startup script completed\" >> /data/orbic-wifi-qr/logs/startup.log
SCRIPT_EOF'"

# Make the startup script executable
adb shell "/bin/rootshell -c 'chmod +x /data/orbic-wifi-qr/start_orbic_wifi_qr.sh'"

echo "✅ Enhanced startup script created"

# Test the new startup script
echo "🧪 Testing the enhanced startup script..."
adb shell "/bin/rootshell /data/orbic-wifi-qr/start_orbic_wifi_qr.sh"

# Wait and check results
echo "⏳ Waiting for startup to complete..."
sleep 25

echo "📊 Checking results..."
adb shell "ls -la /data/orbic-wifi-qr/logs/ 2>/dev/null || echo 'No logs directory'"
adb shell "cat /data/orbic-wifi-qr/logs/startup.log 2>/dev/null || echo 'No startup log'"
adb shell "ps aux | grep orbic-wifi-qr | grep -v grep || echo 'No orbic-wifi-qr process running'"

echo "✅ Startup fix attempt completed!"
