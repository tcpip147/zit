# Phase 1 Data Model: Reusable TextInput Widget

## TextInput

The reusable component that owns editing state and exposes value changes to its host.

| Field | Meaning | Validation |
|---|---|---|
| committed value | Current application-visible single-line text | Valid UTF-8; contains no line breaks after normalization |
| working text | Text currently displayed during composition | Valid UTF-8; equals committed value when no composition is active |
| selection | Anchor and active cursor boundaries | Both boundaries lie within working text and on valid text boundaries |
| placeholder | Optional non-value hint | Displayed only when value is empty; never selectable or reported as value |
| enabled | Whether editing interactions are accepted | Disabled state rejects all mutating user paths |
| focus | GPUI focus identity and current focus state | Caret shown only when focused and enabled |
| composition | Optional provisional IME session | Marked range is valid and belongs to working text |
| history | Undo and redo transaction sequences | Redo is cleared by a new committed edit |
| viewport | Horizontal display offset and last layout geometry | Keeps active cursor/composition visible; never alters text |
| pointer selection | Optional drag anchor and active drag state | Exists only during an active selection gesture |
| caret blink | Visibility, eligibility, generation, interval, and owned timer task | At most one current generation may toggle visibility |
| minimum layout height | Current one-line text height plus normal vertical chrome | Never smaller than the space needed for one unclipped text line |

## TextRange

A directed selection represented by an anchor and active end. Its normalized range is used for
replacement and painting; direction is retained for extending the active end.

### Invariants

- Boundaries never split UTF-8 code points.
- Character-wise navigation and deletion land on extended grapheme boundaries.
- An equal anchor and active end represents a cursor.
- A non-empty range is replaced exactly once by input, paste, cut, delete, or composition start.

## CompositionSession

Provisional text managed through the platform IME contract.

| Field | Meaning |
|---|---|
| original snapshot | Committed value and selection before composition began |
| marked range | Range of provisional composition in working text |
| composition selection | Platform-provided selection within or around marked text |

### State transitions

```text
Inactive
  └─ start/update marked text → Composing
Composing
  ├─ update marked text       → Composing
  ├─ commit                   → Inactive + one committed edit + change event
  ├─ cancel/unmark            → Inactive + original committed state restored
  └─ disable/focus handling   → platform-conventional resolution, then Inactive
```

Composition updates never emit committed-value events. A commit normalizes line breaks before
updating the committed value.

## EditTransaction

A reversible committed change.

| Field | Meaning |
|---|---|
| before | Value and directed selection before the edit |
| after | Value and directed selection after the edit |
| kind | Insert, delete, replace, cut, paste, or IME commit |
| coalescing class | Whether adjacent typing/deletion may join the transaction |

### History transitions

```text
New edit:   current → undo stack; clear redo stack
Undo:       pop undo.after; restore undo.before; push transaction to redo
Redo:       pop redo.before; restore redo.after; push transaction to undo
Set value:  replace state; clear composition, undo, and redo
```

## TextInputChange

An event visible to the host after a committed user edit, undo, or redo changes the value.

| Field | Meaning |
|---|---|
| value | Complete resulting committed text |

Focus, navigation, selection, copy, provisional composition, and no-op commands do not emit this
event.

## RenderSnapshot

Transient geometry produced for one rendered line: shaped text, content bounds, cursor x-position,
selection bounds, marked-text bounds, horizontal offset, and the minimum height derived from the
current line metrics and vertical chrome. It is not application state and is recomputed when text,
style, bounds, or scale changes.

## CaretBlinkState

Controls whether the one-pixel, text-colored caret is painted without affecting text or selection.

| Field | Meaning |
|---|---|
| visible | Whether the caret is painted during the current blink phase |
| generation | Monotonic token invalidating callbacks from earlier blink cycles |
| interval | Stable delay between visibility transitions; initially 500 ms |
| task | Entity-owned asynchronous loop for the current generation |

### State transitions

```text
Ineligible (blurred, disabled, or non-empty selection)
  └─ become focused + enabled + caret-only → Visible, new generation/timer
Visible
  ├─ interval expires while eligible      → Hidden
  ├─ user interaction                     → Visible, new generation/timer
  └─ become ineligible                    → Ineligible, invalidate timer
Hidden
  ├─ interval expires while eligible      → Visible
  ├─ user interaction                     → Visible, new generation/timer
  └─ become ineligible                    → Ineligible, invalidate timer
```

Blink transitions request repaint only. They never change the value, selection, composition,
history, or host change-event stream.

Pointer-acquired focus performs the focus, caret placement, and visible-phase reset as one logical
transition. A later focus notification may restart the current generation but must never create a
first focused presentation in which an eligible caret remains hidden.

## MinimumHeightConstraint

The rendered widget derives a lower layout bound from the current single-line text metrics and its
normal vertical spacing and boundary treatment. Both the outer control and inner text line resist
shrinking below their respective minimums. Changes to inherited text metrics recompute this value;
the constraint affects presentation only and never changes text or interaction state.

