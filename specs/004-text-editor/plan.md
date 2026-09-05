# Implementation Plan: Reusable TextEditor Widget

**Branch**: `002-text-editor` | **Date**: 2026-09-04 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/component/002-text-editor/spec.md`

## Summary

**Dependency**: Implement and validate the TextInput baseline in
[the TextInput plan](../001-text-input/plan.md) before starting this component.


Add an independent reusable GPUI `TextEditor` to the existing `widget` crate. It will maintain
committed text, selection, IME composition, history, and viewport state separately from transient
layout; index logical lines by UTF-8 offsets; and shape/cache the visible working set as visual
lines. One coordinate-mapping layer will serve painting, hit testing, wrapped-line navigation,
scrolling, cursor reveal, and IME bounds. Existing `TextInput` conventions are retained without
changing its public behavior.

## Technical Context

**Language/Version**: Rust 1.95.0, edition 2024

**Primary Dependencies**: GPUI from the official `zed-industries/zed` Git repository at the
workspace-locked revision; `unicode-segmentation` 1.12

**Storage**: In-memory plain text and edit history; no persistence

**Testing**: `cargo test --workspace`; GPUI `#[gpui::test]` with `TestAppContext` and
`VisualTestContext`; workspace rustfmt and clippy; example-based manual validation

**Target Platform**: Desktop platforms supported by the resolved GPUI revision

**Project Type**: Reusable desktop UI widget library in a Cargo workspace

**Performance Goals**: Visible feedback within 100 ms for documents up to 10,000 lines and
1,000,000 characters; ordinary paint/scroll work proportional to the visible working set

**Constraints**: Plain text; UTF-8 storage with grapheme-safe operations and UTF-16 IME conversion;
platform key conventions; cursor/composition visibility; wrap reflow preserves positions; no
application-specific behavior

**Scale/Scope**: One public widget, one example, one API contract, and focused integration tests

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
specs/component/002-text-editor/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── public-api.md
├── checklists/requirements.md
└── tasks.md                 # Created later by $speckit-tasks
```

### Source Code (repository root)

The root Cargo.toml includes crates/zit and crates/widget. The application package is named zit,
with its manifest at crates/zit/Cargo.toml and entry point at crates/zit/src/main.rs.
The widget library does not replace the application. Shared setup must reuse existing files.


```text
crates/widget/
├── src/
│   ├── lib.rs               # Re-exports
│   ├── text_input.rs        # Existing widget, unchanged behavior
│   └── text_editor.rs       # Model, actions, input handler, element, rendering
├── examples/
│   ├── text_input.rs
│   └── text_editor.rs
├── tests/
│   ├── text_editor_contract.rs
│   ├── text_editor_editing.rs
│   ├── text_editor_navigation.rs
│   ├── text_editor_pointer.rs
│   ├── text_editor_clipboard.rs
│   ├── text_editor_history.rs
│   ├── text_editor_ime.rs
│   ├── text_editor_scrolling.rs
│   ├── text_editor_rendering.rs
│   └── text_editor_regression.rs
└── Cargo.toml               # Registers example; no new dependency planned
```

**Structure Decision**: Extend the existing `widget` crate with an independent `text_editor`
module. Keep the initial implementation in one module to match `TextInput`; split private
model/layout modules during task generation only if clear boundaries justify it. Tests are grouped
by observable capability and exercise the public surface through GPUI test contexts.

## Complexity Tracking

No constitution violations require justification.

