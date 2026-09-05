# Implementation Plan: Component Tab Bar

**Branch**: `002-component-tab-bar` | **Date**: 2026-09-05 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `specs/002-component-tab-bar/spec.md`

## Summary

Add a reusable GPUI-native `TabBar` entity to `widget`. It owns stable tabs and page entities,
renders SVG/name/X content with `text_xs`, selects one page, applies deterministic closure, and
exposes typed events. A tracked horizontal `ScrollHandle` maps wheel input, clamps overflow, and
reveals the active tab.

## Technical Context

**Language/Version**: Rust 2024, toolchain 1.97.1.
**Primary Dependencies**: Existing official-Zed Git `gpui`; `unicode-segmentation` for safe labels.
No Zed `ui` crate dependency.
**Storage**: Process-local tab order, page entities, focus handles, active identity, and scroll state.
**Testing**: `cargo test -p widget`, GPUI `test-support`, workspace format/check/Clippy.
**Target Platform**: Desktop Windows, Linux, and macOS through GPUI.
**Project Type**: Reusable UI library in the existing Cargo workspace.
**Performance Goals**: Selection, close, hover, and scroll feedback within 100 ms; smooth 30-tab use.
**Constraints**: One visible page; stable identity; `text_xs`; SVG and X controls; bounded horizontal
overflow; keyboard/accessibility parity; no Git work.
**Scale/Scope**: One module, example, and focused tests; reordering, pinning, dirty state, and persistence excluded.

## Constitution Check

*GATE: Passed before and after design.*

| Principle | Decision | Result |
|---|---|---|
| I. GPUI and Git Backend | Use official workspace GPUI and its entity, event, rendering, scroll, focus, and accessibility facilities. No Git operation. | PASS |
| II. English Specifications | All artifacts are English. | PASS |
| III. Rust Workspace | Implement in `crates/widget`; retain `crates/zit/src/main.rs` as the executable. | PASS |

No constitutional exception or additional framework is required.

## Project Structure

### Documentation (this feature)

```text
specs/002-component-tab-bar/
    spec.md
    plan.md
    research.md
    data-model.md
    quickstart.md
    contracts/tab-bar-api.md
    checklists/requirements.md
```

### Source Code (repository root)

```text
crates/widget/
    src/lib.rs
    src/tab_bar.rs
    examples/tab_bar.rs
    assets/tab-bar/close.svg
    assets/tab-bar/fallback-project.svg
    tests/tab_bar_contract.rs
    tests/tab_bar_navigation.rs
    tests/tab_bar_scroll.rs
    tests/tab_bar_accessibility.rs
```

**Structure Decision**: Keep state, rendering, input, and events in `tab_bar.rs` as one reusable
entity and export host-facing types from `lib.rs`. The host asset source resolves project SVGs;
the component provides close and fallback assets.

## Implementation Sequence and Gates

1. Implement pure order, selection, closure, and identity validation transitions.
2. Define the public API and typed selection/close events, including host close veto.
3. Render the tab list, SVG/name/X, `text_xs`, states, and one active page.
4. Add horizontal overflow, wheel mapping, clamping, resize handling, and active reveal.
5. Add focus navigation and TabList/Tab/Button semantics plus accessible actions.
6. Add example and contract/navigation/scroll/accessibility tests; run workspace checks.

Pages must not be recreated during selection. Close propagation is verified before visual polish.
Scroll input is consumed only when the horizontal offset can change.

## Complexity Tracking

No constitution violations require justification.
