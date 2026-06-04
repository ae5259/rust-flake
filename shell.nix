{ self, pkgs, ... }:
pkgs.mkShell {
  packages = with pkgs; [
    # Rust
    rustc
    cargo
    rustfmt
    clippy
    rust-analyzer
    cargo-watch
    pkgconf
    libinput

    # Nix
    alejandra
    nixd
    deadnix
    statix
    self.formatter.${system}
  ];

  nativeBuildInputs = with pkgs; [
    pkg-config
    libinput
  ];

  buildInputs = with pkgs; [
    libinput
  ];
}
