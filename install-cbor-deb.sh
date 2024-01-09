#!/bin/sh
# Script to install the latest version of CBOR-CLI

# Exit on error
set -e
# Echo commands
set -x

# Add the public key
echo "Adding the public key..."
curl -sSL https://github.com/TakenPilot/cbor-cli/raw/main/public.gpg -o /usr/share/keyrings/public.gpg

# GitHub repository
repo="TakenPilot/cbor-cli"

# Function to detect architecture
detect_architecture() {
    local arch
    arch=$(dpkg --print-architecture)
    case "$arch" in
        amd64) echo "x86_64-unknown-linux-gnu" ;;
        i386)  echo "i686-unknown-linux-gnu" ;;
        arm64) echo "aarch64-unknown-linux-gnu" ;;
        armhf) echo "arm-unknown-linux-gnueabihf" ;;
        *)     echo "unsupported"; exit 1 ;;
    esac
}

# Function to map dpkg architecture to .deb file naming
map_arch_to_deb_name() {
    case "$1" in
        amd64) echo "amd64" ;;
        i386)  echo "i386" ;;
        arm64) echo "arm64" ;;
        armhf) echo "armhf" ;;
        *)     echo "unsupported"; exit 1 ;;
    esac
}

# Detect system architecture
system_arch=$(dpkg --print-architecture)
deb_arch=$(map_arch_to_deb_name $system_arch)

if [ "$deb_arch" = "unsupported" ]; then
    echo "Your architecture is not supported."
    exit 1
fi

# Use GitHub API to get the latest release tag
latest_release_tag=$(curl -sSL "https://api.github.com/repos/$repo/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

# Construct the package URL
package_url="https://github.com/$repo/releases/download/$latest_release_tag/cbor-cli_${latest_release_tag}_${deb_arch}.deb"

# Download and install the package
echo "Downloading and installing the CBOR-CLI package for $arch..."
curl -sSL -o cbor-cli.deb "$package_url"
dpkg -i cbor-cli.deb

echo "Installation complete!"
