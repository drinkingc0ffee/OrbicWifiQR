#!/bin/bash

echo "🔧 OrbicWifiQR Service Manager"
echo "============================="

case "$1" in
    start)
        echo "📱 Starting OrbicWifiQR service..."
        adb shell "/bin/rootshell -c '
            cd /data/orbic-wifi-qr
            pkill -f orbic-wifi-qr 2>/dev/null
            sleep 2
            nohup /bin/rootshell ./orbic-wifi-qr > orbic-wifi-qr.log 2>&1 < /dev/null &
            echo \$! > orbic-wifi-qr.pid
            sleep 3
            if kill -0 \$(cat orbic-wifi-qr.pid) 2>/dev/null; then
                echo \"Service started with PID \$(cat orbic-wifi-qr.pid)\"
            else
                echo \"Failed to start service\"
                exit 1
            fi
        '"
        ;;
    stop)
        echo "🛑 Stopping OrbicWifiQR service..."
        adb shell "/bin/rootshell -c '
            cd /data/orbic-wifi-qr
            if [ -f orbic-wifi-qr.pid ]; then
                kill \$(cat orbic-wifi-qr.pid) 2>/dev/null
                rm -f orbic-wifi-qr.pid
            fi
            pkill -f orbic-wifi-qr 2>/dev/null
            echo \"Service stopped\"
        '"
        ;;
    status)
        echo "📊 Checking OrbicWifiQR service status..."
        adb shell "/bin/rootshell -c '
            if [ -f /data/orbic-wifi-qr/orbic-wifi-qr.pid ] && kill -0 \$(cat /data/orbic-wifi-qr/orbic-wifi-qr.pid) 2>/dev/null; then
                echo \"Service is running (PID: \$(cat /data/orbic-wifi-qr/orbic-wifi-qr.pid))\"
            else
                echo \"Service is not running\"
            fi
        '"
        adb shell "ps aux | grep orbic-wifi-qr | grep -v grep || echo 'No orbic-wifi-qr processes found'"
        ;;
    logs)
        echo "📝 OrbicWifiQR logs:"
        adb shell "cat /data/orbic-wifi-qr/orbic-wifi-qr.log 2>/dev/null || echo 'No log file found'"
        ;;
    test)
        echo "🧪 Testing OrbicWifiQR binary..."
        adb shell "/bin/rootshell -c 'cd /data/orbic-wifi-qr && timeout 5s ./orbic-wifi-qr || echo \"Test completed\"'"
        ;;
    *)
        echo "Usage: $0 {start|stop|status|logs|test}"
        echo ""
        echo "Commands:"
        echo "  start  - Start the OrbicWifiQR service"
        echo "  stop   - Stop the OrbicWifiQR service"  
        echo "  status - Check service status"
        echo "  logs   - Show service logs"
        echo "  test   - Test the binary briefly"
        exit 1
        ;;
esac
