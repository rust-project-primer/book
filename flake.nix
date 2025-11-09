{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    mdbook-files.url = "github:xfbs/mdbook-files";
    mdbook-reading.url = "github:rust-project-primer/mdbook-reading";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      mdbook-files,
      mdbook-reading,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        book = pkgs.stdenv.mkDerivation {
          src = ./.;
          name = "rust-project-primer";
          nativeBuildInputs = [
            pkgs.mdbook
            pkgs.just
            pkgs.mdbook-admonish
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
        pages = pkgs.stdenv.mkDerivation {
          name = "rust-project-primer-pages";
          src = null;
          nativeBuildInputs = [
            pkgs.gzip
            pkgs.brotli
          ];
          unpackPhase = ''
            pwd
            cp -r ${book.out}/* .
            chmod -R u+rw .
          '';
          buildPhase = ''
            pwd
            find . -not -name '*.gz' -not -name '*.br' -not -name '*.pdf' -type f -exec gzip -vk {} \;
            find . -not -name '*.gz' -not -name '*.br' -not -name '*.pdf' -type f -exec brotli -vk {} \;
          '';
          installPhase = ''
            pwd
            mkdir -p $out
            cp -r . $out/
          '';
        };
      in
      {
        packages = rec {
          rust-project-primer = book;
          gitlab-pages = pages;
          default = book;
        };
        apps = rec {
          #hello = flake-utils.lib.mkApp { drv = self.packages.${system}.hello; };
          #default = hello;
        };
        checks = {
          prettier-md = pkgs.stdenv.mkDerivation {
            name = "prettier-markdown-check";
            src = ./.;
            nativeBuildInputs = [ pkgs.nodePackages.prettier ];
            buildPhase = ''
              prettier --check '**/*.md'
            '';
            installPhase = ''
              mkdir -p $out
              echo "Prettier markdown check passed" > $out/result
            '';
          };
        };
      }
    );
}
