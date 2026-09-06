# Implementation Plan: Reusable TextEditor Widget

**Branch**: No feature branch | **Date**: 2026-09-06 | **Spec**: [spec.md](spec.md)

**Status**: As-built plan synchronized with the current widget foundation.

## Summary

Extend the single-line editing architecture into a reusable multi-line `TextEditor`. Preserve line feeds, add logical-line navigation and newline insertion, retain clipboard/history/IME behavior, render multiple shaped logical lines inside a vertical scroll viewport, provide cursor reveal and a draggable vertical scrollbar, and expose read-only and wrapping-preference configuration. Soft wrapping and visual-line navigation are intentionally outside the synchronized implementation.

## Technical Context

**Language/Version**: Rust 1.97.1, edition 2024

**Primary Dependencies**: GPUI and `unicode-segmentation`

**Testing**: Unit tests in `text_editor.rs` plus GPUI contracts in `crates/widget/tests/text_editor_contract.rs` and `text_editor_editing.rs`

**Target**: Reusable desktop widget
**Constraints**: Plain text, normalized line feeds, logical-line layout, vertical scroll, grapheme-safe horizontal edits, platform input ranges, per-instance state

## Constitution Check

- **GPUI foundation**: PASS. Rendering, focus, input, clipboard, scrolling, timers, and events are GPUI-native.
- **Git backend**: PASS. The widget is independent of Git behavior.
- **English specifications**: PASS.
- **Rust workspace**: PASS. The reusable component remains in `crates/widget`.

## Project Structure

```text
crates/widget/
├── examples/text_editor.rs
├── src/lib.rs
├── src/text_editor.rs
├── src/theme.rs
├── tests/text_editor_contract.rs
└── tests/text_editor_editing.rs
```

## As-Built Design

### Document and interaction state

`TextEditor` owns normalized committed/display content, directed selection, composition, undo/redo snapshots, enabled/read-only/wrap preference, scroll state, shaped-line cache, pointer selection state, caret lifecycle, and theme. Public setters expose configuration without creating user-change events.

### Editing and navigation

The editor supports insertion, newline insertion, grapheme deletion, character and word motion, logical-line start/end, adjacent-line movement, document boundaries, ten-line page movement, and selection-extending variants. Clipboard, Undo/Redo, and input-method paths reuse common snapshot and commit logic.

### Layout and scrolling

Logical lines are shaped and painted in a clipped vertically scrollable region. Cursor movement estimates its logical row and adjusts the vertical scroll offset. Overflow creates a draggable vertical scrollbar. Current code exposes a wrap preference but does not soft-wrap lines or provide user-controlled horizontal scrolling.

## Validation

Automated evidence covers line-ending and Unicode conversion helpers, multi-line construction and configuration, read-only replacement rejection, insertion and selection replacement, and cursor reveal in a fixed viewport. Broader navigation, clipboard, history, IME, pointer, and scroll behavior exists in source but needs additional focused regression tests in a future feature if its scope expands.
