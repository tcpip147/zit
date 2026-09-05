# Build Validation

**Date**: 2026-09-05  
**Host**: Windows, x86_64-pc-windows-msvc  
**Toolchain**: Rust 1.97.1 (pinned by `rust-toolchain.toml`)  
**GPUI source**: official `https://github.com/zed-industries/zed`, locked at `5a9b9558`  
**gix**: 0.87.1

## Results

| Check | Result | Notes |
|---|---|---|
| `cargo metadata --locked --format-version 1 --no-deps` | PASS | `zit` resolves `gpui` and `gpui_platform` from the official Zed Git source and `gix` from crates.io. |
| `cargo check --workspace --all-targets --locked` | PASS | Includes the `chrome_probe` example and Zit contract tests. |
| `cargo test --workspace --locked` | PASS | 43 tests passed across `zit` and the preserved `widget` crate. |
| `cargo fmt --all -- --check` | PASS | Recorded after formatting the implementation. |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | PASS | Recorded after resolving test-target dead code. |

## Native scope

Compilation and deterministic contract checks pass on this host. Linux X11/Wayland, macOS,
assistive-technology, reference-image, multi-monitor scale, and repeated-launch checks remain
NOT RUN and are not represented as passing evidence.
