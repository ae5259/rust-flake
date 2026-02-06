{ pkgs, ... }:
pkgs.rustPlatform.buildRustPackage {
  pname = "sae";
  version = "0.1";

  src = pkgs.lib.cleanSource ./.;
  cargoLock.lockFile = ./Cargo.lock;
}
