#!/bin/sh
set -e

# ==========================================
# CONFIGURATION
# ==========================================
OWNER="alexfigliolia"
REPO="repokit"
BIN_NAME="repokit"
# Leave empty to automatically fetch the latest release version
VERSION=""
# The global folder where the binary will live
INSTALL_DIR="/usr/local/bin"

# ==========================================
# SYSTEM DETECTION
# ==========================================
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$OS" in
  linux)
    PLATFORM="unknown-linux-gnu"
    FORMAT="tar.gz"
    ;;
  darwin)
    PLATFORM="apple-darwin"
    FORMAT="tar.gz"
    ;;
  msys*|mingw*|cygwin*)
    PLATFORM="pc-windows-msvc"
    FORMAT="zip"
    BIN_NAME="${BIN_NAME}.exe"
    # Windows doesn't use /usr/local/bin, so fallback to current directory
    INSTALL_DIR="."
    ;;
  *)
    echo "❌ Error: Unsupported operating system: $OS" >&2
    exit 1
    ;;
esac

case "$ARCH" in
  x86_64|amd64)
    TARGET_ARCH="x86_64"
    ;;
  arm64|aarch64)
    TARGET_ARCH="aarch64"
    ;;
  *)
    echo "❌ Error: Unsupported CPU architecture: $ARCH" >&2
    exit 1
    ;;
esac

TARGET_TRIPLE="${TARGET_ARCH}-${PLATFORM}"
ASSET_NAME="${BIN_NAME}-${TARGET_TRIPLE}.${FORMAT}"

# ==========================================
# FETCH VERSION & DOWNLOAD URL
# ==========================================
echo "🔍 Detecting system... Found ${OS} (${ARCH})"

if [ -z "$VERSION" ]; then
  echo "📡 Fetching latest release info from GitHub..."
  API_URL="https://github.com{OWNER}/${REPO}/releases/latest"
  DOWNLOAD_URL=$(curl -sSL "$API_URL" | grep "browser_download_url" | grep "$ASSET_NAME" | head -n 1 | cut -d '"' -f 4)
else
  echo "📡 Using specified version: ${VERSION}"
  DOWNLOAD_URL="https://github.com{OWNER}/${REPO}/releases/download/${VERSION}/${ASSET_NAME}"
fi

if [ -z "$DOWNLOAD_URL" ]; then
  echo "❌ Error: Could not find a release asset matching: ${ASSET_NAME}" >&2
  exit 1
fi

# ==========================================
# DOWNLOAD AND EXTRACT
# ==========================================
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

echo "📥 Downloading ${ASSET_NAME}..."
curl -sSL -O "$DOWNLOAD_URL"

echo "📦 Extracting binary..."
if [ "$FORMAT" = "zip" ]; then
  unzip -q "$ASSET_NAME"
else
  tar -xzf "$ASSET_NAME"
fi

# ==========================================
# INSTALLATION TO SYSTEM PATH
# ==========================================
echo "🚀 Installing ${BIN_NAME} to ${INSTALL_DIR}..."

if [ "$INSTALL_DIR" = "." ]; then
  # Windows fallback: move to the directory where the user ran the command
  mv "$BIN_NAME" "$OLDPWD/"
else
  # Linux/macOS path handling
  # Ensure the directory exists
  if [ ! -d "$INSTALL_DIR" ]; then
    echo "📂 Creating directory ${INSTALL_DIR}..."
    if [ -w "$(dirname "$INSTALL_DIR")" ]; then
      mkdir -p "$INSTALL_DIR"
    else
      sudo mkdir -p "$INSTALL_DIR"
    fi
  fi

  # Move binary with appropriate permissions
  if [ -w "$INSTALL_DIR" ]; then
    mv "$BIN_NAME" "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/$BIN_NAME"
  else
    echo "🔐 Administrator privileges needed to write to ${INSTALL_DIR}."
    sudo mv "$BIN_NAME" "$INSTALL_DIR/"
    sudo chmod +x "$INSTALL_DIR/$BIN_NAME"
  fi
fi

# Clean up temporary downloads
cd "$OLDPWD"
rm -rf "$TMP_DIR"

if [ "$INSTALL_DIR" = "." ]; then
  echo "✨ Success! ${BIN_NAME} has been saved to your current folder."
else
  echo "✨ Success! ${BIN_NAME} is fully installed. You can now run it from anywhere by typing: ${BIN_NAME}"
fi