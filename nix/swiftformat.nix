{ pkgs }:
pkgs.swiftformat.overrideAttrs (old: rec {
  version = "0.58.7";
  src = pkgs.fetchFromGitHub {
    owner = "nicklockwood";
    repo = "SwiftFormat";
    rev = version;
    hash = "sha256-CL+3z7wCIIJGWz7FPTFY9A+vBqyS6uGb6hgGRkJobUk=";
  };
})
