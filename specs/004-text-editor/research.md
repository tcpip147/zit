# Phase 0 Research: Reusable TextEditor Widget

**Migration note (2026-09-05)**: Imported research from the source project. Dependency versions,
revision-specific API observations, and implementation evidence must be revalidated in Zit.


## Independent Widget

**Decision**: Implement a new GPUI entity/custom element in `crates/widget/src/text_editor.rs`,
reusing behavioral conventions but not the `TextInput` public type.

**Rationale**: Visual-line layout, wrapping, vertical navigation, and two-axis viewport invariants
are materially different. Independence avoids regressions in the single-line widget.

**Alternatives considered**: A multi-line flag on `TextInput` would make most paths conditional;
Zed's full editor imports product-scale concerns; a new crate is unnecessary for one widget.

## Text Coordinates

**Decision**: Store UTF-8 text and normalized byte offsets for cursor, selection, lines, marked
ranges, and edits. Convert only at GPUI's UTF-16 input boundary; use `unicode-segmentation` for
user-perceived character movement/deletion.

**Rationale**: This matches Rust strings and the proven `TextInput` integration while preventing
broken graphemes. Absolute offsets make edits and IME ranges unambiguous.

**Alternatives considered**: UTF-16 storage causes pervasive conversion; scalar navigation splits
graphemes; authoritative row/column positions complicate mutation.

## Logical and Visual Lines

**Decision**: Maintain logical-line start offsets. Shape/cache visible lines plus overscan as visual
fragments containing document range, origin, dimensions, and GPUI layout. No-wrap maps one logical
line to one visual line; wrap mode produces multiple fragments.

**Rationale**: Line lookup stays predictable and routine rendering avoids shaping the whole
document. Visual fragments are the shared unit for paint, hit testing, Home/End, Up/Down, page
movement, selection geometry, and IME bounds.

**Alternatives considered**: Whole-document shaping does not scale; using logical lines for wrapped
navigation disagrees with the display. A rope is deferred until measurement justifies the added
dependency and offset complexity.

## Canonical Geometry and Scrolling

**Decision**: Centralize conversions among document offset, logical row, visual fragment, and
viewport point. Keep clamped two-axis scroll offset and measured extents in transient editor state;
handle GPUI wheel events, suppress horizontal offset in wrap mode, and minimally reveal the active
cursor/composition after relevant transitions.

**Rationale**: A single mapping prevents paint and interaction drift at wrap/newline boundaries.
Editor-owned state lets Page Up/Down and cursor reveal share viewport metrics. GPUI `ScrollHandle`
may be used internally if useful but is not public contract.

**Alternatives considered**: Independent geometry per handler is error-prone; a parent-only scroller
cannot guarantee cursor visibility; stored pixel cursor coordinates become invalid after reflow.

## Editing, IME, and History

**Decision**: Start with snapshot undo/redo entries holding committed text, selection direction,
and preferred horizontal position. Each completed mutation creates at most one entry/event.
Composition updates remain provisional; only commit enters history. New edits clear redo.

**Rationale**: This extends a working local pattern and prioritizes exact, testable restoration.
Viewport is recalculated to reveal restored selection rather than stored as document history.

**Alternatives considered**: Delta history and typing coalescing can follow profiling; composition
updates must not enter history; persistence is outside scope.

## Capability States

**Decision**: Model `enabled` and `read_only` separately. Read-only permits focus, navigation,
selection, copy, and scroll but rejects all mutations. Disabled rejects normal focus and all editor
interaction. Either transition safely cancels an active composition when mutation becomes illegal.

**Rationale**: This directly enforces the specification and allows centralized permission checks.

**Alternatives considered**: Treating read-only as disabled prevents useful selection/copy;
scattered permission checks risk bypass through IME or clipboard paths.

## GPUI Integration and Verification

**Decision**: Use locked official GPUI facilities already proven locally: `Focusable`,
`EntityInputHandler`, `ElementInputHandler`, shaped/wrapped lines, custom `Element` phases,
clipboard, actions/key contexts, and GPUI test contexts. Separate state assertions from rendered
interaction tests, then run workspace format/test/lint and the example.

**Rationale**: Platform input, focus, shaping, and dispatch remain within GPUI and meet the
constitution. Layered tests localize failures.

**Alternatives considered**: A second UI/shaping framework violates project direction; example-only
validation cannot reliably check edge cases or event cardinality.

## Clarification Resolution

No `NEEDS CLARIFICATION` remains. Host-platform conventions resolve unspecified commands; wrapping
defaults enabled; read-only stays selectable/copyable; disabled rejects normal interaction.

