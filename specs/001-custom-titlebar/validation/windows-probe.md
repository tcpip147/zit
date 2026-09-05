# Windows Native Capability Probe

**Date**: 2026-09-05
**Status**: PARTIAL — probe launch verified; interactive scenarios NOT RUN
**Platform**: Windows x86_64 MSVC host
**GPUI revision**: official Zed commit `5a9b9558`
**Rust**: 1.97.1

## Completed Evidence

- `cargo check -p zit --all-targets --locked`: PASS.
- `cargo test -p zit --locked`: PASS (one foundational command-state test).
- `cargo run -p zit --example chrome_probe --locked`: the process built and entered the GPUI
  event loop successfully. It was stopped with Ctrl+C after the automation connection failed.
- The probe declares `WindowControlArea::Drag`, `Min`, `Max`, and `Close` hit-test regions and
  uses a decoration-free GPUI window with a 480 x 320 logical minimum size.

## Not Yet Verified

- Visual absence of duplicate native controls.
- Pointer drag and edge/corner resize behavior.
- Native minimize, maximize/restore, and close behavior through the declared hit-test regions.
- State synchronization after operating-system shortcuts.
- Accessibility exposure and window-independent startup error presentation.

The Computer Use native pipe was unavailable (`os error 2`), so no screenshot or UI action
evidence was captured. None of the scenarios above is marked PASS. T006 remains incomplete.
