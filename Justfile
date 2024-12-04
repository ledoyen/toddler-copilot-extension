#!/usr/bin/env -S just --justfile

set dotenv-filename := '.env-just'

_default:
  @just --list --unsorted --justfile '{{justfile()}}'

fmt:
  cargo fmt --all

fmt-check:
  cargo fmt --all --check

clippy:
  cargo clippy -- \
    -W clippy::pedantic \
    -W clippy::nursery \
    -W clippy::unwrap_used \
    -W clippy::expect_used \
    -A clippy::significant_drop_tightening \
    -A clippy::no_effect_underscore_binding \
    -A clippy::missing_errors_doc

build *FLAGS='':
  cargo build {{FLAGS}}

test:
  cargo test

run:
   cargo shuttle run --debug

rund:
   export RUST_LOG="debug" && export RUST_BACKTRACE=1 && just run

shuttle-restart:
  cargo shuttle project restart --idle-minutes 0
