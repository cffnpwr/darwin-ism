{ pkgs }:
let
  pname = "swift";
  version = "6.2.3";
in
pkgs.stdenv.mkDerivation {
  inherit pname version;

  src = pkgs.fetchurl {
    url = "https://download.swift.org/swift-${version}-release/xcode/swift-${version}-RELEASE/swift-${version}-RELEASE-osx.pkg";
    hash = "sha256-we2Ez1QyhsVJyqzMR+C0fYxhw8j+284SBd7cvr52Aag=";
  };

  nativeBuildInputs = with pkgs; [
    xar
    gzip
    cpio
  ];

  unpackPhase = ''
    runHook preUnpack

    xar -xf $src
    zcat swift-6.2.3-RELEASE-osx-package.pkg/Payload | cpio -idm

    runHook postUnpack
  '';

  installPhase = ''
    mkdir -p $out
    cp -r usr/* $out/
  '';

  meta = with pkgs.lib; {
    description = "Swift programming language";
    homepage = "https://swift.org";
    license = licenses.asl20;
    platforms = platforms.darwin;
  };
}