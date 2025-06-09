# book build script
# expects to run on alpine linux
# run with `just build`

MDBOOK_VERSION=0.4.51
MDBOOK_MERMAID_VERSION=0.15.0
MDBOOK_ADMONISH_VERSION=1.20.0
MDBOOK_READING_VERSION=0.3.0
MDBOOK_FILES_VERSION=0.2.0

# build book
apk add curl
curl -sSL "https://github.com/rust-lang/mdBook/releases/download/v$MDBOOK_VERSION/mdbook-v$MDBOOK_VERSION-x86_64-unknown-linux-musl.tar.gz" | tar -C /usr/local/bin -zxv
mdbook --version

curl -sSL "https://github.com/badboy/mdbook-mermaid/releases/download/v$MDBOOK_MERMAID_VERSION/mdbook-mermaid-v$MDBOOK_MERMAID_VERSION-x86_64-unknown-linux-musl.tar.gz" | tar -C /usr/local/bin -zxv
mdbook-mermaid --version

curl -sSL "https://github.com/tommilligan/mdbook-admonish/releases/download/v$MDBOOK_ADMONISH_VERSION/mdbook-admonish-v$MDBOOK_ADMONISH_VERSION-x86_64-unknown-linux-musl.tar.gz" | tar -C /usr/local/bin -zxv
mdbook-admonish --version

curl -sSL "https://github.com/xfbs/mdbook-files/releases/download/v$MDBOOK_FILES_VERSION/mdbook-files-v$MDBOOK_FILES_VERSION-x86_64-unknown-linux-musl.tar.gz" | tar -C /usr/local/bin -zxv
mdbook-files --version

curl -sSL "https://gitlab.com/rust-project-primer/mdbook-reading/-/releases/v$MDBOOK_READING_VERSION/downloads/mdbook-reading-amd64" -o /usr/local/bin/mdbook-reading
chmod +x /usr/local/bin/mdbook-reading
mdbook-reading --version

mdbook build
