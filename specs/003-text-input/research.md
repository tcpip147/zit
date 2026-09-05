# Phase 0 Research: Reusable TextInput Widget

**Migration note (2026-09-05)**: Imported research from the source project. Dependency versions,
revision-specific API observations, and implementation evidence must be revalidated in Zit.


## GPUI text-input integration

**Decision**: Implement the widget as a focusable GPUI entity and register it as the focused
element's input handler. Use the current `EntityInputHandler` contract for selected/marked ranges,
text replacement, range bounds, and point-to-character conversion.

**Rationale**: GPUI's official input example demonstrates this exact integration for platform text
input and IME, including marked text and UTF-16 selections. It also uses focus tracking, action
listeners, shaped text, mouse events, and custom cursor/selection painting, providing the closest
upstream compatibility reference. See the official
[GPUI input example](https://github.com/zed-industries/zed/blob/main/crates/gpui/examples/input.rs).

**Alternatives considered**: Handling only key events was rejected because it bypasses platform
text services and cannot satisfy IME requirements. A hidden native text control was rejected
because it would not be a reusable GPUI-native widget and would introduce platform-specific
behavior outside GPUI.

## Indexing and Unicode boundaries

**Decision**: Store internal text ranges as valid UTF-8 byte ranges, move/delete by extended
grapheme cluster boundaries, and convert between UTF-8 offsets and UTF-16 code-unit offsets only at
the platform input-handler boundary.

**Rationale**: Rust strings require UTF-8 boundaries, users expect visually whole characters to
move/delete together, and GPUI's input contract exchanges UTF-16 ranges with platform input
services. The upstream example explicitly performs UTF-8/UTF-16 conversion and uses grapheme
segmentation for adjacent movement and deletion.

**Alternatives considered**: Raw byte movement can corrupt or panic on Unicode. Unicode scalar
movement can split user-perceived characters. Storing UTF-16 internally would add conversions to
ordinary Rust string operations and still require grapheme-boundary logic.

## IME composition

**Decision**: Represent active composition as a marked range within the displayed working text.
Composition updates replace the previous marked range without emitting a committed-value change;
commit clears the mark and records one edit, while cancellation restores the pre-composition
snapshot.

**Rationale**: The platform must query and update selected and marked ranges while positioning its
candidate UI. GPUI exposes marked-range replacement and range-bound geometry specifically for this
flow. Separating provisional and committed observation meets FR-030 and FR-035.

**Alternatives considered**: Emitting every provisional update as a value change would expose text
the user has not committed and complicate undo. Keeping composition completely outside displayed
text would prevent correct shaping, underlining, selection geometry, and candidate positioning.

## Rendering and hit testing

**Decision**: Shape one display line through GPUI, retain the latest shaped line and viewport
bounds, and use its index/x-position mapping for hit testing, cursor placement, selection paint,
IME range bounds, and horizontal viewport adjustment. Paint placeholder, marked text, selection,
and caret as distinct visual layers governed by focus and enabled state.

**Rationale**: Shaping-based geometry keeps pointer and IME positions consistent with the actual
glyph layout. The official example uses `ShapedLine` for `x_for_index`, nearest index lookup, range
bounds, cursor, selection, and marked-text underline.

**Alternatives considered**: Estimating positions from character counts fails for proportional
fonts, ligatures, combining marks, and bidirectional text. A separate text-layout dependency would
duplicate GPUI's UI foundation.

## Commands, clipboard, and platform conventions

**Decision**: Define semantic editing actions and bind standard platform keystrokes by key context.
Use GPUI's clipboard service with plain-text clipboard items. Disabled state gates every mutating
action and input-handler replacement path.

**Rationale**: Semantic actions make bindings platform-adjustable and testable. GPUI exposes
clipboard read/write operations in its platform abstraction, while its official input example
demonstrates copy, cut, paste, and key-context dispatch. See
[GPUI platform clipboard contract](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/platform.rs)
and [GPUI key dispatch](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/key_dispatch.rs).

**Alternatives considered**: Hard-coding physical keys inside editing methods would mishandle
platform modifiers and user keymaps. Accessing operating-system clipboards directly would bypass
GPUI and multiply platform-specific code.

## Undo and redo semantics

**Decision**: Record committed value edits as before/after snapshots of text and selection.
Coalesce adjacent uninterrupted character insertion and repeated adjacent deletion into user-level
transactions; selection replacement, cut, paste, and IME commit form distinct transactions. A new
edit after undo clears redo history. Programmatic value setting resets history by default.

**Rationale**: Snapshot transactions are simple and reliable for a single ordinary text field,
restore cursor/selection exactly, and match user expectations better than undoing every character
separately. The model can later optimize storage without changing the public contract.

**Alternatives considered**: Per-keystroke history creates frustrating undo behavior. A complex
operational transform or rope history is unnecessary for this scope. Including cursor-only moves
in undo would conflict with common desktop behavior.

## Toolchain and dependency policy

**Decision**: Pin the workspace toolchain to Rust 1.95.0 initially and consume `gpui` directly from
the official Zed Git repository, using the revision resolved by the lockfile. Enable GPUI test
support only where needed for tests.

**Rationale**: The official Zed repository currently pins Rust 1.95.0, so alignment reduces
compiler incompatibility with a moving Git dependency. The current GPUI manifest identifies the
crate as version 0.2.2 and provides default desktop and `test-support` features. See Zed's
[Rust toolchain](https://github.com/zed-industries/zed/blob/main/rust-toolchain.toml) and official
[GPUI manifest](https://github.com/zed-industries/zed/blob/main/crates/gpui/Cargo.toml).

**Alternatives considered**: An arbitrary older compiler risks incompatibility. A crates.io GPUI,
fork, or unrelated implementation violates the constitution. Pinning a Git revision directly in
the manifest was not selected because the constitution mandates the source form; reproducibility
is provided by committing the generated lockfile.

## Cursor blink lifecycle

**Decision**: Use a 500 ms GPUI background-executor timer loop owned by the TextInput entity. Track
cursor visibility plus a monotonically increasing blink generation. Focus acquisition and every
editing, navigation, selection, or pointer placement interaction increment the generation, show the
cursor immediately, and start a replacement loop. Blur, disablement, or a non-empty selection stop
blinking and invalidate outstanding timer callbacks. A callback may toggle visibility only when
its captured generation still matches and the widget remains focused, enabled, and caret-only.

**Rationale**: A task owned by the GPUI entity is cancelled naturally when replaced or when the
entity is released. The generation check additionally prevents a timer that has already awakened
from toggling a newly reset cursor. Zed's own editor uses a 500 ms interval, immediate cursor
visibility on disruption, and epoch-based invalidation in its `BlinkManager`; GPUI's current view
example demonstrates an entity-owned cancellable task driven by focus and blur subscriptions.

**Alternatives considered**: An application-global timer would couple independent TextInput
instances and complicate disposal. Starting untracked loops on every input event could produce
multiple competing toggles. Rendering continuously and deriving visibility from wall-clock time
would waste redraw work and make deterministic tests harder.

## Immediate caret on pointer-acquired focus

**Decision**: Treat the primary-pointer focus path as a synchronous caret-visibility transition.
After requesting focus, the same interaction update records focused state, places the caret, makes
it visible, and starts a fresh blink generation. The ordinary focus observer remains an idempotent
fallback for keyboard and host-driven focus changes.

**Rationale**: Focus observers may be delivered after the pointer handler has completed. If caret
eligibility depends only on that deferred notification, the first focused frame can omit the caret
or wait until later work. Synchronously establishing the visible phase satisfies FR-039 while the
generation guard prevents a later focus notification from allowing an older timer to toggle state.

**Alternatives considered**: Waiting for the focus observer reproduces the visible delay. Waiting
for the first 500 ms tick is contrary to the required initial visible phase. Painting a caret
unconditionally whenever focus is queried would bypass the blink state and make reset behavior
harder to test consistently.

## Minimum control height

**Decision**: Derive the control's minimum height from the active single-line text metrics plus its
normal vertical padding and boundary thickness. Apply a corresponding one-line minimum to the
inner text layout so neither parent constraints nor flex shrinking can collapse the glyph area.

**Rationale**: Text size and inherited line metrics may vary between host applications. A minimum
based on current metrics preserves a complete line, selection background, and caret without
forcing hosts to duplicate widget-specific spacing calculations.

**Alternatives considered**: A fixed total height fails for larger fonts. Relying only on natural
content size allows explicit parent constraints to clip the field. Removing padding at small
heights would make the widget's visual and pointer geometry inconsistent across integrations.

