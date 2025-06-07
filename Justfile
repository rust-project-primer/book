
list:
  just --list

# build the graphic
graphic NAME:
  mkdir -p src/graphics
  deno run --allow-read --allow-env --allow-net graphics/{{NAME}}.js > src/graphics/{{NAME}}.svg

admonish-update:
  mdbook-admonish generate-custom assets/mdbook-admonish-custom.css
  mdbook-admonish install --css-dir assets

# update submodules to latest commit hash
submodules-update:
  git submodule update --init --remote

compress:
  find public -not -name '*.gz' -not -name '*.br' -type f -exec gzip -vk {} \;
  find public -not -name '*.gz' -not -name '*.br' -type f -exec brotli -vk {} \;
