# Implementation Plan: New Repository

**Branch**: `[011-new-repository]` | **Date**: 2026-09-08 | **Spec**: [spec.md](spec.md)

## Summary

Enable the existing New Repository menu command, collect a project name and parent folder,
initialize the selected folder itself as a Git repository with gix, then persist and activate it through ProjectStore.

## Technical Context

**Language/Version**: Rust 2024
**Primary Dependencies**: GPUI, gix, rfd, existing project store
**Storage**: Local filesystem and existing project session JSON
**Testing**: cargo check and cargo test
**Target Platform**: Windows desktop
**Constraints**: Never overwrite an existing destination; preserve existing Open Project behavior.

## Constitution Check

- GPUI and gix foundation: PASS
- Rust workspace and zit entry point: PASS
- English specifications: PASS

## Project Structure

- `crates/zit/src/platform.rs`: enable command dispatch
- `crates/zit/src/shell.rs`: dialog and submission flow
- `crates/zit/src/project/store.rs`: repository creation and activation
- `crates/zit/tests/`: menu, store, and shell contracts

## Implementation Sequence

1. Enable New Repository and open the reusable dialog in creation mode.
2. Validate input, initialize the child repository, and activate it.
3. Add failure/cancellation tests and run workspace checks.
