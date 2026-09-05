# Implementation Plan: Custom Title Bar and Menu Bar

**Branch**: `master` | **Date**: 2026-09-05 | **Spec**: [spec.md](spec.md)

**Input**: `specs/001-custom-titlebar/spec.md`

The setup resolver returned the feature identifier `001-custom-titlebar`; the actual Git branch
is `master`. This planning operation does not create or switch branches.

## Summary

Create a Rust Cargo workspace whose `zit` executable directly starts through
`gpui_platform::application()` and contains
GPUI-native window chrome, platform SVG controls, File/Help menus, and an About dialog.
The three repository commands remain disabled. Exit quits the application; Close closes only
the current window according to platform lifecycle. No repository operation is implemented.

Use GPUI entities, actions, focus, rendering, window state, and platform facilities. Keep native
window operations distinct from visual control state. Render SVG assets through GPUI and use
one menu state machine to prevent drag, menu, and window-control input from conflicting.

## Technical Context

**Language/Version**: Rust, edition 2024. Initial toolchain candidate 1.97.1, as observed in
upstream rust-toolchain.toml during research; verify against the revision resolved into Cargo.lock.

**Primary Dependencies**: `gpui = { git = "https://github.com/zed-industries/zed" }` and
`gpui_platform` from the same official Git repository, locked at `5a9b9558`; `gix` 0.87.1.
Windows-only dependencies are `raw-window-handle` 0.6 and `windows` 0.62 with
`Win32_UI_WindowsAndMessaging`, used solely to restore an already-maximized GPUI window.
No alternate UI framework or GPUI patch source is used.

**Storage**: In-memory window, menu, and focus state; bundled SVG assets. No persistent user data.

**Testing**: Rust state-machine and command-boundary tests currently pass. Workspace formatting,
Clippy, metadata, and tests pass as recorded in `validation/build.md`. Native desktop interaction,
accessibility inspection, and reference-image comparison remain pending where recorded.

**Target Platform**: Windows 11 24H2, macOS 15.6, Ubuntu 24.04 with GNOME 46/Adwaita;
Linux Wayland and X11 sessions are separate validation rows. These are chosen reference targets,
not claims about currently installed environments. See research.md for reference-capture gates.

**Project Type**: Desktop Git client shell.

**Performance Goals**: Design target: menu and hover feedback within 100 ms on reference hardware;
no polling loop or background Git work while idle. Mandatory acceptance metrics remain SC-001–008.

**Constraints**: All 20 functional requirements; Constitution v1.2.0; custom SVG controls on all
three platforms; no duplicated native controls; accessible menus; no dependency on unimplemented
TextInput/TextEditor components. Initial window 1024 x 720 logical pixels; minimum 480 x 320.
Reserve at least 64 logical pixels of draggable title-bar space at minimum width. Derive native
button dimensions from reference profiles rather than stretching them to fill the title bar.

**Scale/Scope**: One primary window, two menu headings, five submenu items, one About dialog,
three platform asset profiles, inside the zit crate, alongside the existing component workspace crate.

## Constitution Check

| Principle | Pre-research decision | Post-design result |
|---|---|---|
| I: GPUI foundation | Official Zed Git source; GPUI-native UI and platform operations | Design PASS; runtime compatibility must be verified |
| I: gix backend | Include gix in zit; no Git CLI or repository operations in this scope | Design PASS |
| II: English specifications | All planning documents and specification prose are English | PASS |
| III: Rust workspace | Root Cargo.toml; crates/zit and crates/widget; zit/src/main.rs executable | Design PASS |

No architectural exceptions are requested. Native compatibility and visual evidence gates below
remain open; design compliance is not proof of implementation feasibility on every platform.

## Project Structure

### Documentation (this feature)

```text
specs/001-custom-titlebar/
    spec.md
    plan.md
    research.md
    data-model.md
    quickstart.md
    contracts/application-shell.md
    checklists/requirements.md
```

`tasks.md` belongs to the subsequent task-generation phase and is not created here.

### Source Code (repository root)

```text
Cargo.toml
Cargo.lock
rust-toolchain.toml
crates/
    zit/
        Cargo.toml
        src/
            main.rs
            shell.rs
            titlebar.rs
            menu.rs
            about.rs
            platform.rs
            assets.rs
            startup_error.rs
        assets/window-controls/windows/
        assets/window-controls/linux/
        assets/window-controls/macos/
        tests/menu_contract.rs
        tests/window_contract.rs
        tests/accessibility.rs
    component/                 # Existing reusable widget library; unchanged by this feature
```

**Structure Decision**: zit owns startup, application lifecycle, version information, and gix.
The `zit` crate owns application startup, lifecycle, title-bar rendering, menus, and platform window operations.
Platform-specific code stays behind platform.rs using GPUI operations, a bounded Windows
`SW_RESTORE` adapter, and a bounded macOS AppKit adapter if native probing proves one necessary;
see research.md. The existing
`crates/widget` workspace member remains independent and is not a prerequisite of this
feature. The `zit` executable does not introduce a shell library boundary.

### Implementation Sequence and Gates

1. Establish manifests, official dependency resolution, and the zit main entry point. Record the
   GPUI commit, compatible toolchain, and actual platform build prerequisites.
2. Before full UI construction, prove custom chrome on each platform: hidden native controls,
   resize/drag, full screen, state observation, accessibility, and error presentation. Record any
   missing GPUI capability as a blocker; do not silently substitute native controls or another framework.
3. Capture native reference profiles defined in research.md. Lock screenshot metadata and hashes
   before authoring final SVG geometry. Visual parity cannot be certified without these captures.
4. Implement shell, authoritative window-state mapping, SVG assets, explicit control clicks,
   and Windows restore through the GPUI raw window handle where `zoom_window()` cannot restore.
5. Implement File/Help state, `text_xs` typography, appearance-aware lightened surfaces,
   pointer/hover feedback, keyboard navigation, disabled commands, Exit, and About.
6. Execute automated behavioral coverage and all platform scenarios from quickstart.md. Retain
   unavailable native checks as explicit release blockers rather than reporting a passing matrix.

## Complexity Tracking

No constitution violations. The single zit application crate keeps lifecycle and shell UI together; reusable controls remain in the existing component crate.
A generic UI toolkit, additional Git service crate, and external menu framework are unnecessary.

## Planning Evidence Status

The design artifacts are written. Native screenshot capture and execution-based GPUI capability
proof are not available in this planning session. The specification's requirement to freeze actual
reference screenshots during planning is therefore not yet satisfied. This is an explicit planning
validation gap; do not label the feature fully implementation-ready until the capture gate is met.


