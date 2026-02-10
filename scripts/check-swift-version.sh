#!/bin/sh
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

# Extract nixpkgs revision from flake.lock (POSIX-compatible JSON parsing)
NIXPKGS_REV=$(grep -A 10 '"nixpkgs":' "$ROOT_DIR/flake.lock" | grep '"rev":' | head -1 | sed 's/.*"rev": *"\([^"]*\)".*/\1/')

# Get Swift version from nixpkgs sources.nix (source of truth)
SOURCES_URL="https://raw.githubusercontent.com/NixOS/nixpkgs/$NIXPKGS_REV/pkgs/development/compilers/swift/sources.nix"
NIX_VERSION=$(curl -fsSL "$SOURCES_URL" | grep 'version = ' | head -1 | awk -F'"' '{print $2}')

# Extract version from mise.toml
MISE_VERSION=$(grep 'swift = ' "$ROOT_DIR/mise.toml" | awk -F'"' '{print $2}')

# Extract version from .swift-version
SWIFT_VERSION=$(tr -d '[:space:]' < "$ROOT_DIR/.swift-version")

ERRORS=0

if [ "$NIX_VERSION" != "$MISE_VERSION" ]; then
  echo "ERROR: Swift version mismatch between nixpkgs ($NIX_VERSION) and mise.toml ($MISE_VERSION)"
  ERRORS=$((ERRORS + 1))
fi

if [ "$NIX_VERSION" != "$SWIFT_VERSION" ]; then
  echo "ERROR: Swift version mismatch between nixpkgs ($NIX_VERSION) and .swift-version ($SWIFT_VERSION)"
  ERRORS=$((ERRORS + 1))
fi

if [ $ERRORS -gt 0 ]; then
  echo ""
  echo "nixpkgs (via flake.lock) is the source of truth. Please update other files to match."
  exit 1
fi

echo "Swift version check passed: $NIX_VERSION"
