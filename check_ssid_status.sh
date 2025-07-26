#!/bin/bash

# Check SSID Status Script for Orbic WiFi QR System
# This script shows the current SSID visibility status

echo "=== Current SSID Visibility Status ==="

# Check if device is connected
if ! adb devices | grep -q "device$"; then
    echo "❌ Error: No ADB device connected"
    exit 1
fi

echo "�� Device connected, checking current configuration..."
echo ""

# Get current settings
echo "Current SSID visibility settings:"
adb shell "grep -E 'ssid_visible|broadcast_ssid' /usrdata/data/usr/wlan/wlan_conf_6174.xml"

echo ""
echo "📊 Status Summary:"

# Check ssid_visible setting
if adb shell "grep -q '<ssid_visible>0</ssid_visible>' /usrdata/data/usr/wlan/wlan_conf_6174.xml"; then
    echo "�� SSID Web Interface: HIDDEN"
else
    echo "👁️ SSID Web Interface: VISIBLE"
fi

# Check broadcast_ssid setting for active network
if adb shell "grep -A 5 '<Basic_0>' /usrdata/data/usr/wlan/wlan_conf_6174.xml | grep -q '<broadcast_ssid>0</broadcast_ssid>'"; then
    echo "🔒 SSID WiFi Broadcast: HIDDEN"
else
    echo "👁️ SSID WiFi Broadcast: VISIBLE"
fi

echo ""
echo "💡 Use ./enable_hidden_ssid.sh to hide the SSID"
echo "💡 Use ./disable_hidden_ssid.sh to make the SSID visible"
