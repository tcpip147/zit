# Implementation Plan: Component Tab Bar

**Branch**: No feature branch | **Date**: 2026-09-06 | **Spec**: [spec.md](spec.md)

**Status**: As-built plan synchronized with the current component.

## Summary

Implement `TabBar` as a reusable retained-page GPUI entity in the `widget` crate. Stable `TabId` values identify fixed-order entries; each `TabSpec` owns a name, SVG reference, and retained page. Selection displays only the matching page, closure follows a deterministic neighbor rule with host veto, metadata updates preserve page identity, and the tab row uses tracked horizontal overflow.

## Technical Context

**Language/Version**: Rust 1.97.1, edition 2024

**Primary Dependency**: GPUI from the official Zed repository

**Assets**: Project-authored close and fallback SVGs under `crates/widget/assets/tab-bar/`

**Testing**: GPUI test context in `crates/widget/tests/tab_bar_contract.rs`

**Target**: Reusable desktop widget
**Constraints**: Unique IDs, one retained page per tab, at most one active tab, deterministic close selection, pointer interaction, horizontal overflow

## Constitution Check

- **GPUI foundation**: PASS. The component is a GPUI entity and view.
- **Git backend**: PASS. The component has no repository dependency and does not weaken the workspace's `gix` foundation.
- **English specifications**: PASS.
- **Rust workspace**: PASS. The reusable component remains in `crates/widget`.

## Project Structure

```text
crates/widget/
├── assets/tab-bar/{close.svg,fallback-project.svg,manifest.json}
├── examples/tab_bar.rs
├── src/lib.rs
├── src/tab_bar.rs
└── tests/tab_bar_contract.rs
```

## As-Built Design

### State and validation

`TabBar` stores ordered entries, an optional active ID, a tracked scroll handle, a close predicate, and theme state. Construction and insertion reject duplicate IDs. An absent or unknown active ID falls back to the first entry.

### Selection and closure

Selection scrolls toward the target and emits `SelectionChanged` only for a changed identity. Close emits `CloseRequested`, consults the host predicate, removes the entry, and selects the right neighbor before the left neighbor when the active entry closes.

### Rendering

The component renders one tab row and only the active retained page. Each tab uses a fixed width, clipped label, SVG icon, separate close target, pointer hover states, and semantic tab metadata. Empty icon references use the bundled fallback.

## Validation

### Current Implementation Alignment

Validate retained page identity, close-veto behavior, overflow reveal, fallback icons, and independent tab instances through the widget contract tests and example.

Contract tests cover initial fallback, stable page identity, duplicate rejection, deterministic closure, close veto, and metadata updates. Interactive overflow and pointer presentation are available in `crates/widget/examples/tab_bar.rs`.
