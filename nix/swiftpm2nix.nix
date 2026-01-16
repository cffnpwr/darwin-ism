# Patch swiftpm2nix to support workspace-state.json version 7 (Swift 6)
{ pkgs, ... }:
let
  inherit (pkgs) lib;
in
pkgs.runCommand "swiftpm2nix"
  {
    nativeBuildInputs = [ pkgs.makeWrapper ];
    meta.mainProgram = "swiftpm2nix";

    passthru.helpers =
      generated:
      let
        inherit (import generated) workspaceStateFile hashes;
        workspaceState = lib.importJSON workspaceStateFile;
        pinFile = (pkgs.formats.json { }).generate "Package.resolved" {
          version = 1;
          object.pins = map (dep: {
            package = dep.packageRef.name;
            repositoryURL = dep.packageRef.location;
            state = dep.state.checkoutState;
          }) workspaceState.object.dependencies;
        };
        sources = lib.listToAttrs (
          map (
            dep:
            lib.nameValuePair dep.subpath (
              pkgs.fetchgit {
                url = dep.packageRef.location;
                rev = dep.state.checkoutState.revision;
                sha256 = hashes.${dep.subpath};
                fetchSubmodules = true;
              }
            )
          ) workspaceState.object.dependencies
        );
      in
      {
        inherit sources;
        configure = ''
          mkdir -p .build/checkouts
          ln -sf ${pinFile} ./Package.resolved
          install -m 0600 ${workspaceStateFile} ./.build/workspace-state.json
        ''
        + lib.concatStrings (
          lib.mapAttrsToList (name: src: ''
            ln -s '${src}' '.build/checkouts/${name}'
          '') sources
        );
      };
  }
  ''
    mkdir -p $out/bin
    sed 's/\$stateVersion -gt 6/\$stateVersion -gt 7/g' \
      ${pkgs.swiftpm2nix}/bin/.swiftpm2nix-wrapped > $out/bin/.swiftpm2nix-wrapped
    chmod +x $out/bin/.swiftpm2nix-wrapped
    makeWrapper $out/bin/.swiftpm2nix-wrapped $out/bin/swiftpm2nix \
      --prefix PATH : ${
        lib.makeBinPath [
          pkgs.jq
          pkgs.nurl
        ]
      }
  ''
