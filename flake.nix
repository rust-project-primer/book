{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    mdbook-files.url = "github:xfbs/mdbook-files";
    mdbook-reading.url = "github:rust-project-primer/mdbook-reading";
  };

  outputs = { self, nixpkgs, flake-utils, mdbook-files, mdbook-reading }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        book = pkgs.stdenv.mkDerivation {
          src = ./.;
          name = "rust-project-primer";
          nativeBuildInputs = [
            pkgs.mdbook
            pkgs.just
            pkgs.mdbook-admonish
            pkgs.mdbook-mermaid
            mdbook-files.packages.${system}.default
            mdbook-reading.packages.${system}.default
          ];
          buildPhase = ''
            mdbook build
          '';
          installPhase = ''
            mkdir -p $out
            cp -r book/* $out/
          '';
        };
      in
      {
        packages = rec {
          rust-project-primer = book;
          default = book;
        };
        apps = rec {
          hello = flake-utils.lib.mkApp { drv = self.packages.${system}.hello; };
          default = hello;
        };
      }
    );
}
