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
            cp -r ${book.out}/* .
            chmod -R u+rw .
          '';
          buildPhase = ''
            find . -not -name '*.gz' -not -name '*.br' -not -name '*.pdf' -type f -exec gzip -vk {} \;
            find . -not -name '*.gz' -not -name '*.br' -not -name '*.pdf' -type f -exec brotli -vk {} \;
          '';
          installPhase = ''
            mkdir -p $out
            cp -r . $out/
          '';
        };
      in
      {
        packages = {
          book = book;
          rust-project-primer = book;
          pages = pages;
          default = book;
        };
        apps = {
          fmt = {
            type = "app";
            program = "${pkgs.writeShellScriptBin "prettier-fmt" ''
              ${pkgs.nodePackages.prettier}/bin/prettier --write 'src/**/*.md' '*.md'
            ''}/bin/prettier-fmt";
            meta = {
              description = "Format Markdown files with Prettier";
            };
          };
          serve = {
            type = "app";
            program = "${pkgs.writeShellScriptBin "serve-book" ''
              echo "Building book..."
              BOOK_DIR=$(mktemp -d)
              cp -r ${book}/* $BOOK_DIR/
              echo "Book built successfully!"
              echo "Starting server at http://localhost:8000"
              echo "Press Ctrl+C to stop the server"
              cd $BOOK_DIR
              ${pkgs.python3}/bin/python3 -m http.server 8000
            ''}/bin/serve-book";
            meta = {
              description = "Build and serve the mdbook on localhost:8000";
            };
          };
          default = self.apps.${system}.serve;
        };
        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.nodePackages.prettier
            pkgs.mdbook
            pkgs.just
            pkgs.mdbook-admonish
            mdbook-files.packages.${system}.default
            mdbook-reading.packages.${system}.default
          ];
        };
        checks = {
          prettier-md = pkgs.stdenv.mkDerivation {
            name = "prettier-markdown-check";
            src = ./.;
            nativeBuildInputs = [ pkgs.nodePackages.prettier ];
            buildPhase = ''
              prettier --check 'src/**/*.md' '*.md'
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
