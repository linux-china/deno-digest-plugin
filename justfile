build:
  cargo build --release

build-debug:
  cargo build

test: build-debug
  rm -rf .deno_plugins
  deno run --no-check --unstable --allow-plugin --allow-env --allow-read --allow-write demo.ts
