# Quickstart: Validate the Reusable Data Grid

## Prerequisites

- Rust 1.97.1 from the repository toolchain.
- Workspace dependencies already resolved.
- A graphical desktop session for interactive example checks.

## Automated validation

From the repository root, run:

```powershell
cargo test -p widget --test data_grid_contract
cargo test -p widget --test data_grid_scroll
cargo test -p widget --test data_grid_accessibility
cargo test -p widget --test data_grid_performance
cargo test -p zit --test project_page_contract
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Expected results:

- Column validation accepts valid fixed/fraction layouts and rejects invalid identities or widths.
- Rows with exact, missing, and excess cells preserve the declared column structure.
- Zero, one, 100, and 10,000-row configurations request only valid visible indexes.
- Scroll offsets and thumb positions remain bounded before and after row-count changes.
- Multiple grids keep independent scroll, drag, visibility, and hide-timer state.
- Accessibility nodes expose the visible header/row/cell relationships.
- Project history retains Graph, Commit, Author, Date, and Hash columns and its existing load states.

## Interactive widget validation

Run:

```powershell
cargo run -p widget --example data_grid
```

Verify:

1. A small grid shows its header and all rows without a functional thumb.
2. A 10,000-row grid scrolls smoothly while its header remains fixed.
3. Wheel input reveals the scrollbar, which hides about one second after activity.
4. Hover keeps the scrollbar visible.
5. Dragging the thumb reaches the first and last rows and releasing outside ends the drag.
6. Long cell content is clipped without crossing column dividers.
7. Two grids maintain different scroll positions.
8. Dragging any internal header divider resizes both adjacent columns; the pointer can leave the grid before release, and neither column becomes narrower than 40 pixels.
9. After resizing columns, resize the application window and verify that the selected column-width ratios remain unchanged.

## Project integration validation

Run Zit with a repository containing enough commits to overflow the History view:

```powershell
cargo run -p zit
```

Open the History section and verify:

1. Lazy loading, loading feedback, failure, retry, and empty-history behavior are unchanged.
2. Non-empty history uses the reusable grid.
3. Column order and visible commit values match the prior project-specific table.
4. Wheel and thumb scrolling make every loaded commit reachable.
5. Switching tabs preserves each project page and does not leak grid state between projects.

## Structural validation

Confirm the generic module has no project-domain dependencies:

```powershell
rg -n "Project|Repository|Commit" crates/widget/src/data-grid.rs
```

Expected result: no matches other than documentation explicitly describing prohibited coupling, preferably no matches at all.

## Implementation validation record

Validated on Windows in the repository workspace on 2026-09-06:

- `cargo fmt --all -- --check`: passed after formatting.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: passed.
- `cargo test --workspace`: passed, including DataGrid contract, accessibility, scroll geometry, 10,000-row construction, and existing project-page integration suites.
- `cargo check --workspace`: passed.
- Structural review confirmed the widget uses GPUI only, contains no project/Git domain references, remains in the `widget` workspace crate, and leaves `crates/zit/src/main.rs` as the application entry point.

The graphical scenarios remain available through `cargo run -p widget --example data_grid`; the automated run environment did not launch an interactive desktop window.
