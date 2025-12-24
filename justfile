#!/usr/bin/env -S just --justfile

set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
set shell := ["bash", "-cu"]

_default:
  @just --list -u

# Before committing, run this to make sure everything is ready for a PR.
pr:
  cargo fmt
  cargo check --all-targets --all-features
  cargo test
  cargo clippy --all-targets --all-features -- -D warnings
  just doc

[unix]
doc:
  RUSTDOCFLAGS='-D warnings' cargo doc --no-deps --document-private-items

[windows]
doc:
  $Env:RUSTDOCFLAGS='-D warnings'; cargo doc --no-deps --document-private-items
