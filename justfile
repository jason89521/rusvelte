set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
set shell := ["bash", "-cu"]

test-parser-all:
  cargo test --package rusvelte_parser

test-parser TEST_NAME:
  cargo test --package rusvelte_parser --test snapshot -- {{TEST_NAME}}::test_success --exact --show-output 

test-transformer:
  cargo test --package rusvelte_transformer --test snapshot

lint:
  cargo clippy -- -D warnings

build-wasm mode="release":
  wasm-pack build --out-dir ../../npm/wasm --target web --{{mode}} --scope rusvelte crates/rusvelte_wasm

update-patches:
  cargo update oxc_allocator oxc_ast oxc_parser oxc_span oxc_syntax oxc_diagnostics

run-example package="analyzer" example=package:
  cargo run --package rusvelte_{{package}} --example {{example}}
