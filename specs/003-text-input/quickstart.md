# Quickstart Validation: Reusable TextInput Widget

## Prerequisites

- Implement the Zit root workspace with crates/zit and crates/widget before running these commands.
- Verify the zit executable entry point is crates/zit/src/main.rs and the application includes gix.


- Rust toolchain declared by the repository's `rust-toolchain.toml`.
- Desktop build prerequisites required by the official GPUI revision for the target operating
  system.
- A graphical desktop session for manual pointer, clipboard, focus, and IME validation.

## Verify workspace and dependency provenance

From the repository root:

```powershell
cargo metadata --no-deps
cargo tree -p widget -i gpui
```

Expected: `widget` is a workspace crate and its `gpui` package resolves from the official
`https://github.com/zed-industries/zed` Git source. No alternate GPUI package or patch source is
present.

## Run automated validation

```powershell
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

Expected: formatting and lint checks pass; tests cover editing transitions, grapheme-safe
boundaries, selection direction, single-line normalization, undo/redo, disabled behavior, change
events, UTF-8/UTF-16 conversion, IME composition, focus, actions, pointer hit testing, clipboard,
rendering state, minimum-height enforcement, and immediate caret visibility after pointer focus.

## Run the interactive example

```powershell
cargo run -p widget --example text_input
```

Use the example to validate the acceptance stories and the
[public contract](contracts/text-input.md):

1. Focus the input by keyboard and mouse; verify focused and unfocused visuals.
2. Type at the start, middle, and end; use Backspace and Delete at ordinary and boundary positions.
3. Navigate by character, word, start, and end; extend and contract selections with keyboard
   modifiers.
4. Click between glyphs and drag inside and outside the field; verify cursor/selection geometry.
5. Replace selected text and exercise copy, cut, paste, undo, and redo using platform shortcuts.
6. Enter a value wider than the field and verify the active cursor remains visible.
7. Toggle disabled state and confirm all mutating keyboard, clipboard, pointer, and IME operations
   are rejected while the value remains readable.
8. Clear the value and verify placeholder text is visible but cannot be selected or copied.
9. Use Korean, Japanese, or Chinese IME composition: update, commit, and cancel marked text in the
   middle of an existing value; verify provisional updates do not notify the host and commit does.
10. Paste and compose text containing line breaks; verify the result remains one visual line.
11. Leave a focused input idle and verify its caret alternates visibility every 500 ms; type, move,
    select, click, undo, redo, paste, and compose text and verify each interaction immediately shows
    the caret and restarts the cycle.
12. Verify blinking stops on blur, disablement, and non-empty selection, then restarts visibly when
    the input returns to a focused, enabled caret-only state.
13. Focus two inputs in succession and verify their blink state is independent and no stale timer
    toggles the currently active caret.
14. Click an unfocused input and verify the caret is present in the first focused frame at the
    clicked text boundary, without waiting for the 500 ms blink interval.
15. Apply a height smaller than the current text line at several text sizes and verify the input
    retains one complete line plus its normal vertical spacing without clipping text, selection,
    composition, or caret visuals.

## Reuse validation

Render two independently configured instances in the same example view with different initial
values, placeholders, enabled states, and change observers. Editing one instance MUST NOT alter the
other, and neither integration may require changes to `TextInput` internals.

## Performance and visual acceptance

- Confirm visible interaction feedback remains below the specification's 100 ms threshold under
  normal desktop use.
- Review focused, unfocused, disabled, cursor, selection, placeholder, and composition-mark states
  at supported display scales and themes.
- Use a controllable test executor to advance time around 500 ms boundaries and verify visibility,
  generation invalidation, reset behavior, and the absence of value-change events from blinking.
- Capture the first render after pointer-acquired focus and verify the caret is already visible;
  then advance the controllable clock to confirm the ordinary blink cycle follows.
- Exercise constrained heights with multiple inherited text sizes and verify the resulting bounds
  satisfy the one-line minimum without vertical clipping.
- Record any unavailable platform/IME combination as an explicit validation gap before release.

## Historical Source-Project Validation (2026-09-04)

These imported results do not describe Zit. Zit validation has not run; the workspace and
component implementation are absent at migration time (2026-09-05).

- `cargo metadata --no-deps`: PASS; `widget` is a workspace member.
- `cargo tree -p widget -i gpui`: PASS; GPUI resolves from the official Zed Git repository at
  commit `81da416e` through `Cargo.lock`.
- `cargo fmt --all --check`: PASS after formatting.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: PASS.
- `cargo test --workspace --all-features`: PASS with 23 automated tests.
- Manual GUI checks on Windows, macOS, X11, Wayland, and Korean/Japanese/Chinese system IMEs remain
  environment-dependent release checks; they cannot be completed in the current headless session.

