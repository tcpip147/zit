# Feature Specification: Reusable TextInput Widget

**Feature Branch**: `not created (no before_specify hook configured)`

**Created**: 2026-09-04

**Status**: Implemented; specification synchronized with current widget behavior

**Input**: User description: "Create a reusable single-line TextInput widget for Zit with standard desktop keyboard and mouse editing behavior."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Enter and Edit Text (Priority: P1)

As a desktop application user, I can focus a TextInput and enter or edit a single line of text so
that I can provide textual information to the application.

**Why this priority**: Displaying and editing text is the minimum useful behavior of a text input.

**Independent Test**: Focus an enabled TextInput, type at multiple cursor positions, and delete
characters on either side of the cursor while confirming the visible and readable value.

**Acceptance Scenarios**:

1. **Given** an enabled empty TextInput, **When** the user focuses it and types text, **Then** the
   text appears at the cursor and becomes the current value.
2. **Given** an enabled TextInput containing text with the cursor between characters, **When** the
   user types, **Then** the new text is inserted at that cursor position.
3. **Given** an enabled TextInput with no selection, **When** the user presses Backspace or Delete,
   **Then** the character before or after the cursor respectively is removed when one exists.
4. **Given** a TextInput containing more text than can be shown at once, **When** the cursor moves
   beyond the visible portion, **Then** the visible content follows the cursor without changing the
   value.

---

### User Story 2 - Navigate and Select Text (Priority: P1)

As a user, I can position the cursor and select text with familiar keyboard and mouse interactions
so that I can precisely choose what to edit.

**Why this priority**: Accurate navigation and selection are essential for correcting existing
text rather than merely appending new text.

**Independent Test**: Use arrow, Home, End, modified navigation, click, and drag interactions on a
populated field and verify cursor and selection boundaries after each action.

**Acceptance Scenarios**:

1. **Given** a focused TextInput, **When** the user invokes standard character, word, start, or end
   navigation, **Then** the cursor moves to the corresponding valid text boundary.
2. **Given** a focused TextInput, **When** the user extends navigation while holding the platform's
   selection modifier, **Then** text between the selection anchor and new cursor position is
   selected.
3. **Given** an enabled, unfocused TextInput, **When** the user clicks within its text area,
   **Then** it gains focus, places the cursor at the nearest valid text boundary, and displays the
   cursor in that same visual update without waiting for the first blink interval.
4. **Given** an enabled TextInput, **When** the user presses and drags across text, **Then** the text
   between the drag origin and current pointer position is selected.
5. **Given** a cursor or non-empty selection, **When** it is displayed, **Then** the cursor and
   selected range are clearly distinguishable from unselected text in the current visual state.
6. **Given** a focused TextInput with no active selection, **When** the cursor is displayed at any
   valid text position, **Then** it uses the same color as the input text and has a consistent
   one-pixel width.
7. **Given** a focused and enabled TextInput with no active selection, **When** the user is not
   actively editing, **Then** the cursor alternates between visible and hidden states at a steady,
   perceptible interval.
8. **Given** a blinking cursor, **When** the user types, moves the cursor, changes the selection,
   or focuses the TextInput, **Then** the cursor becomes visible immediately and restarts its blink
   cycle.

---

### User Story 3 - Replace and Transfer Text (Priority: P2)

As a user, I can replace selected text and use copy, cut, and paste so that I can efficiently reuse
and revise text.

**Why this priority**: These operations complete the standard editing workflow after basic entry
and selection are available.

**Independent Test**: Select a substring, replace it by typing, and exercise copy, cut, and paste
through standard platform commands while checking the resulting value and selection.

**Acceptance Scenarios**:

1. **Given** selected text, **When** the user types or pastes text, **Then** the selected text is
   replaced and the cursor is placed after the inserted text.
2. **Given** selected text, **When** the user copies it, **Then** the selected text becomes available
   for pasting and the TextInput value is unchanged.
3. **Given** selected text in an enabled TextInput, **When** the user cuts it, **Then** the selected
   text becomes available for pasting and is removed from the value.
4. **Given** no selection in an enabled TextInput, **When** the user pastes, **Then** clipboard text
   is inserted at the cursor.

---

### User Story 4 - Undo and Redo Changes (Priority: P2)

As a user, I can undo and redo editing operations so that I can safely recover from mistakes and
restore changes.

**Why this priority**: Reversible editing is expected in desktop text controls and protects user
work.

**Independent Test**: Perform typing, deletion, replacement, cut, and paste operations, then walk
backward and forward through the resulting edit history.

**Acceptance Scenarios**:

1. **Given** one or more completed text edits, **When** the user repeatedly invokes undo, **Then**
   each available edit is reversed in most-recent-first order, including its cursor and selection
   result.
2. **Given** one or more undone edits and no intervening new edit, **When** the user invokes redo,
   **Then** each undone edit is restored in order.
3. **Given** an undone edit, **When** the user performs a new text edit, **Then** the superseded redo
   history is no longer available.

---

### User Story 5 - Configure and Observe the Widget (Priority: P3)

As an application developer, I can reuse TextInput in different GPUI applications, configure its
basic state and placeholder, read its value, and respond to changes without application-specific
behavior being built into the widget.

**Why this priority**: Application integration and reusable states turn editing behavior into a
usable library component.

**Independent Test**: Place TextInput in two unrelated host views, configure value, placeholder,
and enabled state, then verify value reads and change notifications for user edits.

**Acceptance Scenarios**:

1. **Given** an empty unfocused TextInput with placeholder text, **When** it is displayed, **Then**
   the placeholder is visible but is not part of the current value.
2. **Given** a disabled TextInput, **When** the user attempts keyboard, mouse-editing, cut, paste,
   undo, or redo operations, **Then** its value and edit history do not change.
3. **Given** an enabled TextInput, **When** a user edit changes its value, **Then** the host
   application can observe the updated value and respond to the change.
4. **Given** focused and unfocused TextInputs, **When** they are displayed, **Then** their focus
   states are visually distinguishable.
5. **Given** a host layout that requests a height smaller than the current text line, **When** the
   TextInput is displayed, **Then** it retains enough height to show one complete line of text
   without clipping its glyphs.

---

### User Story 6 - Enter International Text (Priority: P1)

As a user, I can enter text using an input method editor (IME) so that I can use languages that require text composition, such as Korean, Japanese, and Chinese.

**Why this priority**: IME support is essential for a general-purpose text input widget and is required for users whose languages rely on text composition.

**Independent Test**: Activate an IME, compose text, modify the composition, commit the composition, and cancel the composition while verifying the displayed text, cursor position, and final value.

**Acceptance Scenarios**:

1. **Given** a focused and enabled TextInput, **When** the user begins an IME composition, **Then** the composition text is displayed without immediately becoming part of the committed text value.

2. **Given** an active IME composition, **When** the user modifies the composition, **Then** the displayed composition updates without corrupting the existing text value.

3. **Given** an active IME composition, **When** the user commits the composition, **Then** the composed text is inserted into the current value at the cursor position.

4. **Given** an active IME composition, **When** the user cancels the composition, **Then** the uncommitted composition is removed and the committed text value remains unchanged.

5. **Given** an active IME composition, **When** the user interacts with the cursor or selection according to the host platform's standard behavior, **Then** the TextInput preserves valid text and composition state without producing corrupted or partial characters.

6. **Given** a TextInput configured as single-line, **When** the IME commits text containing a line break, **Then** the line break is normalized according to the single-line behavior defined by this specification.

### Edge Cases

- Backspace at the start and Delete at the end leave the value unchanged.
- Navigation stops at valid start and end boundaries and never produces an invalid cursor or
  selection position.
- Typing, deletion, cut, or paste with an active selection applies to the selection exactly once.
- An empty selection behaves as a cursor, not as a selected range.
- Clicking before the first visible character or after the last visible character places the
  cursor at the nearest valid boundary.
- Dragging outside the widget continues to update selection toward the closest reachable text
  position until the drag ends.
- Pasted text containing line breaks is converted to a single-line value without creating a
  second visual line.
- Empty clipboard content and unavailable undo or redo history leave the value unchanged.
- Losing focus preserves the value and selection but hides the active cursor indication.
- Disabling a focused TextInput prevents further editing and removes it from active keyboard
  interaction until it is enabled again.
- Text containing multi-code-point user-perceived characters is navigated and deleted without
  leaving visibly broken partial characters.
- Cursor geometry is aligned to the display pixel grid so moving between glyph positions does not
  make the one-pixel cursor appear alternately one or two pixels wide.
- Cursor blinking stops while the TextInput is unfocused, disabled, or displaying a non-empty
  selection; returning to a focused, enabled caret state restarts the cycle with the cursor visible.
- Clicking an unfocused TextInput never leaves a focused field temporarily without a visible
  cursor; the cursor is visible in the first updated presentation resulting from that click.
- A height constraint smaller than the current text line does not shrink the TextInput below the
  height required for one complete line and its normal visual chrome.
- Starting, updating, committing, and cancelling an IME composition must not duplicate committed text.
- Cancelling an IME composition must restore the previously committed value.
- Composing text immediately before or after a selection must replace the selection according to standard text editing behavior.
- IME composition must work correctly when the cursor is positioned in the middle of an existing value.
- IME input containing multi-code-point characters must not leave broken or partial user-perceived characters.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: TextInput MUST display its current single-line text value.
- **FR-002**: An enabled TextInput MUST accept keyboard focus through standard desktop focus
  navigation and direct pointer interaction.
- **FR-003**: TextInput MUST expose visually distinguishable focused and unfocused states.
- **FR-004**: TextInput MUST support enabled and disabled states, with the disabled state visually
  distinguishable from the enabled state.
- **FR-005**: A disabled TextInput MUST NOT accept text input or perform editing, clipboard,
  selection-changing mouse, undo, or redo operations.
- **FR-006**: TextInput MUST insert entered text at the current cursor position.
- **FR-007**: TextInput MUST replace the active selection when the user enters or pastes text.
- **FR-008**: Backspace MUST remove the selection when one exists; otherwise it MUST remove the
  user-perceived character immediately before the cursor when one exists.
- **FR-009**: Delete MUST remove the selection when one exists; otherwise it MUST remove the
  user-perceived character immediately after the cursor when one exists.
- **FR-010**: TextInput MUST support standard platform keyboard commands for character-wise and
  word-wise cursor movement and movement to the start and end of the value.
- **FR-011**: TextInput MUST support extending and reducing a selection with standard platform
  keyboard selection commands.
- **FR-012**: TextInput MUST place the cursor at the nearest valid text boundary when the user
  clicks within the text area.
- **FR-013**: TextInput MUST create and update a text selection while the user drags from a pointer
  press within the text area.
- **FR-014**: While TextInput is focused and enabled with no non-empty selection, it MUST clearly
  display the current cursor position during the visible phase of the cursor blink cycle.
- **FR-015**: TextInput MUST clearly distinguish selected text from unselected text while focused.
- **FR-016**: TextInput MUST support standard platform copy, cut, and paste commands.
- **FR-017**: Copy MUST leave the current value unchanged; cut MUST remove selected text only when
  editing is enabled; paste MUST replace the selection or insert at the cursor.
- **FR-018**: TextInput MUST support undo and redo for value-changing typing, deletion, replacement,
  cut, and paste operations in the order users performed them.
- **FR-019**: Performing a new value-changing edit after undo MUST discard the superseded redo
  sequence.
- **FR-020**: TextInput SHOULD display configurable placeholder text when its value is empty and
  MUST NOT treat placeholder text as part of the value or selectable content.
- **FR-021**: The host application MUST be able to set and read the current text value.
- **FR-022**: The host application MUST be able to respond whenever a user editing operation
  changes the text value, receiving the resulting current value.
- **FR-023**: TextInput MUST preserve single-line presentation and MUST prevent entered or pasted
  line breaks from producing additional lines.
- **FR-024**: TextInput MUST keep the cursor visible when the value is wider than the available
  display area.
- **FR-025**: Cursor movement and deletion MUST operate on valid user-perceived character
  boundaries so editing does not leave visibly broken partial characters.
- **FR-026**: TextInput MUST follow the host platform's conventional shortcuts and observable
  behavior for single-line desktop text editing unless this specification states otherwise.
- **FR-027**: TextInput MUST be reusable across GPUI applications and MUST NOT contain
  application-specific business rules.
- **FR-028**: TextInput MUST use GPUI as its UI foundation and reside within the constitutionally
  Zit workspace structure with a reusable widget library.

- **FR-029**: TextInput MUST support text input through the host platform's standard IME/text composition mechanism.

- **FR-030**: TextInput MUST distinguish uncommitted IME composition text from the committed text value.

- **FR-031**: TextInput MUST update the displayed composition when the active IME composition changes.

- **FR-032**: TextInput MUST commit IME composition text into the current value when the composition is committed.

- **FR-033**: TextInput MUST discard uncommitted composition text when the active IME composition is cancelled.

- **FR-034**: IME composition MUST NOT corrupt, duplicate, or partially remove existing text.

- **FR-035**: The host application MUST observe the resulting committed value when an IME composition changes the TextInput value.
- **FR-036**: The visible cursor MUST use the current input text color, MUST have a one-pixel width,
  and MUST remain visually consistent at every valid cursor position by aligning its geometry to
  the display pixel grid.
- **FR-037**: The cursor MUST blink while the TextInput is focused, enabled, and has no non-empty
  selection. The blink cycle MUST begin in the visible state, MUST reset to visible after keyboard
  input, cursor movement, selection-changing navigation or pointer interaction, or focus
  acquisition, and MUST stop when the TextInput becomes unfocused, disabled, or has a non-empty
  selection.
- **FR-038**: TextInput MUST resist layout constraints that would make it shorter than one complete
  line of its currently displayed text. Its resulting minimum height MUST also retain the widget's
  normal vertical spacing and boundary treatment so text, selection, and cursor indicators are not
  vertically clipped.
- **FR-039**: When a pointer click gives an enabled TextInput focus and leaves an empty selection,
  the cursor MUST become visible in the same user-visible update that applies focus and cursor
  placement. Cursor display MUST NOT be deferred until a blink interval or a later unrelated
  interaction occurs.

### Key Entities

- **Text value**: The editable single-line content exposed to both the user and host application;
  placeholder text is excluded.
- **Cursor**: The valid text boundary at which insertion occurs when there is no selection.
- **Selection**: A possibly empty range within the text value, with an anchor and an active end;
  value-changing input replaces a non-empty selection.
- **Edit history**: The ordered set of reversible value-changing user operations and their
  associated cursor or selection outcomes.
- **Interaction state**: Whether the TextInput is enabled, disabled, focused, or unfocused, which
  determines its visual presentation and permitted user interactions.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In acceptance testing, 100% of specified typing, insertion, deletion, replacement,
  navigation, selection, clipboard, undo, and redo scenarios produce the expected visible value,
  cursor, and selection.
- **SC-002**: At least 95% of representative users can focus the widget, correct text in the middle
  of a value, and submit the intended result on their first attempt without instruction.
- **SC-003**: Users receive a visible response to focus, cursor movement, selection, and text edits
  within 100 milliseconds under normal desktop interaction conditions.
- **SC-004**: In visual review across every supported interaction state, 100% of reviewers can
  correctly identify whether the widget is focused, disabled, and whether text is selected.
- **SC-005**: At least two independently configured TextInput instances can be integrated into the
  same demonstration view with no widget code changes and no application-specific business
  behavior.
- **SC-006**: Every user-originated value change is observable by the host application, while
  navigation, focus changes, copying, and unchanged edit attempts produce no false value-change
  notification.
- **SC-007**: Visual validation across representative cursor positions confirms that 100% of
  displayed cursors match the input text color and retain a consistent one-pixel width.
- **SC-008**: In interaction testing, the cursor becomes visible immediately after every focus or
  editing interaction and continues alternating visibility at a stable interval while all blinking
  conditions remain satisfied.
- **SC-009**: Across all tested text sizes, applying a smaller external height constraint results in
  100% of displayed TextInputs retaining at least one complete, unclipped line of text together
  with their normal vertical spacing and boundary treatment.
- **SC-010**: In interaction testing, 100% of pointer clicks that focus an enabled TextInput with an
  empty selection show its cursor in the first visual update caused by the click, before any blink
  interval elapses.

## Assumptions

## As-Built Alignment

- The widget exposes its command bindings and initialization through the reusable widget crate while preserving host-observable committed-value events.
- Composition, selection, cursor, and clipboard behavior are rendered in the same control rather than delegated to a project-specific page.

- The initial feature is a general-purpose, single-line, plain-text editor; rich text, validation,
  masking, password concealment, and multi-line editing are outside its scope.
- "Standard" shortcuts and modifier keys follow the conventions of the host desktop platform.
- Line breaks in typed or pasted content are normalized so the value remains a single line.
- Undo and redo cover user editing performed during the current widget lifetime; persistence of
  history across application restarts is outside scope.
- Visual styling details are theme-dependent, but cursor, selection, focus, and disabled states
  remain clearly distinguishable.
- The widget is implemented in the reusable `crates/widget` crate and uses the official GPUI
  dependency required by the Zit Constitution.

