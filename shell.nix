{ self, pkgs, ... }:
pkgs.mkShell {
  packages = with pkgs; [
    rustc
    cargo
    alejandra
    nixd
    deadnix
    statix

    rustc
    cargo
    rustfmt
    clippy
    rust-analyzer
    cargo-watch

    self.formatter.${system}
  ];
}
