#!/bin/bash
# Quick installation on Linux

# Build and install the binary
cargo install --git https://github.com/GnRlLeclerc/firefox-native-base16

# Create the launcher script
cat >~/.local/bin/firefox-native-base16-launcher <<'EOF'
#!/bin/bash

trap 'kill -SIGTERM $native_pid' SIGTERM
~/.cargo/bin/firefox-native-base16 &
native_pid=$!
wait $native_pid
EOF

# Make the launcher script executable
chmod +x ~/.local/bin/firefox-native-base16-launcher

# Create the firefox application manifest
mkdir ~/.mozilla/native-messaging-hosts
jq ".path = \"$HOME/.local/bin/firefox-native-base16-launcher\"" manifest.json >~/.mozilla/native-messaging-hosts/firefox_native_base16.json

echo "Installation complete!"
