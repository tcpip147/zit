# Quickstart Validation: Reusable TextEditor Widget

## Prerequisites

- Implement the Zit root workspace with crates/zit and crates/widget before running these commands.
- Verify the zit executable entry point is crates/zit/src/main.rs and the application includes gix.
 and Commands

Use Rust 1.95.0 and the platform prerequisites for the locked GPUI revision. From repository root:

```powershell
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo run -p widget --example text_editor
```

All checks must pass, existing `TextInput` tests must remain green, and the example must show at
least two independent editors: editable/wrappable/scrollable and state-switchable.

## Validation Scenarios

### Editing and History

Create three lines including a blank line; edit in the middle; replace a cross-line selection; undo
and redo; then branch history with a new edit. Expect intact line breaks/graphemes, restored
selection, discarded superseded redo, and one event per actual mutation.

### Navigation and Selection

Exercise every specified movement with and without Shift on short, long, blank, and wrapped lines.
Click, drag beyond the viewport, select a word, and Select All. Expect valid grapheme boundaries,
preferred horizontal position across vertical moves, visual-line wrap behavior, and autoscroll.

### Clipboard and IME

Copy/cut/paste across lines. Start, revise, cancel, and commit Korean/Japanese/Chinese composition in
mid-line and over selection near viewport edges. Expect preserved line breaks, distinct provisional
text, cancellation without mutation, exactly-once commit, and automatic reveal.

### Wrap and Scroll

Use content taller and wider than the viewport. Resize under wrap, then disable wrap and scroll both
axes to extremes. Move the cursor off-screen. Expect unchanged value, no horizontal overflow while
wrapped, clamped offsets, and minimal cursor reveal.

### Placeholder and Capability States

Verify placeholder on empty content. In read-only mode, try selection/copy and every mutation. In
disabled mode, try focus, pointer, keys, clipboard, and scroll. Expect placeholder excluded from
value/selection, read-only non-mutating interaction only, disabled rejection, and no false events.

### Scale and Regression

Repeat representative interaction with 10,000 lines and 1,000,000 characters, including combining
characters, emoji, Korean, and mixed-direction text. Expect feedback within the 100 ms acceptance
target, no broken graphemes, and unchanged `TextInput` behavior.

