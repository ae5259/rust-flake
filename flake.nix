{
  description = "Rust flake";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.11";
  };

  outputs =
    {
      self,
      nixpkgs,
      ...
    }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      formatter.${system} = pkgs.nixfmt;

      devShells.${system}.default = import ./shell.nix { inherit self pkgs; };
      packages.${system}.default = import ./default.nix { inherit pkgs; };
    };
}
