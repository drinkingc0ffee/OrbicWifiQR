#!/bin/sh
# Simple startup script for orbic-wifi-qr
export PATH=/bin:/sbin:/usr/bin:/usr/sbin:/data/orbic-wifi-qr

# Wait for system to be ready
sleep 30

# Start the app with rootshell
cd /data/orbic-wifi-qr
/bin/rootshell -c '/data/orbic-wifi-qr/orbic-wifi-qr' > orbic-wifi-qr.log 2>&1 &
echo $! > orbic-wifi-qr.pid

# Log startup
echo "$(date): OrbicWifiQR started with PID $(cat orbic-wifi-qr.pid 2>/dev/null || echo 'unknown')" >> /data/orbic-wifi-qr/startup.log
