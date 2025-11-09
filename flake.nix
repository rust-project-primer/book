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
          src = pkgs.lib.fileset.toSource {
            root = ./.;
            fileset = pkgs.lib.fileset.unions [
              ./src
              ./examples
              ./theme
              ./assets
              ./book.toml
            ];
          };
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
        pdf = pkgs.stdenv.mkDerivation {
          name = "rust-project-primer-pdf";
          src = null;
          nativeBuildInputs = [
            pkgs.chromium
            pkgs.fontconfig
            pkgs.liberation_ttf
            pkgs.xorg.xvfb
          ];
          unpackPhase = ''
            cp -r ${book.out}/* .
            chmod -R u+rw .
          '';
          buildPhase = ''
            export HOME=$(pwd)/home
            export FONTCONFIG_PATH=${pkgs.fontconfig.out}/etc/fonts
            export FONTCONFIG_FILE=${pkgs.fontconfig.out}/etc/fonts/fonts.conf
            export DISPLAY=:99

            mkdir -p $HOME/.config/google-chrome/Crashpad
            mkdir -p $HOME/.cache/fontconfig
            mkdir -p $HOME/.local/share/fonts

            # Copy fonts to user directory
            cp -r ${pkgs.liberation_ttf}/share/fonts/truetype/* $HOME/.local/share/fonts/

            # Start virtual display
            ${pkgs.xorg.xvfb}/bin/Xvfb :99 -screen 0 1920x1080x24 &
            XVFB_PID=$!

            # Wait for display to be ready
            sleep 2

            # Generate PDF with timeout
            timeout 120 ${pkgs.chromium}/bin/chromium \
              --headless \
              --disable-gpu \
              --no-sandbox \
              --no-pdf-header-footer \
              --print-to-pdf=rust-project-primer.pdf \
              file://$(pwd)/print.html

            # Clean up
            kill $XVFB_PID 2>/dev/null || true
          '';
          installPhase = ''
            mkdir -p $out
            cp rust-project-primer.pdf $out/rust-project-primer.pdf
          '';
        };
      in
      {
        packages = {
          book = book;
          rust-project-primer = book;
          pdf = pdf;
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
            src = pkgs.lib.fileset.toSource {
              root = ./.;
              fileset = pkgs.lib.fileset.unions [
                (pkgs.lib.fileset.fileFilter (file: file.hasExt "md") ./.)
                ./.prettierrc.toml
              ];
            };
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
