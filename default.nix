{ pkgs, ... }:
let
  config = pkgs.lib.importTOML ./Cargo.toml;
  package = config.package;
in
pkgs.rustPlatform.buildRustPackage {
  pname = package.name;
  version = package.version;

  src = pkgs.lib.cleanSource ./.;
  cargoLock.lockFile = ./Cargo.lock;
}
