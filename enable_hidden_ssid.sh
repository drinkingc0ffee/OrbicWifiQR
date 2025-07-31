#!/bin/bash

# Enable Hidden SSID Script for Orbic WiFi QR System
# This script hides the WiFi SSID from scanners and web interface

echo "=== Enabling Hidden SSID Configuration ==="

# Check if device is connected
if ! adb devices | grep -q "device$"; then
    echo "❌ Error: No ADB device connected"
    exit 1
fi

echo "📱 Device connected, applying hidden SSID configuration..."

# Enable hidden SSID settings
echo "🔒 Hiding SSID from web interface..."
adb shell "/bin/rootshell -c 'sed -i \"s/<ssid_visible>1<\/ssid_visible>/<ssid_visible>0<\/ssid_visible>/\" /usrdata/data/usr/wlan/wlan_conf_6174.xml'"

echo "🔒 Hiding SSID from WiFi scanners..."
adb shell "/bin/rootshell -c 'sed -i \"s/<broadcast_ssid>1<\/broadcast_ssid>/<broadcast_ssid>0<\/broadcast_ssid>/\" /usrdata/data/usr/wlan/wlan_conf_6174.xml'"

# Verify changes
echo "✅ Verifying configuration changes..."
echo ""
echo "Current SSID visibility settings:"
adb shell "grep -E 'ssid_visible|broadcast_ssid' /usrdata/data/usr/wlan/wlan_conf_6174.xml"

echo ""
echo "🎯 Hidden SSID Configuration Applied Successfully!"
echo "   - SSID is now hidden from WiFi scanners"
echo "   - Network access requires QR code"
echo "   - Network: Hidden"
echo "   - Password: Hidden"
echo ""
echo "💡 Test by scanning the QR code - the network should not appear in WiFi settings"
