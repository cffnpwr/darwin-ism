{ pkgs }:
pkgs.swiftlint.overrideAttrs (old: rec {
  version = "0.63.0";
  src = pkgs.fetchurl {
    url = "https://github.com/realm/SwiftLint/releases/download/${version}/portable_swiftlint.zip";
    hash = "sha256-apzA4+CUZvzl6r1LkgzXFp6eVrYOEu4TQKeT6otgzrk=";
  };
})
