
graphic NAME:
  mkdir -p src/graphics
  deno run --allow-read --allow-env --allow-net graphics/{{NAME}}.js > src/graphics/{{NAME}}.svg


submodules-update:
  git submodule update --init --remote
