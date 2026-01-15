{
  description = "macOS Input Source Manager CLI";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      perSystem =
        { pkgs, system, ... }:
        let
          pkgs' = pkgs.extend (
            final: prev: {
              swift_6 = import ./nix/swift.nix { pkgs = final; };
              swiftlint = import ./nix/swiftlint.nix { pkgs = prev; };
              swiftformat = import ./nix/swiftformat.nix { pkgs = prev; };
            }
          );
        in
        {
          formatter = pkgs'.treefmt;

          # Package build for nix build
          packages.default = import ./nix/darwin-ism.nix { pkgs = pkgs'; };

          # Development shell
          devShells.default = pkgs'.mkShell {
            buildInputs = [ pkgs'.apple-sdk_15 ];

            packages = with pkgs'; [
              git
              lefthook
              nil
              nixd
              nixfmt
              swift_6
              swiftformat
              swiftlint
              treefmt
            ];

            shellHook = ''
              lefthook install

              # Only exec into user shell for interactive sessions
              # Skip for non-interactive commands (like VSCode env detection)
              if [ -t 0 ] && [ -z "$__NIX_SHELL_EXEC" ]; then
                export __NIX_SHELL_EXEC=1

                # Detect user's login shell (works on both macOS and Linux)
                if command -v dscl >/dev/null 2>&1; then
                  # macOS
                  USER_SHELL=$(dscl . -read ~/ UserShell | sed 's/UserShell: //')
                elif command -v getent >/dev/null 2>&1; then
                  # Linux
                  USER_SHELL=$(getent passwd $USER | cut -d: -f7)
                else
                  # Fallback: read /etc/passwd directly
                  USER_SHELL=$(grep "^$USER:" /etc/passwd | cut -d: -f7)
                fi

                exec ''${USER_SHELL:-$SHELL}
              fi
            '';

            # Required for SwiftLint to find SourceKit framework
            DYLD_FRAMEWORK_PATH = "${pkgs'.swift_6}/lib";
            # Backup for DYLD_FRAMEWORK_PATH (preserved after exec to user shell)
            __NIX_SWIFT_LIB = "${pkgs'.swift_6}/lib";
          };
        };
    };
}
