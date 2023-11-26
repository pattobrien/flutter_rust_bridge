# To use this file, install Just: `cargo install just` (or other ways)

DART_LINE_LENGTH := "120"

default:
    @# Make this the first recipe of justfile, such that when users type `just`, it will list all commands
    @just --list --unsorted --justfile {{justfile()}}

# ----------------------------------- linter -----------------------------------

lint mode="default": (lint_rust mode) (lint_dart mode)

# TODO frb_example packages are separated, are they ok?
lint_rust mode="default": (_lint_rust_main mode) (_lint_rust_wasm mode)

_lint_rust_main mode="default":
    cargo fmt \
      {{ if mode == "fix" { "" } else { "--check" } }}

    cargo clippy \
      {{ if mode == "fix" { "--fix" } else { "" } }} \
      -- -D warnings

_lint_rust_wasm mode="default":
    rustup target add wasm32-unknown-unknown
    cd frb_rust && \
      cargo clippy --target wasm32-unknown-unknown \
      {{ if mode == "fix" { "--fix" } else { "" } }} \
      -- -D warnings

lint_dart mode="default":
    just dart_pub_get

    just _lint_dart_single {{mode}} frb_dart dart 80
    just _lint_dart_single {{mode}} {{dir_example_pure_dart}}/dart dart {{DART_LINE_LENGTH}}
    just _lint_dart_single {{mode}} {{dir_example_pure_dart_multi}}/dart dart {{DART_LINE_LENGTH}}
    just _lint_dart_single {{mode}} {{dir_example_with_flutter}} dart {{DART_LINE_LENGTH}}

    just _lint_dart_pana

_lint_dart_single mode directory executable line_length:
    cd {{directory}} && dart format \
      --line-length {{line_length}} \
      {{ if mode == "fix" { "--fix" } else { "--output=none --set-exit-if-changed" } }} \
      .

    cd {{directory}} && {{executable}} analyze --fatal-infos

_lint_dart_pana:
    flutter pub global activate pana
    cd frb_dart && {{pana}} --no-warning --line-length 80 --exit-code-threshold 0
