# Implementation Plan: File Status

**Branch**: `[009-file-status]` | **Spec**: [spec.md](spec.md)

## Summary

Extract the File Status UI from Project Pages into an independent feature. Keep the split layout in the project page, use DiffEditor for selected changes, and add a repository status boundary for staged, unstaged, stage, unstage, and commit operations.

## Technical Context

**Language**: Rust, edition 2024
**UI**: GPUI, local widget crate
**Repository**: Existing gix-backed repository loader
**Primary files**: `crates/zit/src/project/page.rs`, `crates/zit/src/project/repository.rs`, `crates/widget/src/text_editor.rs`, `crates/widget/src/diff_editor.rs`

## Design

1. Keep File Status state per retained ProjectPage.
2. Model staged and unstaged entries as repository-owned status data.
3. Render a horizontal shell with a one-part left workflow column and a two-part right DiffEditor column; split the left column vertically into staged and unstaged files, then render commit controls in a full-width footer below the horizontal shell.
4. Render the footer as author metadata, multi-line TextEditor, two commit-option checkboxes, and a right-aligned Commit action.
5. Execute stage/unstage/commit work asynchronously with generation-safe updates and user-safe errors.
6. Refresh status after every successful repository mutation.
7. Measure the File Status container bounds during layout, then convert each drag delta from the pointer start position using that measured axis; use an 8px centered hit area without consuming layout space.

## Validation

- Empty, staged, unstaged, and mixed working-tree fixtures.
- File selection and DiffEditor integration.
- Stage/unstage/commit repository integration.
- Missing-message and repository-error handling.
- Per-project state isolation.
