# Implementation Plan: Project Pages

**Branch**: `[005-project-pages]` | **Date**: 2026-09-06 | **Spec**: [spec.md](spec.md)

**Status**: As-built plan synchronized with the current application.

## Summary

Replace placeholder page content with retained project workspaces backed by one application-wide project store. Restore a versioned session, inspect repositories through `gix`, deduplicate canonical Git identities, create one retained page per project, and load commit history on first activation. Each page owns independent workspace navigation, a toolbar action boundary, explicit load state, generation-checked background work, a virtualized history table, and retry behavior.

## Technical Context

**Language/Version**: Rust 1.97.1, edition 2024

**Primary Dependencies**: GPUI, `gpui_platform`, `gix` 0.87.1, `serde`, `serde_json`, and local `widget`

**Storage**: Versioned JSON session under the platform application-data/configuration directory or `ZIT_SESSION` override

**Testing**: GPUI contracts and temporary on-disk repositories in `crates/zit/tests/project_*.rs`

**Target Platform**: Windows, Linux, and macOS desktop
**Constraints**: One global store, one retained page per canonical repository, inactive history stays unloaded, one current load generation, stale/closed results ignored, Git access through `gix`

## Constitution Check

- **GPUI foundation**: PASS. Global state, retained pages, events, background tasks, and rendering are GPUI-native.
- **Git backend**: PASS. Discovery and history traversal use `gix`; no Git CLI fallback exists.
- **English specifications**: PASS.
- **Rust workspace and entry point**: PASS. Work remains in `crates/zit` and startup remains in `crates/zit/src/main.rs`.

## Project Structure

```text
crates/zit/
├── assets/project-toolbar/*.svg
├── src/main.rs
├── src/shell.rs
├── src/project/
│   ├── mod.rs
│   ├── model.rs
│   ├── repository.rs
│   ├── session.rs
│   ├── store.rs
│   └── page.rs
└── tests/project_*.rs

crates/widget/src/tab_bar.rs
```

## As-Built Design

### Repository and session boundary

`GixRepositoryLoader` discovers a repository, derives canonical Git-directory identity, preserves unavailable entries with safe diagnostics, and walks HEAD history into owned commit summaries. `OpenProjectSession` reads and atomically replaces versioned JSON while preserving project order and the active ID.

### Global store and shell

`ProjectStore` owns ordered metadata, retained page entities, active identity, persistence, and events. Restore deduplicates identities and chooses the persisted active project or an available fallback. `ZitShell` builds `TabSpec` values, reconciles selection/close/metadata events, activates only the selected retained page, and renders the empty state through `TabBar`.

### Project page

`ProjectPage` owns metadata, load generation, background task, load state, selected File Status/History/Search section, virtual history scroll state, and toolbar events. First activation starts one background history load; ready data is retained, failure exposes Retry, and cancellation invalidates late completion. History renders graph, summary, author, date, and short hash columns with a draggable transient scrollbar.

### Development bootstrap

When no restored projects exist, current startup code seeds three local test repositories and may replace the first with a machine-specific sample repository. This is development scaffolding, not a product requirement, and remains isolated in store helper methods and `main.rs`.

## Validation

Store, session, repository, page, performance, and tab-bar contracts cover ordered uniqueness, persistence, availability isolation, stable page identity, activation-only loading, retry, stale results, canonical aliases, history summaries, empty repositories, project-local section state, and 20-project startup behavior.
