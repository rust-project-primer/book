
list:
  just --list

# build the graphic
graphic NAME:
  mkdir -p src/graphics
  deno run --allow-read --allow-env --allow-net graphics/{{NAME}}.js > src/graphics/{{NAME}}.svg

# update submodules to latest commit hash
submodules-update:
  git submodule update --init --remote
