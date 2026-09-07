# Implementation Plan: Custom Application Chrome

**Branch**: No feature branch | **Date**: 2026-09-06 | **Spec**: [spec.md](spec.md)

**Status**: As-built plan synchronized with the current source.

## Summary

The `zit` executable opens a decoration-free GPUI window and renders application-owned chrome in `ZitShell`. Platform policy selects SVG controls and routes supported commands to GPUI window operations, with a narrow Windows restore adapter. File and Help menus, About, application exit, window reopen, and a fallback startup-error window are integrated into the same shell.

## Technical Context

**Language/Version**: Rust 1.97.1, edition 2024

**Primary Dependencies**: GPUI and `gpui_platform` from the official Zed repository; `gix`; target-only `windows` and `raw-window-handle` for Windows restore

**Assets**: Platform-specific SVGs under `crates/zit/assets/window-controls/`

**Testing**: `crates/zit/tests/window_contract.rs`, `crates/zit/tests/menu_contract.rs`, module tests, and manual native validation records

**Target Platform**: Windows, Linux X11/Wayland, and macOS desktop
**Constraints**: One custom title bar, no duplicate native chrome, disabled repository menu commands, platform capability guards, 480 by 320 minimum window size

## Constitution Check

- **GPUI foundation**: PASS. Window creation, rendering, input, accessibility, and application lifecycle use GPUI facilities.
- **Git backend**: PASS. No Git behavior is introduced by this feature; the workspace retains `gix`.
- **English specifications**: PASS.
- **Rust workspace and entry point**: PASS. The executable remains `crates/zit/src/main.rs`.
- **Bounded platform adapter**: PASS. Windows restore uses a target-only adapter because the pinned GPUI maximize operation does not restore a maximized window.

## Project Structure

```text
crates/zit/
├── assets/window-controls/{windows,linux,macos}/
├── examples/chrome_probe.rs
├── src/main.rs
├── src/shell.rs
├── src/platform.rs
├── src/titlebar.rs
├── src/menu.rs
├── src/about.rs
├── src/assets.rs
├── src/startup_error.rs
└── tests/{window_contract,menu_contract}.rs
```

## As-Built Design

### Application lifecycle

`main.rs` loads the theme, installs bundled assets, opens the main window, registers menu key actions, reopens a window when application activation finds none, and falls back to `StartupError` when main-window creation fails.

### Shell and menus

`shell.rs` owns menu state, About visibility, focus, platform profile, and window subscriptions. It renders the menu bar, platform-ordered controls, draggable region, content, popups, and modal overlay. `menu.rs` defines immutable menu contents and enabled-command navigation.

### Platform commands and assets

`platform.rs` snapshots capabilities and state, rejects disabled repository commands, and dispatches close, minimize, maximize/restore, full-screen, and exit. `assets.rs` maps semantic control names to platform SVG paths.

## Validation

### Current Implementation Alignment

Validate `main.rs`, `shell.rs`, `platform.rs`, `titlebar.rs`, `assets.rs`, and `startup_error.rs` together; confirm platform ordering, accessible command routing, reopen behavior, and simulated startup failure.

- Deterministic command and menu contracts run with `cargo test --workspace`.
- `chrome_probe` supports interactive host validation.
- Exact native appearance, display-scale matrices, and assistive-technology runs remain release validation, not completed implementation claims.
