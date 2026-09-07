# Implementation Plan: Diff Editor

**Branch**: `[008-diff-editor]` | **Spec**: [spec.md](spec.md)

## Summary

Extract the parsed, virtualized DiffEditor from Git Graph History into a reusable widget feature. Keep repository loading in `crates/zit`, while the widget owns diff parsing, line-number presentation, split gutter/code layout, synchronized vertical scrolling, code-only horizontal scrolling, and scrollbar interactions.

## Technical Context

**Language**: Rust, edition 2024
**UI**: GPUI in the reusable `widget` crate
**Primary files**: `crates/widget/src/diff_editor.rs`, `crates/widget/src/scroll_container.rs`, `crates/widget/src/lib.rs`, `crates/zit/src/project/page.rs`
**Constraints**: no repository or Git types in the widget; no frame-scoped GPUI elements retained across frames; preserve fixed gutter and virtualization.

## Design

1. Parse unified diff text into owned row data.
2. Render separate gutter and code panes with a shared vertical contract and independent horizontal code scrolling.
3. Materialize only visible rows plus overscan while retaining total content dimensions.
4. Calculate scrollbar geometry from the actual pane bounds and max offsets.
5. Reset both scroll axes whenever a new diff is supplied.
6. Keep project-page loading and error handling outside the widget.

## Validation

- Parser and line-number fixtures.
- Large-diff virtualization checks.
- Fixed-gutter and synchronized-scroll interaction checks.
- Thumb drag, track click, hover color, boundary, and file-switch checks.
- `cargo check -p zit` and workspace formatting/tests.
