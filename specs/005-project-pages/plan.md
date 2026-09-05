# Implementation Plan: Project Pages

**Branch**: `[005-project-pages]` | **Date**: 2026-09-06 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/005-project-pages/spec.md`

## Summary

Replace the shell's placeholder project tabs with retained pages backed by one application-wide project store. Restore a lightweight ordered session immediately, resolve each repository to a canonical identity with `gix`, and start commit-history loading on a background executor only when the corresponding page first becomes active. Each page owns an explicit lazy-load state and generation token so repeated activation, retry, close, and late completion remain isolated.

## Technical Context

**Language/Version**: Rust 1.97.1, edition 2024

**Primary Dependencies**: GPUI from the official Zed repository, `gix` 0.87.1, `serde`/`serde_json`, existing local `widget` crate

**Storage**: Versioned JSON session file in the per-user application data directory; Git repository data remains in each local repository

**Testing**: `cargo test --workspace`, GPUI test-support contexts, temporary on-disk Git repositories for integration tests, `cargo clippy`, and `cargo fmt`

**Target Platform**: Windows, macOS, and Linux desktop targets already supported by the application shell

**Project Type**: Native desktop application in a Cargo workspace

**Performance Goals**: Restore and display 20 project tabs within 2 seconds; show loading/ready feedback within 100 ms of activation; never load inactive history; keep tab switching responsive while history is read

**Constraints**: UI-thread work must remain lightweight; Git operations use `gix`, not the Git CLI; exactly one retained page per open repository; at most one current history load per project; closed-page results must be ignored; offline-capable

**Scale/Scope**: Up to 20 concurrently restored local repositories for acceptance validation; session-scoped history cache; commit listing for the active project; opening new repositories and live refresh are excluded

## Constitution Check

*GATE: Passed before Phase 0 and re-checked after Phase 1.*

- **GPUI foundation**: PASS. Project pages, global state, events, retained entities, background work, and rendering remain GPUI-native.
- **Git backend**: PASS. Repository discovery, identity, HEAD resolution, and revision walking use `gix` 0.87.1. No Git CLI fallback is planned.
- **English specifications**: PASS. The specification and all planning artifacts are written in English.
- **Rust workspace**: PASS. Work remains in the existing root Cargo workspace and its member crates.
- **Application entry point**: PASS. `zit` remains the executable package and `crates/zit/src/main.rs` remains its entry point.
- **Post-design re-check**: PASS. The data model, UI contract, persistence design, and validation guide introduce no constitutional exception.

## Project Structure

### Documentation (this feature)

```text
specs/005-project-pages/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── project-page.md
└── tasks.md                 # Created later by $speckit-tasks
```

### Source Code (repository root)

```text
crates/
├── zit/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs          # Initialize global store and restore session
│   │   ├── shell.rs         # Bind store entries to retained tabs/pages
│   │   └── project/
│   │       ├── mod.rs       # Module exports and initialization
│   │       ├── model.rs     # IDs, metadata, history, load states
│   │       ├── store.rs     # Application-wide authoritative project store
│   │       ├── session.rs   # Versioned JSON load/save and normalization
│   │       ├── repository.rs# gix discovery, identity, metadata, history
│   │       └── page.rs      # GPUI project page and lazy-load lifecycle
│   └── tests/
│       ├── project_store_contract.rs
│       ├── project_page_contract.rs
│       └── project_session_integration.rs
└── widget/
    ├── src/tab_bar.rs       # Existing retained tab/page and event contract
    └── tests/tab_bar_contract.rs
```

**Structure Decision**: Keep domain and page orchestration in the `zit` application crate because project state is application-wide and not a reusable generic widget. Continue using the existing `widget::TabBar` as the navigation/retention primitive. Split the project feature by model, persistence, repository access, store, and page lifecycle so Git work is testable without rendering and page behavior is testable with injected loaders.

## Complexity Tracking

No constitutional violations require justification.
