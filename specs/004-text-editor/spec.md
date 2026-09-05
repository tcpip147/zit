# Feature Specification: Reusable TextEditor Widget

**Feature Branch**: `not created (no before_specify hook configured)`

**Created**: 2026-09-04

**Status**: Draft

**Input**: User description: "Create a TextEditor supporting multi-line text, insertion, deletion, comprehensive cursor movement, keyboard and mouse selection, clipboard operations, IME, undo/redo, vertical and horizontal scrolling, line wrapping, placeholder text, read-only, and disabled states."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Enter and Edit Multi-line Text (Priority: P1)

As a desktop application user, I can enter, insert, and delete plain text across multiple lines so
that I can create and revise longer content.

**Why this priority**: Multi-line editing is the minimum useful behavior that distinguishes the
TextEditor from the existing single-line TextInput.

**Independent Test**: Focus an enabled editor, create several lines, insert text at different
positions, and delete both selected and adjacent text while verifying the resulting value.

**Acceptance Scenarios**:

1. **Given** an enabled empty editor, **When** the user types text and inserts line breaks, **Then**
   the editor displays and exposes the resulting multi-line value.
2. **Given** an enabled editor with the cursor within existing content, **When** the user enters
   text, **Then** the text is inserted at the cursor and the cursor moves after it.
3. **Given** an enabled editor with no selection, **When** the user invokes backward or forward
   deletion, **Then** the preceding or following user-perceived character or line boundary is
   removed when one exists.
4. **Given** an enabled editor with selected text spanning one or more lines, **When** the user
   types, pastes, or deletes, **Then** the selection is replaced or removed exactly once.

---

### User Story 2 - Navigate and Select Precisely (Priority: P1)

As a user, I can move the cursor and select content with standard keyboard and pointer gestures so
that I can edit any part of a document accurately.

**Why this priority**: Longer documents are impractical to edit without predictable navigation and
selection.

**Independent Test**: On a document containing short, long, wrapped, and blank lines, exercise all
specified cursor and selection commands and verify their resulting positions and ranges.

**Acceptance Scenarios**:

1. **Given** a focused editor, **When** the user presses Left or Right, **Then** the cursor moves to
   the previous or next valid user-perceived character boundary.
2. **Given** a focused editor, **When** the user presses Up or Down, **Then** the cursor moves by one
   visual line while preserving its preferred horizontal position when possible.
3. **Given** a focused editor, **When** the user invokes Home or End, **Then** the cursor moves to
   the start or end of the current visual line according to host-platform conventions.
4. **Given** a focused editor, **When** the user invokes Document Home or Document End, **Then** the
   cursor moves to the beginning or end of the complete value.
5. **Given** a focused editor, **When** the user invokes Page Up or Page Down, **Then** the viewport
   and cursor move approximately one visible page in the requested direction without leaving the
   document bounds.
6. **Given** any supported keyboard navigation command, **When** the user holds the platform's
   selection modifier, **Then** the selection extends or contracts from its anchor to the new
   cursor position.
7. **Given** an enabled editor, **When** the user clicks and drags across text, **Then** the nearest
   valid text positions define and continuously update the selection.
8. **Given** a word, **When** the user performs the platform's conventional word-selection pointer
   gesture on it, **Then** that word is selected.
9. **Given** a focused editor, **When** the user invokes Select All, **Then** the complete text value
   is selected.

---

### User Story 3 - Transfer and Recover Content (Priority: P1)

As a user, I can copy, cut, paste, undo, and redo so that I can reorganize content and recover from
mistakes safely.

**Why this priority**: Clipboard and history operations are expected safeguards for substantive
text editing.

**Independent Test**: Perform insertions, deletions, replacements, cuts, and pastes across multiple
lines, then undo and redo the sequence while checking value, cursor, selection, and viewport.

**Acceptance Scenarios**:

1. **Given** selected text, **When** the user copies it, **Then** the exact selected plain text,
   including line breaks, becomes available for pasting and the value remains unchanged.
2. **Given** selected text in an editable editor, **When** the user cuts it, **Then** it is copied
   and removed from the value.
3. **Given** an editable editor, **When** the user pastes text, **Then** it replaces the selection or
   is inserted at the cursor with its supported line breaks preserved.
4. **Given** completed value-changing edits, **When** the user invokes Undo repeatedly, **Then**
   available edits are reversed in most-recent-first order with their cursor and selection states.
5. **Given** undone edits and no intervening edit, **When** the user invokes Redo, **Then** those
   edits are restored in order; a new edit after Undo discards the superseded redo sequence.

---

### User Story 4 - Read and Edit Long Documents (Priority: P2)

As a user, I can scroll through content and choose whether long lines wrap so that documents remain
usable within the available editor area.

**Why this priority**: Viewport management is necessary once content exceeds the visible bounds.

**Independent Test**: Use content exceeding both viewport dimensions, toggle wrapping, navigate and
scroll to all extremes, and verify visible content and cursor tracking.

**Acceptance Scenarios**:

1. **Given** content taller than the viewport, **When** the user scrolls vertically, **Then** the
   requested portion becomes visible without changing the value or selection.
2. **Given** wrapping disabled and a line wider than the viewport, **When** the user scrolls
   horizontally, **Then** the requested horizontal portion becomes visible.
3. **Given** wrapping enabled and a logical line wider than the viewport, **When** it is displayed,
   **Then** it continues on additional visual lines and does not require horizontal scrolling to
   read its text.
4. **Given** cursor navigation or editing moves the cursor outside the viewport, **When** the
   operation completes, **Then** the viewport moves by the minimum needed to reveal the cursor.
5. **Given** the wrapping mode or viewport size changes, **When** content is laid out again, **Then**
   the text value and valid selection are preserved.

---

### User Story 5 - Enter International Text (Priority: P1)

As a user, I can compose text with the host platform's input method editor so that I can enter
languages such as Korean, Japanese, and Chinese.

**Why this priority**: International input is fundamental to a general-purpose editor.

**Independent Test**: Start, update, commit, and cancel compositions in empty text, within existing
text, and over multi-line selections while checking both displayed and committed values.

**Acceptance Scenarios**:

1. **Given** a focused editable editor, **When** composition begins or updates, **Then** the current
   composition is visibly distinguishable and the committed value is not prematurely changed.
2. **Given** an active composition, **When** it is committed, **Then** its text replaces the current
   selection or is inserted at the cursor exactly once.
3. **Given** an active composition, **When** it is cancelled, **Then** uncommitted text disappears
   and the previously committed value remains unchanged.
4. **Given** an active composition near a viewport edge, **When** it changes, **Then** the relevant
   composition and insertion position remain visible.

---

### User Story 6 - Configure Interaction States (Priority: P2)

As an application developer, I can configure placeholder, wrapping, read-only, and disabled states
and observe value changes so that the same reusable editor fits different GPUI applications.

**Why this priority**: Clear interaction states and host integration make the widget reusable and
prevent unintended modification.

**Independent Test**: Embed independently configured editors, switch each supported state, attempt
all input methods, and verify visuals, focus behavior, selection, clipboard, and notifications.

**Acceptance Scenarios**:

1. **Given** an empty editor with placeholder text, **When** it displays no active composition,
   **Then** the placeholder is visible but is neither value content nor selectable text.
2. **Given** a read-only editor, **When** the user focuses, navigates, selects, scrolls, or copies,
   **Then** those non-mutating interactions work, while typing, deletion, cut, paste, undo, redo,
   and committed composition do not change the value.
3. **Given** a disabled editor, **When** the user attempts keyboard, pointer editing, selection, or
   clipboard interaction, **Then** it does not accept focus through normal user interaction or
   change its value, selection, history, or viewport because of that interaction.
4. **Given** a user edit changes the value, **When** the operation completes, **Then** the host
   application can observe the resulting current value exactly once.
5. **Given** any enabled, read-only, disabled, focused, or unfocused state, **When** the editor is
   displayed, **Then** users can distinguish the states relevant to permitted interaction.

### Edge Cases

- Navigation and deletion at the beginning or end of the document stop safely without changing
  content or producing invalid positions.
- Empty documents, trailing line breaks, consecutive blank lines, and the final empty line remain
  navigable and selectable.
- Up, Down, Page Up, and Page Down across lines of unequal width place the cursor at the closest
  valid boundary and retain the preferred horizontal position for subsequent vertical movement.
- Wrapped visual lines participate in visual-line navigation, Home/End, page movement, pointer hit
  testing, selection, and vertical scrolling without inserting line breaks into the value.
- Multi-code-point user-perceived characters and mixed-direction text are never split into visibly
  broken partial characters by navigation, deletion, or selection endpoints.
- Clicking outside rendered text but inside the editor places the cursor at the nearest valid text
  position; dragging beyond the viewport continues selection and scrolls when more content exists.
- Empty selections act as cursors; copy or cut with no selection leaves the value and clipboard
  unchanged.
- Clipboard unavailability, empty paste content, or unavailable undo/redo history leaves editor
  content unchanged and does not corrupt state.
- Toggling read-only or disabled during composition safely ends the composition without committing
  unintended text.
- External value replacement invalidates or adjusts cursor, selection, history, and composition so
  every remaining position is valid for the new value.
- Resizing or changing wrap mode at document or viewport extremes clamps scrolling to valid ranges
  and avoids leaving blank inaccessible space where content should be visible.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: TextEditor MUST display and expose a plain-text value containing zero or more lines.
- **FR-002**: An enabled, non-read-only TextEditor MUST insert entered text and line breaks at the
  cursor or replace the active selection.
- **FR-003**: Backward and forward deletion MUST remove the active selection or the adjacent
  user-perceived character or line boundary when one exists.
- **FR-004**: Cursor and selection endpoints MUST always remain on valid text boundaries.
- **FR-005**: TextEditor MUST support Left, Right, Up, Down, Home, End, Document Home, Document End,
  Page Up, and Page Down cursor navigation using host-platform conventions.
- **FR-006**: Vertical cursor movement MUST preserve a preferred horizontal position when moving
  through visual lines of differing widths whenever that position can be meaningfully retained.
- **FR-007**: Every supported keyboard navigation operation MUST support extending or contracting
  the selection using the host platform's selection modifier.
- **FR-008**: TextEditor MUST support pointer placement, drag selection, conventional word
  selection, and Select All.
- **FR-009**: TextEditor MUST clearly distinguish the cursor, selected text, active IME composition,
  and unselected committed text whenever each is applicable.
- **FR-010**: TextEditor MUST support standard host-platform Copy, Cut, and Paste commands while
  preserving supported line breaks.
- **FR-011**: Copy MUST be non-mutating; Cut and Paste MUST mutate only when the editor is editable.
- **FR-012**: TextEditor MUST support Undo and Redo for typing, line-break insertion, deletion,
  replacement, cut, paste, and committed IME input.
- **FR-013**: Undo and Redo MUST restore the value and associated cursor and selection result, and a
  new edit after Undo MUST discard superseded redo history.
- **FR-014**: TextEditor MUST support input through the host platform's standard IME composition
  behavior, including start, update, commit, and cancellation.
- **FR-015**: Uncommitted composition MUST remain distinct from the committed value and MUST NOT be
  duplicated, corrupted, or partially committed.
- **FR-016**: TextEditor MUST provide vertical scrolling when laid-out content exceeds the viewport
  height.
- **FR-017**: With line wrapping disabled, TextEditor MUST provide horizontal scrolling when content
  exceeds the viewport width.
- **FR-018**: With line wrapping enabled, each logical line MUST flow across visual lines within the
  available width without modifying the underlying value.
- **FR-019**: TextEditor MUST keep the cursor and active composition visible after navigation or
  editing and MUST clamp both scroll axes to valid content bounds.
- **FR-020**: Page Up and Page Down MUST move by approximately the visible viewport height while
  preserving a valid cursor position and making progress unless already at the relevant boundary.
- **FR-021**: Configurable placeholder text SHOULD appear only when the committed value is empty and
  no composition is active; it MUST NOT be part of the value, selection, clipboard, or history.
- **FR-022**: A read-only TextEditor MUST allow focus, navigation, selection, scrolling, and copying,
  but MUST reject every user operation that would change its value or edit history.
- **FR-023**: A disabled TextEditor MUST be visually distinguishable, MUST NOT receive focus through
  normal user interaction, and MUST reject user editing, navigation, selection, and clipboard
  commands.
- **FR-024**: The host application MUST be able to set and read the value and configure placeholder,
  wrap, read-only, and disabled states.
- **FR-025**: The host application MUST be notified exactly once for each completed user operation
  that changes the committed value and MUST NOT receive value-change notifications for navigation,
  selection, scrolling, copying, cancelled composition, or rejected operations.
- **FR-026**: TextEditor MUST preserve valid value, cursor, selection, composition, history, and
  viewport state when focus or configuration changes.
- **FR-027**: TextEditor MUST follow host-platform desktop text-editing conventions where this
  specification does not define an observable behavior.
- **FR-028**: TextEditor MUST be reusable across GPUI applications without application-specific
  business rules.
- **FR-029**: TextEditor MUST use GPUI as its UI foundation and belong to the constitutionally
  Zit workspace structure with a reusable widget library.

### Key Entities

- **Text value**: The committed plain-text document, including logical line breaks; placeholder and
  uncommitted composition text are excluded.
- **Text position**: A valid boundary within the value used by the cursor and selection endpoints.
- **Cursor**: The active text position for insertion and navigation when selection is empty.
- **Selection**: A range with an anchor and active endpoint that may span logical and visual lines.
- **Composition**: Temporary IME text and its composition selection before commit or cancellation.
- **Edit history**: Ordered reversible user edits together with their resulting cursor and selection
  states.
- **Viewport**: The visible region of laid-out content and its horizontal and vertical scroll
  positions.
- **Interaction state**: The editor's focus, editable, read-only, and disabled conditions that
  determine permitted operations and presentation.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In acceptance testing, 100% of specified multi-line insertion, deletion, navigation,
  selection, clipboard, IME, undo, and redo scenarios produce the expected value and interaction
  state.
- **SC-002**: At least 95% of representative desktop users can create three lines, correct text on
  another line, select a word, and undo the change on their first attempt without instruction.
- **SC-003**: Cursor movement, selection, editing, and scrolling provide visible feedback within 100
  milliseconds under normal desktop use for documents of at least 10,000 lines and 1,000,000
  characters.
- **SC-004**: In tests covering documents larger than the viewport, 100% of cursor-moving edit and
  navigation operations leave the cursor visible and all scroll positions within valid bounds.
- **SC-005**: Across representative narrow, wide, blank, and wrapped lines, 100% of navigation and
  pointer-selection tests produce valid text boundaries without broken user-perceived characters.
- **SC-006**: In usability review, 100% of participants can distinguish editable, read-only, and
  disabled editors and correctly predict whether content can be changed.
- **SC-007**: Every completed user-originated value change generates exactly one observable change
  event, with zero events generated by non-mutating or rejected interactions in the acceptance set.
- **SC-008**: At least two independently configured TextEditor instances can coexist in one
  demonstration view without widget changes or application-specific behavior.

## Assumptions

- The initial TextEditor handles plain text only; rich-text formatting, syntax highlighting,
  diagnostics, search/replace, file loading/saving, and collaborative editing are outside scope.
- Standard shortcuts, modifier keys, word boundaries, Home/End behavior, and selection gestures
  follow conventions of the host desktop platform.
- Line wrapping is configurable and enabled by default; horizontal scrolling applies when wrapping
  is disabled, while vertical scrolling remains available in both modes.
- Undo and redo cover user edits during the current widget lifetime; persisted or shared history is
  outside scope, and adjacent typing may be grouped according to platform convention.
- Read-only permits selecting and copying content; disabled prevents normal user interaction.
- Styling is theme-dependent, but focus, selection, composition, read-only, and disabled states
  remain perceptibly distinguishable.
- External value or configuration updates are host-controlled operations and do not themselves
  represent user edits or require user-value-change notifications.
- The widget is implemented in the reusable `crates/widget` crate and uses the official GPUI
  dependency required by the Zit Constitution.

