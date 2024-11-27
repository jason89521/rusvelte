set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
set shell := ["bash", "-cu"]

test-parser-all:
  cargo test --package rusvelte_parser

test-parser TEST_NAME:
  cargo test --package rusvelte_parser --test snapshot -- {{TEST_NAME}}::test_success --exact --show-output 

lint:
  cargo clippy -- -D warnings

build-wasm mode="release":
  wasm-pack build --out-dir ../../npm/wasm --target web --{{mode}} --scope rusvelte crates/rusvelte_wasm