# Public Contract: TextInput

## Purpose

`TextInput` is a reusable, single-line, plain-text GPUI widget exported by the `widget` crate. It
contains no validation policy or application-specific business behavior.

## Construction and configuration

The public interface MUST allow a host to:

- create an input with an initial value;
- optionally set placeholder text;
- enable or disable editing;
- obtain the widget's GPUI focus handle so the host can participate in focus management;
- programmatically replace the value; and
- render the widget in any compatible GPUI view.

Programmatic value replacement MUST normalize the value to one line, place the cursor at a valid
boundary, cancel provisional composition, and establish a new history baseline. It MUST NOT emit a
user-originated change event.

## Value access and observation

The host MUST be able to read the complete committed value at any time. The widget MUST emit a
public change event containing the resulting complete value after each user operation that changes
committed text, including undo, redo, cut, paste, and IME commit.

The widget MUST NOT emit a value-change event for focus changes, cursor movement, selection, copy,
provisional IME updates, rejected disabled interactions, or edits that leave the value unchanged.

## Editing actions

The crate MUST expose or register semantic actions for:

- backward and forward deletion;
- character-wise, word-wise, start, and end navigation;
- the corresponding selection-extending navigation;
- select all;
- copy, cut, and paste; and
- undo and redo.

Default bindings MUST follow each host platform's standard desktop conventions and MUST be scoped
to the TextInput key context so host applications can integrate or override them consistently.

## Pointer and focus behavior

- Clicking an enabled widget focuses it and places the cursor at the nearest valid text boundary.
  When this produces an empty selection, the caret is visible in the first presentation resulting
  from that click; it does not wait for a blink interval or later interaction.
- Dragging from a primary-button press extends selection, including when the pointer leaves the
  visible text bounds.
- Disabled widgets do not mutate cursor, selection, value, composition, or history from user input.
- Focused/enabled, unfocused, disabled, selected, placeholder, and marked-composition states have
  visually distinct representations as applicable.

## Layout behavior

- The widget accepts inherited text metrics from its host while retaining space for one complete
  single-line text layout.
- Its minimum rendered height includes that line and the widget's normal vertical spacing and
  boundary treatment.
- A smaller external height constraint does not vertically clip text, selection, composition, or
  caret indicators.

## Cursor blink behavior

- A focused, enabled input with an empty selection starts with its caret visible and alternates
  visible/hidden phases every 500 ms.
- Text input, deletion, navigation, selection changes, pointer placement, undo, redo, paste, IME
  updates, and focus acquisition reset the cycle with the caret immediately visible. Pointer
  placement that also acquires focus completes this reset in the same interaction update.
- Blur, disablement, or a non-empty selection stops the cycle and prevents stale timer callbacks
  from changing visibility.
- Returning to an eligible caret state starts a fresh cycle; independent TextInput instances do
  not share blink state or timers.
- Blink-only repaint MUST NOT emit `TextInputChange` or alter text, selection, composition, or edit
  history.
- When visible, the caret retains the current input text color, pixel-grid alignment, and one-pixel
  width defined by FR-036.

## Platform text-input contract

The widget MUST provide GPUI with:

- selected text range and direction in UTF-16 code units;
- marked composition range in UTF-16 code units;
- text for requested UTF-16 ranges;
- replacement of committed or marked ranges;
- replacement and marking of provisional composition text;
- screen bounds for requested ranges; and
- character indices for screen positions.

All boundary conversions MUST be checked and clamped to valid text positions. Text passed through
commit, paste, or programmatic configuration MUST be normalized to one visual line.

## Compatibility guarantees

- The widget uses the official `zed-industries/zed` GPUI Git dependency.
- The public contract does not expose internal history, range-storage, or shaping representations.
- Host code can reuse the same component in unrelated GPUI applications without modifying widget
  internals.
- Changes required by upstream GPUI revisions are compatibility changes and require focused
  input, rendering, and IME regression validation.

