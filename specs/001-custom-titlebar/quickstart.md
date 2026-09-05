# Quickstart Validation: Zit Application Shell

This guide describes future validation after implementation. No Cargo workspace exists yet;
commands below are not claims of successful execution.

## Prerequisites

- Implement the workspace described in plan.md and pin the compatible Rust toolchain and Cargo.lock.
- Install the native build dependencies for the selected official GPUI revision on each target.
- Provide graphical Windows, macOS, Linux X11, and Linux Wayland environments from research.md.
- Complete the native capability spike and capture/reference manifest before visual acceptance.
- Record unavailable runners, native screen readers, or display scales as explicit validation gaps.

## Workspace and Source Verification

From the repository root:

```powershell
cargo metadata --no-deps --format-version 1
cargo tree -p zit -i gpui
cargo tree -p zit -i gix
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo run -p zit
```

Expected: zit and component are workspace members under crates/. zit has a binary target at
crates/zit/src/main.rs. GPUI resolves from the official Zed Git source; gix is included.
The zit executable starts directly through `gpui_platform::application()`. Any target-native
adapter is confined to the GPUI-owned window and has no alternate renderer or shell crate.

## Automated Behavior

- Verify exact menu labels/order, disabled repository commands, one-open-menu invariant,
  keyboard navigation, Escape focus restoration, and cancel/outside-click/blur behavior.
- Simulate external window state changes and verify maximize/restore command and icon mapping.
- Assert command dispatch is rejected for unavailable capabilities and disabled menu items.
- Inspect accessible roles, names, disabled/expanded states, and semantic activation.
- Inject window-open failure into the startup boundary; assert visible error dispatch and failure exit.
- Use process-level validation for Exit rather than quitting the unit-test process.

## Native Capability Spike (Before Full UI Work)

On each platform prove custom controls with no native duplicates, move/resize, minimize/reactivate,
maximize/restore or fullscreen round trip, and state updates from system shortcuts. On macOS
prove the bounded adapter hides traffic lights while preserving resize/minimize and that they
stay hidden after fullscreen. Check startup error visibility when no GPUI window exists.
Any failure blocks the relevant platform; a mock alone cannot close this gate.

## End-to-End Acceptance

1. Launch and close Zit 10 times per reference platform. Every launch shows one usable window
   with File/Help replacing visible title text and one set of SVG window controls (SC-001).
2. Execute Story 1 and Story 2, including external state changes and title-bar double-click;
   check actual OS state rather than the displayed icon alone (SC-002).
3. Open File and Help with pointer and keyboard; compare all five labels. Repository entries
   stay disabled. Escape/outside click/blur closes popups without side effects (SC-007).
   Verify GPUI `text_xs` on the menu bar and popup, then hover File, Help, and all five submenu
   rows in light and dark appearance. Each row changes background and shows a hand pointer (SC-009).
4. Open About, verify Zit and package version, dismiss and inspect focus restoration. Run Exit
   separately and verify process termination, including after macOS last-window closure.
5. Compare each control with the frozen native screenshots across all listed states (SC-003).
   Record each FR-004 attribute; permit only antialiasing differences, not different geometry/colors.
6. Test minimum 480 x 320 logical size and each supported scale. Check at least 64 logical pixels
   of empty draggable region, popup bounds, all hit areas, and cross-monitor scale changes (SC-004/008).
7. Use Narrator on Windows, VoiceOver on macOS, and Orca on Linux. Identify and activate every
   enabled control and menu item. Confirm disabled states and About focus containment (SC-005).
8. Exercise press/release-outside, unsupported capabilities, and startup failure (SC-006).

## Evidence Record

For every run record commit, Cargo.lock GPUI revision, Rust version, OS build/session, display
scale, appearance, scenario, expected/actual result, and screenshot or log path. Store new results
separately from imported component evidence. Mark absent native runs NOT RUN, never PASS.
Use the [UI contract](contracts/application-shell.md) for requirement coverage and
[research](research.md) for the outstanding native-reference planning gate.


