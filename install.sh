#!/bin/bash

# Code Chrono macOS Installer
# This script downloads the latest release from GitHub and installs it securely without macOS quarantine.

set -e

REPO="CapelaA10/code-chrono"
APP_NAME="Code Chrono"
APP_DIR="/Applications/$APP_NAME.app"

echo "🕒 Installing $APP_NAME for macOS..."

if [[ "$OSTYPE" != "darwin"* ]]; then
  echo "❌ This install script is only for macOS. Please see the README for Windows/Linux instructions."
  exit 1
fi

echo "🔍 Fetching latest release info..."
# Fetch the latest release payload and pull the browser_download_url for the .app.tar.gz
LATEST_RELEASE=$(curl -s "https://api.github.com/repos/$REPO/releases/latest")
DOWNLOAD_URL=$(echo "$LATEST_RELEASE" | grep -o '"browser_download_url": ".*\.app\.tar\.gz"' | cut -d '"' -f 4)

if [[ -z "$DOWNLOAD_URL" ]]; then
  echo "❌ Failed to find the latest macOS application bundle (.app.tar.gz)."
  echo "The release might still be building. Check github.com/$REPO/releases"
  exit 1
fi

# Download in a temporary folder
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

echo "⬇️  Downloading $APP_NAME..."
curl -sL "$DOWNLOAD_URL" -o "app.tar.gz"

echo "📦 Extracting application..."
tar -xzf app.tar.gz

# If old version exists, remove it
if [[ -d "$APP_DIR" ]]; then
  echo "🗑️  Removing older version..."
  rm -rf "$APP_DIR"
fi

echo "🚀 Moving to /Applications..."
# The tarball contains the .app folder itself (or inside a wrapper directory)
# Because Tauri creates `App Name.app` inside the tarball, we find it:
EXTRACTED_APP=$(find . -maxdepth 2 -name "*.app" | head -n 1)

if [[ -n "$EXTRACTED_APP" ]]; then
  cp -R "$EXTRACTED_APP" "/Applications/"
else
  echo "❌ Failed to find .app inside the downloaded archive."
  rm -rf "$TMP_DIR"
  exit 1
fi

# We manually remove the quarantine attribute just in case standard HTTP libraries added it back (rare for curl, but good measure)
xattr -cr "$APP_DIR" 2>/dev/null || true

# Cleanup
cd ~
rm -rf "$TMP_DIR"

echo "✅ $APP_NAME successfully installed bypassing Apple Gatekeeper!"
echo "Enjoy tracking! You can now open it from Launchpad or Spotlight."

