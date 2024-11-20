set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
set shell := ["bash", "-cu"]

test-parser-all:
  cargo test --package rusvelte_parser

test-parser TEST_NAME:
  cargo test --package rusvelte_parser --test snapshot -- {{TEST_NAME}}::test_success --exact --show-output 

lint:
  cargo clippy -- -D warnings