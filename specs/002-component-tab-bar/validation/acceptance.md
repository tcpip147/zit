# Acceptance Validation

**Date**: 2026-09-05

## Automated evidence

| Requirement | Result | Evidence |
|---|---|---|
| FR-001-FR-007, FR-020 | PASS | `tab_bar_contract` verifies order, fallback selection, retained pages, idempotent selection, duplicate rejection, veto, and right-then-left closure. |
| FR-009-FR-015 | PASS (structural) | The render tree uses fixed 180 px tabs, fixed icon/close regions, `text_xs`, bounded tracked horizontal overflow, active/hover/focus colors, and pointer cursors. |
| FR-016 | PASS | `tab_bar_accessibility` dispatches focus navigation, selection, and close actions without pointer input. |
| FR-017 | PASS (structural) | Rendered nodes expose TabList, Tab, and Button roles with full labels, positions, counts, and selected state. GPUI maps `on_click` to accessible Click. |
| FR-018 | PARTIAL | Empty icon paths use the bundled fallback. Native rendering of a nonexistent host asset has not been validated. |
| FR-019 | PASS (structural) | GPUI owns overflow measurement and the tracked handle clamps offsets when geometry changes. |

## Native checks

The Windows graphical example, mouse-wheel/trackpad boundary behavior, pointer propagation,
accessibility-tree inspection, light/dark appearance, display scales, and sub-100-ms measurement are
**NOT RUN** in this non-interactive implementation session. SC-001 through SC-008 therefore remain
pending native acceptance even though their implementation paths compile and the deterministic state
tests pass. Linux and macOS are also **NOT RUN**.

Run `cargo run -p widget --example tab_bar` and follow `quickstart.md` to close these evidence gaps.
