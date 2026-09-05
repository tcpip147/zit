# Implementation Plan: Reusable TextInput Widget

**Branch**: `001-text-input` | **Date**: 2026-09-04 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/component/001-text-input/spec.md`

**Note**: This plan ends after Phase 1 design. Implementation tasks are generated separately.

## Summary

Create or extend the Zit Cargo workspace and a reusable `TextInput` in the reusable `widget`
crate. The widget will use GPUI's focus, action dispatch, clipboard, text shaping, painting, and
`EntityInputHandler` facilities to provide single-line editing, Unicode-safe navigation and
deletion, pointer selection, IME composition, undo/redo, disabled behavior, placeholder display,
host-observable value changes, and a blinking one-pixel caret that inherits the input text color.
Caret blinking will be driven by a cancellable GPUI task tied to focus and caret eligibility, with
an epoch/generation guard preventing stale timers from changing current state. Pointer-acquired
focus will synchronously enter the visible caret phase in the same interaction update, rather than
waiting for deferred focus observation or the first blink interval. The rendered control will also
enforce a minimum height equal to one text line plus its vertical chrome so external constraints
cannot clip text, selection, or caret rendering. Pure editing
transitions will be separated from GPUI event and render integration so most behavior can be tested
deterministically, with GPUI window tests and a small example validating integration and visual
interaction.

## Technical Context

**Language/Version**: Rust 1.95.0, aligned with the official Zed repository toolchain at planning
time

**Primary Dependencies**: `gpui` from `https://github.com/zed-industries/zed`; GPUI default
desktop features; `unicode-segmentation` for extended grapheme boundaries

**Storage**: In-memory widget value, selection, composition, and bounded-lifetime edit history;
no persistent storage

**Testing**: Rust unit tests via `cargo test`; GPUI test support for focus, actions, input-handler,
pointer, clipboard, constrained-height layout, immediate click-focus caret visibility, and
rendering integration; runnable example for manual cross-platform checks

**Target Platform**: Desktop platforms supported by the selected official GPUI revision, with
Windows, macOS, Linux X11, and Linux Wayland behavior validated where runners are available

**Project Type**: Reusable Rust UI component library in a Cargo workspace

**Performance Goals**: Visible response to normal focus, cursor, selection, and edit interactions
within 100 ms; cursor visibility toggles at a stable 500 ms interval while eligible; linear-time
edits and shaping are acceptable for ordinary single-line field values

**Constraints**: Official GPUI Git dependency only; single-line plain text; user-perceived
character-safe navigation/deletion; UTF-16 conversion at the platform input boundary; one active
blink loop per widget; stale blink timers cannot change current state; pointer focus must expose a
visible caret in its first resulting frame; widget height cannot fall below one line plus vertical
chrome; no application-specific business logic

**Scale/Scope**: One reusable `widget` crate, one public TextInput component, one example, and
tests covering 39 functional requirements across ordinary desktop-sized values

## Constitution Check

Design review against Zit Constitution v1.2.0 (2026-09-05):

| Rule | Planned compliance | Status |
|---|---|---|
| GPUI foundation | Use `gpui = { git = "https://github.com/zed-industries/zed" }` and GPUI-native facilities; no substitute | Design compliant; implementation unverified |
| Git backend | Include gix in the application dependency layer; generic widgets do not perform Git operations | Design compliant; implementation unverified |
| English specifications | Maintain specification prose in English | Reviewed |
| Rust workspace | Root Cargo.toml includes crates/zit and crates/widget | Design compliant; implementation unverified |
| Application entry point | Package and executable named zit at crates/zit/src/main.rs; widget is an additional library | Design compliant; implementation unverified |

Imported toolchain versions, dependency revisions, and API research are historical candidates.
Revalidate them against the actual Zit dependency resolution before implementation. No Cargo
workspace or implementation is present at migration time; prior project PASS results are not Zit evidence.

## Project Structure

### Documentation (this feature)

```text
specs/component/001-text-input/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── text-input.md
└── tasks.md                 # Created later by $speckit-tasks
```

### Source Code (repository root)

The root Cargo.toml includes crates/zit and crates/widget. The application package is named zit,
with its manifest at crates/zit/Cargo.toml and entry point at crates/zit/src/main.rs.
The widget library does not replace the application. Shared setup must reuse existing files.


```text
Cargo.toml                   # Workspace and official GPUI dependency
rust-toolchain.toml          # Toolchain aligned with the selected GPUI revision
crates/
├── zit/
│   ├── Cargo.toml           # Primary application; includes gix
│   └── src/main.rs          # Application entry point
└── widget/
    ├── Cargo.toml
    ├── src/
    │   ├── lib.rs           # Public exports and action registration entry point
    │   └── text_input.rs    # TextInput state, editing, GPUI input, and rendering
    ├── tests/
    │   └── text_input_*.rs  # Focused public-contract and GPUI interaction suites
    └── examples/
        └── text_input.rs    # Manual end-to-end validation host
```

**Structure Decision**: Use the root Cargo workspace with `crates/zit` as the primary application and
`crates/widget` as an additional reusable library crate. Keep the first component cohesive in one module until
separate internal modules are justified by implementation size. Public-contract integration tests
and a runnable example live with the crate that owns the component.

## Complexity Tracking

No constitution violations or exceptional complexity are planned.


