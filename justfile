build:
  cargo build --release

build-debug:
  cargo build

demo:
  deno run --unstable --allow-plugin demo.ts
