#!/bin/bash

# Disable Hidden SSID Script for Orbic WiFi QR System
# This script makes the WiFi SSID visible again for testing

echo "=== Disabling Hidden SSID Configuration ==="

# Check if device is connected
if ! adb devices | grep -q "device$"; then
    echo "❌ Error: No ADB device connected"
    exit 1
fi

echo "📱 Device connected, restoring visible SSID configuration..."

# Disable hidden SSID settings
echo "👁️ Making SSID visible in web interface..."
adb shell "/bin/rootshell -c 'sed -i \"s/<ssid_visible>0<\/ssid_visible>/<ssid_visible>1<\/ssid_visible>/\" /usrdata/data/usr/wlan/wlan_conf_6174.xml'"

echo "👁️ Making SSID visible to WiFi scanners..."
adb shell "/bin/rootshell -c 'sed -i \"s/<broadcast_ssid>0<\/broadcast_ssid>/<broadcast_ssid>1<\/broadcast_ssid>/\" /usrdata/data/usr/wlan/wlan_conf_6174.xml'"

# Verify changes
echo "✅ Verifying configuration changes..."
echo ""
echo "Current SSID visibility settings:"
adb shell "grep -E 'ssid_visible|broadcast_ssid' /usrdata/data/usr/wlan/wlan_conf_6174.xml"

echo ""
echo "🎯 Visible SSID Configuration Applied Successfully!"
echo "   - SSID is now visible to WiFi scanners"
echo "   - Network appears in WiFi settings"
echo "   - Network: Visible"
echo "   - Password: Hidden"
echo ""
echo "💡 Test by checking WiFi settings - the network should now be visible"
