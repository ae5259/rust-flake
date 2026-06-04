{ pkgs, ... }:
let
  config = pkgs.lib.importTOML ./Cargo.toml;
  package = config.package;
in
pkgs.rustPlatform.buildRustPackage {
  pname = package.name;
  version = package.version;

  buildInputs = with pkgs; [
    libinput
  ];

  nativeBuildInputs = with pkgs; [
    pkg-config
    libinput
  ];

  src = pkgs.lib.cleanSource ./.;
  cargoLock.lockFile = ./Cargo.lock;

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
