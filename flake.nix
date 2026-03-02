{
  description = "macOS Input Source Manager CLI";

  nixConfig = {
    extra-substituters = [ "https://nix-cache.cffnpwr.dev" ];
    extra-trusted-public-keys = [
      "cffnpwr-nixpkgs-extras:dmp2DUGwdqawLCPOsOcRxU/NpCO/qA1jha/8rmoSzvA="
    ];
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs-extras = {
      url = "github:cffnpwr/nixpkgs-extras";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{
      nixpkgs,
      flake-parts,
      nixpkgs-extras,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      perSystem =
        { pkgs, system, ... }:
        {
          _module.args.pkgs = import nixpkgs {
            inherit system;
            overlays = [
              nixpkgs-extras.overlays.default
              (final: prev: {
                swiftlint = import ./nix/swiftlint.nix { pkgs = prev; };
                swiftformat = import ./nix/swiftformat.nix { pkgs = prev; };
              })
            ];
          };
          formatter = pkgs.treefmt;

          # Package build for nix build
          packages = {
            default = import ./nix/darwin-ism/package.nix { inherit pkgs; };
          };

          # Development shell
          devShells = {
            default = pkgs.mkShell {
              packages = with pkgs; [
                apple-sdk_14
                git
                lefthook
                nil
                nixd
                nixfmt
                swift
                swiftformat
                swiftlint
                swiftPackages.swiftpm
                swiftpm2nix
                treefmt
                xcbuild
                yamlfmt
              ];

              shellHook = ''
                lefthook install

                # Required for SwiftLint to find SourceKit framework
                export DYLD_FRAMEWORK_PATH="${pkgs.swiftPackages.sourcekitd-inproc}/lib"

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

              # Minimum macOS deployment target
              MACOSX_DEPLOYMENT_TARGET = "14.0";
            };
          };

          apps = {
            format = {
              type = "app";
              program = "${
                pkgs.writeShellApplication {
                  name = "format";
                  runtimeInputs = with pkgs; [
                    nixfmt
                    swiftformat
                    treefmt
                    yamlfmt
                  ];
                  text = "treefmt \"$@\"";
                }
              }/bin/format";
            };

            lint = {
              type = "app";
              program = "${
                pkgs.writeShellApplication {
                  name = "lint";
                  runtimeInputs = with pkgs; [
                    apple-sdk_14
                    swiftlint
                  ];
                  text = ''
                    # Required for SwiftLint to find SourceKit framework
                    export DYLD_FRAMEWORK_PATH="${pkgs.swiftPackages.sourcekitd-inproc}/lib"
                    # Required to suppress xcode-select invocation by sourcekitdInProc
                    export DEVELOPER_DIR="${pkgs.apple-sdk_14}"
                    swiftlint lint --config .swiftlint.yaml --strict "$@"
                  '';
                }
              }/bin/lint";
            };
          };
        };
    };
}
