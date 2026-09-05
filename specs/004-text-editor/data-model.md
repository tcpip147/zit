# Data Model: Reusable TextEditor Widget

## Core Model

Committed text and text-relative offsets are authoritative. Layout and viewport geometry are
derived and disposable after text, style, width, or wrap changes.

### TextDocument

| Field | Meaning | Invariant |
|---|---|---|
| `text` | Committed UTF-8 plain text with `\n` line breaks | Always valid UTF-8 |
| `line_starts` | Ordered logical-line start byte offsets | Begins at 0; includes final empty line after trailing newline |

Edits update affected line starts before further interaction. Each start is a valid boundary in
`0..=text.len()`.

### TextPosition and Selection

`TextPosition` is an absolute UTF-8 byte offset, clamped to the document and a character boundary;
user character operations additionally use grapheme boundaries. Logical row/column and UTF-16 IME
offsets are derived.

| Selection field | Meaning |
|---|---|
| `range` | Normalized half-open byte range |
| `reversed` | Active endpoint is `range.start` when true |
| `preferred_x` | Desired horizontal location across vertical moves |

An empty range is a caret. Crossing the anchor flips direction while keeping the range normalized.
Non-vertical movement or direct pointer placement resets `preferred_x`.

### Composition

| Field | Meaning |
|---|---|
| `original` | Pre-composition committed snapshot |
| `marked_range` | Provisional content range |
| `selected_range` | IME selection inside marked text |

```text
Inactive -> Active      capture original state
Active -> Active        replace provisional marked text; no history/event
Active -> Committed     commit once; one history entry and value event
Active -> Cancelled     restore committed state; no value event
Active -> Cancelled     read-only/disabled/external-value transition
```

### EditHistory

Snapshots contain committed text, normalized selection/direction, and preferred horizontal
position. A completed mutation pushes one pre-edit snapshot and clears redo; no-op edits do neither.
Undo/redo transfer current/restored snapshots between stacks. Provisional IME, navigation,
selection, scrolling, copy, focus, and configuration are not edits. External value replacement
clears history.

## Derived Presentation

### LogicalLine and VisualLine

A logical line is a document range between line breaks. A cached visual line contains its document
range, logical row, content-space origin, size, and GPUI shaped/wrapped layout.

- No-wrap: one logical line maps to one visual line.
- Wrap: a logical line maps to ordered fragments without changing text.
- Empty logical lines still provide caret geometry.
- Relevant text/style/width/wrap changes invalidate cache.
- Only the visible working set plus overscan must retain shaped layout.

### Viewport

| Field | Meaning | Invariant |
|---|---|---|
| `bounds` | Visible content rectangle | Set during layout |
| `content_size` | Laid-out extent | Updated after reflow |
| `scroll_offset` | Horizontal/vertical displacement | Clamped to valid extents |

Wheel input changes permitted axes. Active-position changes minimally reveal the cursor. Enabling
wrap sets horizontal offset to zero. Resize/reflow preserves valid offsets and clamps the rest.

## Interaction State

| Enabled | Read-only | Permitted behavior |
|---|---|---|
| yes | no | All edit, IME, history, navigation, selection, clipboard, and scroll operations |
| yes | yes | Focus, navigation, selection, copy, scroll; no mutation |
| no | either | No normal focus or editor interaction |

The aggregate owns document, selection, optional composition, history, viewport/layout cache,
placeholder, wrap, focus, pointer-drag, and caret-blink state.

Aggregate invariants:

1. All ranges are valid after every transition.
2. Public value excludes placeholder and provisional composition.
3. An actual completed user mutation emits exactly one event and creates at most one history entry.
4. Layout invalidation never changes committed content or selection.
5. The next rendered viewport contains the active insertion/composition point.

