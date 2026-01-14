#!/bin/sh
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

# Extract version from nix/swift.nix (source of truth)
NIX_VERSION=$(grep 'version = ' "$ROOT_DIR/nix/swift.nix" | head -1 | awk -F'"' '{print $2}')

# Extract version from mise.toml
MISE_VERSION=$(grep 'swift = ' "$ROOT_DIR/mise.toml" | awk -F'"' '{print $2}')

# Extract version from .swift-version
SWIFT_VERSION=$(tr -d '[:space:]' < "$ROOT_DIR/.swift-version")

ERRORS=0

if [ "$NIX_VERSION" != "$MISE_VERSION" ]; then
  echo "ERROR: Swift version mismatch between nix/swift.nix ($NIX_VERSION) and mise.toml ($MISE_VERSION)"
  ERRORS=$((ERRORS + 1))
fi

if [ "$NIX_VERSION" != "$SWIFT_VERSION" ]; then
  echo "ERROR: Swift version mismatch between nix/swift.nix ($NIX_VERSION) and .swift-version ($SWIFT_VERSION)"
  ERRORS=$((ERRORS + 1))
fi

if [ $ERRORS -gt 0 ]; then
  echo ""
  echo "nix/swift.nix is the source of truth. Please update other files to match."
  exit 1
fi

echo "Swift version check passed: $NIX_VERSION"
