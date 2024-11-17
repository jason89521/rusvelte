set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
set shell := ["bash", "-cu"]

test-parser TEST_NAME:
  cargo test --package rusvelte-parser --test snapshot -- {{TEST_NAME}}::test_success --exact --show-output 