# Implementation Plan: Reusable TextInput Widget

**Branch**: No feature branch | **Date**: 2026-09-06 | **Spec**: [spec.md](spec.md)

**Status**: As-built plan synchronized with the current widget.

## Summary

Implement a reusable, single-line GPUI text input with normalized content, grapheme-safe editing, keyboard and pointer selection, clipboard operations, undo/redo snapshots, IME composition, horizontal cursor reveal, deterministic caret blinking, theme integration, and exactly-once committed-value events.

## Technical Context

**Language/Version**: Rust 1.97.1, edition 2024

**Primary Dependencies**: GPUI and `unicode-segmentation`

**Testing**: GPUI test-support suites under `crates/widget/tests/text_input_*.rs`

**Target**: Reusable desktop widget
**Constraints**: Single-line value, user-perceived character boundaries, platform key bindings, provisional IME state distinct from committed text, no application business rules

## Constitution Check

- **GPUI foundation**: PASS. Rendering, focus, input handling, clipboard, timers, and events use GPUI.
- **Git backend**: PASS. The widget has no Git behavior and does not change the workspace foundation.
- **English specifications**: PASS.
- **Rust workspace**: PASS. The component remains in `crates/widget`.

## Project Structure

```text
crates/widget/
├── examples/text_input.rs
├── src/lib.rs
├── src/text_input.rs
├── src/theme.rs
└── tests/text_input_*.rs
```

## As-Built Design

### State and edits

`TextInput` owns committed and display content, placeholder, enabled/focus state, directed selection, provisional composition, undo/redo snapshots, horizontal offset, blink generation, and theme. External value replacement resets selection and history without emitting a user event.

### Input integration

One initialization function registers platform key bindings. GPUI input-handler methods convert UTF-16 platform ranges to safe UTF-8 boundaries, expose selected and marked ranges, and distinguish provisional composition updates from commit and cancellation.

### Rendering and lifecycle

A custom element shapes one line, paints selection, marked text, and a display-pixel-aligned caret, and keeps the cursor visible horizontally. Focus and interaction restart a generation-checked 500 ms blink cycle. Minimum height derives from current line metrics plus visual chrome.

## Validation

Focused suites cover foundation, editing, navigation, pointer focus, clipboard, history, IME, blinking, rendering constraints, Unicode regression, and independent configuration. The interactive example exercises multiple instances.
