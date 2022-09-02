build:
  @cargo build --release

test: build
  cargo test

run: build
  cargo run --release

build-watch:
  cargo watch -- just build

test-watch:
  cargo watch -- just test

run-watch:
  cargo watch -- just run
