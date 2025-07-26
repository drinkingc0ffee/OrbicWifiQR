#!/bin/sh
export PATH=/bin:/sbin:/usr/bin:/usr/sbin:/data/orbic-wifi-qr
sleep 20
cd /data/orbic-wifi-qr
# Use rootshell with full path to run the binary
/bin/rootshell -c '/data/orbic-wifi-qr/orbic-wifi-qr' > orbic-wifi-qr.log 2>&1 &
echo $! > orbic-wifi-qr.pid
