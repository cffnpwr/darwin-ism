{
  pkgs,
  ...
}:
let
  generated = pkgs.swiftpm2nix.helpers ./nix;
in
pkgs.stdenv.mkDerivation {
  pname = "darwin-ism";
  version = "0.1.0";

  src = ../../.;

  nativeBuildInputs = [ pkgs.swift_6 ];
  buildInputs = [ pkgs.apple-sdk_15 ];

  # SwiftPM requires less restrictive sandbox on macOS
  __noChroot = pkgs.stdenv.isDarwin;

  configurePhase = generated.configure;

  buildPhase = ''
    # Set library path for Swift runtime (needed for x86_64-darwin where
    # system Swift runtime resolution via dyld may fail)
    export DYLD_LIBRARY_PATH="${pkgs.swift_6}/lib/swift/macosx''${DYLD_LIBRARY_PATH:+:$DYLD_LIBRARY_PATH}"

    # Disable SwiftPM's sandbox to avoid conflicts with Nix sandbox
    swift build -c release --disable-sandbox
  '';

  installPhase = ''
    mkdir -p $out/bin
    cp .build/release/darwin-ism $out/bin/
  '';

  meta = with pkgs.lib; {
    description = "macOS Input Source Manager CLI";
    homepage = "https://github.com/cffnpwr/darwin-ism";
    license = licenses.mit;
    platforms = platforms.darwin;
  };
}
