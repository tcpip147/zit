---
description: "Completed implementation record for custom application chrome"
---

# Tasks: Custom Application Chrome

**Status**: As-built task record; every listed task is present in the current source.

## Phase 1: Foundation

- [X] T001 Configure the Rust workspace, official GPUI dependencies, gix, and target-only Windows restore dependencies in `Cargo.toml` and `crates/zit/Cargo.toml`
- [X] T002 Create the executable application bootstrap and decoration-free main-window lifecycle in `crates/zit/src/main.rs`
- [X] T003 [P] Define platform profiles, capability snapshots, command availability, and command dispatch in `crates/zit/src/platform.rs`
- [X] T004 [P] Register bundled assets and platform-specific SVG lookup in `crates/zit/src/assets.rs` and `crates/zit/assets/window-controls/`

## Phase 2: User Story 1 - Use the Application Window

- [X] T005 [US1] Build the custom shell, window subscriptions, minimum layout, and draggable title-bar region in `crates/zit/src/shell.rs`
- [X] T006 [US1] Render platform-ordered close, minimize, maximize/restore, and full-screen controls in `crates/zit/src/shell.rs` and `crates/zit/src/titlebar.rs`
- [X] T007 [US1] Route enabled controls to actual window operations and accessible click handlers in `crates/zit/src/platform.rs` and `crates/zit/src/shell.rs`
- [X] T008 [US1] Cover command availability and dispatch boundaries in `crates/zit/tests/window_contract.rs`

## Phase 3: User Story 2 - Use File and Help Menus

- [X] T009 [P] [US2] Define exact File and Help menu items, disabled repository commands, and keyboard selection rules in `crates/zit/src/menu.rs`
- [X] T010 [US2] Render pointer-operated headings and popups with single-menu state and dismissal behavior in `crates/zit/src/shell.rs`
- [X] T011 [US2] Bind directional, Enter, and Escape menu actions and route Exit and About in `crates/zit/src/main.rs` and `crates/zit/src/shell.rs`
- [X] T012 [US2] Render and dismiss the Zit version About dialog in `crates/zit/src/about.rs` and `crates/zit/src/shell.rs`
- [X] T013 [US2] Verify menu labels, ordering, state, and disabled-command safety in `crates/zit/tests/menu_contract.rs`

## Phase 4: User Story 3 - Recover From Startup Failure

- [X] T014 [US3] Implement a visible fallback startup-error view in `crates/zit/src/startup_error.rs`
- [X] T015 [US3] Route simulated or real main-window creation failure to the fallback window and quit if fallback creation fails in `crates/zit/src/main.rs`

## Phase 5: Validation

- [X] T016 Run workspace build, formatting, lint, and test gates and record native validation gaps in `specs/001-custom-titlebar/validation/`
- [ ] T100 Reconcile shell, platform command, asset, reopen, and startup-error acceptance evidence in `specs/001-custom-titlebar/validation/`

## Dependencies and Delivery

Foundation precedes US1. US2 and US3 depend on the application shell but are otherwise independently testable. The MVP is US1; the synchronized feature includes all three stories. Platform-native visual certification remains an external validation activity and is not represented as an incomplete source task.
