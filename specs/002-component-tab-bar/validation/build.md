# Build Validation

**Date**: 2026-09-05  
**Platform**: Windows, automated GPUI test backend  
**GPUI revision**: `5a9b9558` from the official Zed repository

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo check --workspace --all-targets --locked` | PASS |
| `cargo test -p widget --locked` | PASS, 34 tests including 5 tab-bar tests |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | PASS |

The `tab_bar` example target is included in the all-targets build. Native interactive execution is
tracked separately because this run did not launch a graphical window.
