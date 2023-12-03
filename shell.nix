{ pkgs ? import <nixpkgs> {} }:
with pkgs;
let
  pinnedPkgs = fetchFromGitHub {
    owner = "NixOS";
    repo = "nixpkgs";
    rev = "19cbff58383a4ae384dea4d1d0c823d72b49d614";
    sha256 = "sha256-yrQ8osMD+vDLGFX7pcwsY/Qr5PUd6OmDMYJZzZi0+zc=";
  };

  pkgs = import pinnedPkgs {};

  inherit (lib) optional optionals;
  inherit (darwin.apple_sdk.frameworks) Cocoa CoreGraphics Foundation IOKit Kernel OpenGL Security;
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    cargo-audit
    clippy
    libiconv
    rustc
    openssl
    pkg-config
  ] ++ optionals stdenv.isDarwin [
    Cocoa
    CoreGraphics
    Foundation
    IOKit
    Kernel
    OpenGL
    Security
    libpng
    zlib
  ];

  RUSTONIG_DYNAMIC_LIBONIG = "1";
}
