{
  pkgs,
  ...
}:
let
  generated = pkgs.swiftpm2nix.helpers ./generated;
in
pkgs.stdenv.mkDerivation {
  pname = "darwin-ism";
  version = "0.1.0";

  src = ../../.;

  nativeBuildInputs = with pkgs; [
    apple-sdk_14
    swift
    swiftPackages.swiftpm
  ];

  # SwiftPM requires less restrictive sandbox on macOS
  __noChroot = pkgs.stdenv.isDarwin;

  configurePhase = generated.configure;

  buildPhase = ''
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
