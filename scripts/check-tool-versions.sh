#!/bin/sh
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

ERRORS=0

check_version() {
  local tool_name="$1"
  local nix_file="$2"
  local mise_pattern="$3"

  # Extract version from nix file (source of truth)
  NIX_VERSION=$(grep 'version = ' "$ROOT_DIR/nix/$nix_file" | head -1 | cut -d'=' -f2 | tr -d ' ";')

  # Extract version from mise.toml
  MISE_VERSION=$(grep "$mise_pattern" "$ROOT_DIR/mise.toml" | cut -d'=' -f2 | tr -d ' "')

  if [ "$NIX_VERSION" != "$MISE_VERSION" ]; then
    echo "ERROR: $tool_name version mismatch between nix/$nix_file ($NIX_VERSION) and mise.toml ($MISE_VERSION)"
    ERRORS=$((ERRORS + 1))
  else
    echo "$tool_name version check passed: $NIX_VERSION"
  fi
}

# Check swiftlint
check_version "swiftlint" "swiftlint.nix" "swiftlint"

# Check swiftformat
check_version "swiftformat" "swiftformat.nix" "swiftformat"

if [ $ERRORS -gt 0 ]; then
  echo ""
  echo "nix/*.nix files are the source of truth. Please update mise.toml to match."
  exit 1
fi
