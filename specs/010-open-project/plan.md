# Implementation Plan: Open Project

**Branch**: `[010-open-project]` | **Date**: 2026-09-07 | **Spec**: [spec.md](spec.md)

## Summary

Enable File > Open ... with a GPUI modal for project name and Git repository folder selection. The existing project store will validate, deduplicate, persist, and activate the new project; existing tab/store events will update the shell.

## Technical Context

**Language/Version**: Rust 2024 workspace
**Primary Dependencies**: GPUI, gix, widget TextInput, rfd native folder dialog
**Storage**: Existing project session file
**Testing**: cargo check and cargo test menu/store contracts
**Target Platform**: Windows, Linux, and macOS desktop
**Project Type**: Desktop application
**Performance Goals**: Responsive dialog; validation completes before confirmation returns
**Constraints**: GPUI and gix remain foundational; no repository creation or cloning
**Scale/Scope**: One modal flow and one project-store insertion path

## Constitution Check

- Principle I: PASS. GPUI remains the UI framework and gix the Git layer.
- Principle II: PASS. All design artifacts are in English.
- Principle III: PASS. Changes remain under `crates/zit` in the existing workspace.
- Git CLI exception: NOT REQUIRED.

## Project Structure

```text
crates/zit/src/menu.rs
crates/zit/src/platform.rs
crates/zit/src/shell.rs
crates/zit/src/project/store.rs
crates/zit/tests/menu_contract.rs
crates/zit/tests/project_store_contract.rs
specs/010-open-project/
```

**Structure Decision**: Extend the existing shell and project-store boundaries; no new crate is needed.

## Design Phases

Phase 0 resolves native folder selection and existing store behavior in [research.md](research.md).

Phase 1 defines state in [data-model.md](data-model.md), UI behavior in [contracts/open-project-ui.md](contracts/open-project-ui.md), and checks in [quickstart.md](quickstart.md).

## Complexity Tracking

No constitution violations or exceptional complexity are introduced.
