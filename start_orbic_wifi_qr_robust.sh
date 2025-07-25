#!/bin/sh
# OrbicWifiQR Startup Script - Robust Version

# Wait for system to be ready
sleep 20

# Change to app directory
cd /data/orbic-wifi-qr

# Kill any existing instances
pkill -f orbic-wifi-qr 2>/dev/null
sleep 2

# Start the app with nohup to prevent it from being killed when parent exits
# Use setsid to create a new session and detach from controlling terminal
nohup setsid /bin/rootshell ./orbic-wifi-qr > orbic-wifi-qr.log 2>&1 < /dev/null &

# Save PID
echo $! > orbic-wifi-qr.pid

# Wait a moment and verify it's running
sleep 3
if kill -0 $(cat orbic-wifi-qr.pid) 2>/dev/null; then
    echo "OrbicWifiQR started successfully with PID $(cat orbic-wifi-qr.pid)" >> orbic-wifi-qr.log
else
    echo "Failed to start OrbicWifiQR" >> orbic-wifi-qr.log
fi
