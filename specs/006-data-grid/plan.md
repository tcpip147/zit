# Implementation Plan: Reusable Data Grid Widget

**Branch**: `[006-data-grid]` | **Date**: 2026-09-06 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/006-data-grid/spec.md`

## Summary

Extract the project History section's fixed-header, virtualized row table and transient draggable scrollbar into a reusable `DataGrid` entity in the `widget` crate. The widget owns column configuration, a range-friendly indexed row renderer, per-instance scroll/drag/visibility state, shared-theme styling, and accessible table semantics. `ProjectPage` remains responsible for commit formatting and loading states and supplies generic rows to the grid.

## Technical Context

**Language/Version**: Rust 1.97.1, edition 2024

**Primary Dependencies**: GPUI from the official Zed repository; existing local `widget` crate

**Storage**: N/A; row data remains owned by the host and grid state is session-memory only

**Testing**: `cargo test --workspace`; GPUI test-support contexts; a reusable widget example; project-page regression contracts

**Target Platform**: Windows, macOS, and Linux desktop targets supported by GPUI

**Project Type**: Reusable desktop UI component integrated into the native Zit application

**Performance Goals**: Correctly expose 10,000 indexed rows while rendering only the visible range; initial display and scroll feedback within 100 ms

**Constraints**: Uniform row height; fixed header; vertical scrolling only; no Git-domain dependencies; independent state per grid; one-second scrollbar hide delay; exact requested source path `crates/widget/src/data-grid.rs`

**Scale/Scope**: Zero to at least 10,000 rows, zero or more declared columns, multiple simultaneous grid instances, and one migrated project-history consumer

## Constitution Check

*GATE: Passed before Phase 0 and re-checked after Phase 1.*

- **GPUI foundation**: PASS. `DataGrid` uses GPUI entities, elements, uniform-list virtualization, scroll handles, input events, timers, and accessibility roles.
- **Git backend**: PASS. The generic widget contains no Git operations or domain types. Existing project history continues to arrive from the `gix`-based repository layer.
- **English specifications**: PASS. All specification and planning artifacts are written in English.
- **Rust workspace**: PASS. The new module remains inside the existing `crates/widget` workspace member.
- **Application entry point**: PASS. `crates/zit/src/main.rs` remains the executable entry point.
- **Post-design re-check**: PASS. The public contract, data model, theming, migration, and tests introduce no constitutional exception.

## Project Structure

### Documentation (this feature)

```text
specs/006-data-grid/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── public-api.md
└── tasks.md                 # Created by $speckit-tasks
```

### Source Code (repository root)

```text
crates/widget/
├── Cargo.toml               # Register the data_grid example
├── examples/
│   └── data_grid.rs         # Multiple grids and 10,000-row validation host
├── src/
│   ├── data-grid.rs         # DataGrid, columns, rows, cells, scrolling, rendering
│   ├── lib.rs               # Path-qualified module declaration and public exports
│   └── theme.rs             # DataGridTheme and Theme integration
└── tests/
    ├── data_grid_contract.rs
    ├── data_grid_scroll.rs
    ├── data_grid_accessibility.rs
    └── data_grid_performance.rs

crates/zit/
├── src/project/page.rs      # Project-specific row formatting and DataGrid consumer
└── tests/project_page_contract.rs
```

**Structure Decision**: Keep the generic component in `crates/widget` and the Git history adapter in `crates/zit`. Because the user explicitly requires the hyphenated filename `data-grid.rs`, `lib.rs` will use `#[path = "data-grid.rs"] mod data_grid;` while exporting conventional Rust identifiers. No new crate or dependency is required.

## Design

### Widget ownership

`DataGrid` is a retained GPUI entity. It owns validated column definitions, row count, an indexed row-renderer callback, uniform-list scroll state, thumb drag state, visibility state, hide-task generation, and `DataGridTheme`. Each instance therefore remains isolated and can be embedded as an `Entity<DataGrid>`.

### Host-provided rows

The public row renderer receives a row index and returns `DataGridRow`, whose ordered cells contain text or arbitrary GPUI elements. The uniform list invokes it only for requested indexes. `DataGrid` normalizes each row to the declared column count by padding missing cells and ignoring excess cells.

### Columns and layout

`DataGridColumn` contains stable identity, accessible label, and width policy. The initial width policy supports fixed logical pixels and proportional fractions. Header and body use one shared computed column layout so their boundaries remain aligned. Rows use one configurable uniform height; header height is separately configurable.

### Scrolling and scrollbar lifecycle

`UniformListScrollHandle` supplies measured viewport, maximum offset, and row virtualization. The grid derives proportional thumb size and position with a 24-pixel minimum. Wheel interaction and track hover reveal the scrollbar. Dragging captures window-level move/up events so release outside the grid terminates the drag. A generation-safe task hides the control one second after inactive hover, wheel, or drag completion.

### Accessibility

The outer grid uses the table role, the header uses row plus column-header roles, and visible body content uses row and cell roles. Column and row position/count metadata are supplied where GPUI/accesskit supports them. Virtualization exposes the currently rendered content without manufacturing inaccessible off-screen nodes.

### Project history migration

`ProjectPage` retains `NotLoaded`, `Loading`, `Ready`, `Failed`, retry, and empty-history behavior. On ready non-empty data, it configures or updates one retained `DataGrid` with Graph, Commit, Author, Date, and Hash columns and a renderer over an owned commit snapshot. Commit graph drawing, short-hash formatting, and timestamps remain in `crates/zit/src/project/page.rs`.

## Complexity Tracking

### Current Implementation Alignment

The implemented grid also requires contract coverage for row selection callbacks, proportional column resizing, and window-level scrollbar drag capture in addition to virtualization and fixed-header behavior.

No constitutional violations require justification. The path attribute is required solely to honor the explicitly requested hyphenated filename.
