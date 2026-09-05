# Public API Contract: TextEditor

## Surface

`TextEditor` is a reusable stateful GPUI entity, re-exported from `widget`. `TextEditorChange`
carries the complete committed value. Editor key initialization is idempotent and does not alter
the existing `TextInput` key context.

```rust
TextEditor::new(cx: &mut Context<TextEditor>) -> TextEditor
TextEditor::with_value(value, cx: &mut Context<TextEditor>) -> TextEditor
TextEditor::value(&self) -> &str
TextEditor::set_value(&mut self, value, cx: &mut Context<TextEditor>)
TextEditor::selection(&self) -> Range<usize>

TextEditor::set_placeholder(&mut self, placeholder, cx: &mut Context<TextEditor>)
TextEditor::placeholder(&self) -> &str
TextEditor::set_wrap(&mut self, wrap: bool, cx: &mut Context<TextEditor>)
TextEditor::is_wrapping(&self) -> bool
TextEditor::set_read_only(&mut self, read_only: bool, cx: &mut Context<TextEditor>)
TextEditor::is_read_only(&self) -> bool
TextEditor::set_enabled(&mut self, enabled: bool, window: &mut Window, cx: &mut Context<TextEditor>)
TextEditor::is_enabled(&self) -> bool
```

- New editors are empty, enabled, editable, wrapping, and place the caret at 0.
- Initial/set values preserve line breaks; initial caret is at end.
- `value` excludes provisional composition and placeholder.
- `set_value` emits no user event, cancels composition, clears history, moves selection to end,
  invalidates layout, and clamps viewport.
- `selection` is a normalized UTF-8 byte range.
- Enabling wrap resets horizontal scroll; configuration never changes text or emits value events.
- Read-only permits non-mutating interaction. Disabled rejects normal focus and all commands.
- The editor fills its parent's available width and height. Content exceeding that viewport scrolls
  internally and displays a vertical scrollbar; the parent container owns sizing policy.

## Change Event

```rust
pub struct TextEditorChange { pub value: SharedString }
```

Exactly one event follows an actual completed user mutation: typing, deletion, newline, replacement,
cut, paste, composition commit, undo, or redo. No event follows host value setting, provisional or
cancelled composition, copy, navigation, selection, focus, scroll, configuration, no-op, or rejected
input.

## Actions and Bindings

Public actions cover backward/forward deletion, newline, left/right and word movement, up/down,
page up/down, line start/end, document start/end, a selection variant for every navigation action,
Select All, Copy, Cut, Paste, Undo, and Redo. Defaults use platform conventions and a `TextEditor`
key context; hosts may override normal GPUI bindings.

## Platform Input Contract

- Implements `Focusable` and `EntityInputHandler`.
- Converts platform UTF-16 ranges to/from normalized internal UTF-8 offsets.
- Replacement and marked-text APIs preserve line breaks only while editable.
- Marked updates are provisional; unmark commits once; cancellation restores committed state.
- `bounds_for_range` returns viewport-translated visible geometry when layout is available.
- Read-only and disabled reject replacement/composition operations.

## Rendering and Pointer Contract

- The element establishes key context and `ElementInputHandler`.
- Pointer press focuses only enabled editors and hit-tests the nearest valid boundary.
- Drag selects and autoscrolls beyond the viewport; the conventional word gesture selects a word.
- Wheel/trackpad changes vertical offset and horizontal offset only without wrapping.
- Painting clips placeholder, text, selection, composition, and caret to viewport.
- Navigation/editing minimally reveals the active position.
- Existing `TextInput` behavior and exports remain compatible.

