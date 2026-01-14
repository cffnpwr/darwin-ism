{
  pkgs,
  ...
}:
let
  arch = if pkgs.stdenv.hostPlatform.system == "aarch64-darwin" then "arm64" else "x86_64";
  target = "${arch}-apple-macos15";
in
pkgs.stdenv.mkDerivation {
  pname = "darwin-ism";
  version = "0.1.0";

  src = ../.;

  nativeBuildInputs = [ pkgs.swift_6 ];
  buildInputs = [ pkgs.apple-sdk_15 ];

  buildPhase = ''
    swiftc -O -o darwin-ism \
      -target ${target} \
      -framework Carbon \
      -framework Foundation \
      Sources/darwin-ism/*.swift
  '';

  installPhase = ''
    mkdir -p $out/bin
    cp darwin-ism $out/bin/
  '';

  meta = with pkgs.lib; {
    description = "macOS Input Source Manager CLI";
    homepage = "https://github.com/cffnpwr/darwin-ism";
    license = licenses.mit;
    platforms = platforms.darwin;
  };
}
