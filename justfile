build:
  cargo build --release

build-debug:
  cargo build

test: build-debug
  deno run --unstable --allow-plugin --allow-read --allow-write demo.ts
