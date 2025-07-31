#!/bin/bash -e

echo "🐳 Building OrbicWifiQR for ARM using Docker..."

# Build the Docker environment
docker build -t orbic-wifi-qr-devenv -f devenv.dockerfile .

echo "✅ Docker image built successfully!"

# Build the orbic-wifi-qr binary for ARM
echo "🔨 Building orbic-wifi-qr binary for ARM..."
docker run --user $UID:$GID -v ./:/workdir -w /workdir -it orbic-wifi-qr-devenv sh -c 'cargo build --release --target="armv7-unknown-linux-musleabihf"'

echo "✅ orbic-wifi-qr binary built successfully!"

# Create the orbic-wifi-qr directory on the device with proper permissions
echo "📁 Creating /data/orbic-wifi-qr directory..."
ADB_PATH="/Users/beisenmann/Library/Android/sdk/platform-tools/adb"
"$ADB_PATH" shell "/bin/rootshell -c 'mkdir -p /data/orbic-wifi-qr && chmod 755 /data/orbic-wifi-qr'"

# Push the binary to the permanent location using rootshell
echo "📱 Pushing orbic-wifi-qr to device..."
"$ADB_PATH" push target/armv7-unknown-linux-musleabihf/release/orbic-wifi-qr /tmp/orbic-wifi-qr_temp
"$ADB_PATH" shell "/bin/rootshell -c 'cp /tmp/orbic-wifi-qr_temp /data/orbic-wifi-qr/orbic-wifi-qr && chmod +x /data/orbic-wifi-qr/orbic-wifi-qr && rm /tmp/orbic-wifi-qr_temp'"

echo "🎉 orbic-wifi-qr is ready! Run it with:"
echo "$ADB_PATH shell \"/bin/rootshell /data/orbic-wifi-qr/orbic-wifi-qr\""
