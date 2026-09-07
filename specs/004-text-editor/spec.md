# Feature Specification: Reusable TextEditor Widget

**Feature Branch**: Not created; no branch-creation hook is configured.

**Created**: 2026-09-04

**Status**: Implemented foundation synchronized with the current widget

**Input**: User description: "Create a reusable multi-line plain-text editor with desktop editing, navigation, clipboard, history, IME, scrolling, and configurable interaction states."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Edit Multi-line Plain Text (Priority: P1)

As a user, I can enter, replace, and delete plain text across multiple logical lines.

**Why this priority**: Multi-line mutation is the minimum behavior that distinguishes the editor from a single-line input.

**Independent Test**: Insert text and line breaks, replace a selection spanning lines, and delete in both directions while checking the value and selection.

**Acceptance Scenarios**:

1. **Given** an editable editor, **When** the user enters text or a line break, **Then** it replaces the current selection or is inserted at the cursor.
2. **Given** no selection, **When** Backspace or Delete is invoked, **Then** the adjacent user-perceived character or line boundary is removed when one exists.
3. **Given** a multi-line selection, **When** the user enters, pastes, or deletes, **Then** the selected range is replaced or removed exactly once.
4. **Given** text containing Windows- or classic-Mac-style line endings, **When** it is assigned or inserted, **Then** the editor stores and exposes consistent line-feed separators.

---

### User Story 2 - Navigate and Select Logical Text (Priority: P1)

As a user, I can navigate and select text with standard character, word, logical-line, document, page, and pointer commands.

**Why this priority**: Users must be able to reach and select any editable part of a document.

**Independent Test**: Exercise each navigation command and its selection-extending variant over short, long, blank, and Unicode-containing logical lines.

**Acceptance Scenarios**:

1. **Given** a focused editor, **When** Left, Right, word movement, Home, or End is invoked, **Then** the cursor moves to a valid character, word, or current logical-line boundary.
2. **Given** a focused editor, **When** Up or Down is invoked, **Then** the cursor moves to the nearest valid byte position at the same logical-line column on the adjacent logical line.
3. **Given** a focused editor, **When** Document Home or Document End is invoked, **Then** the cursor moves to the start or end of the complete value.
4. **Given** a focused editor, **When** Page Up or Page Down is invoked, **Then** the cursor moves by up to ten logical lines without leaving the document.
5. **Given** any supported navigation command, **When** its selection-extending form is invoked, **Then** the selection changes from its anchor to the resulting valid position.
6. **Given** an enabled editor, **When** the user clicks or drags over rendered text, **Then** the nearest rendered position becomes the cursor or active selection endpoint.
7. **Given** a focused editor, **When** Select All is invoked, **Then** the complete value is selected.

---

### User Story 3 - Transfer and Recover Content (Priority: P1)

As a user, I can copy, cut, paste, undo, and redo multi-line content.

**Why this priority**: Clipboard and history operations protect and accelerate editing work.

**Independent Test**: Perform insertions, deletions, selection replacements, cuts, and pastes, then undo and redo them while checking value and selection restoration.

**Acceptance Scenarios**:

1. **Given** a non-empty selection, **When** Copy is invoked, **Then** the exact selected plain text is placed on the clipboard without changing the value.
2. **Given** an editable selection, **When** Cut is invoked, **Then** the selected text is copied and removed.
3. **Given** editable content, **When** Paste is invoked, **Then** clipboard text replaces the selection or is inserted at the cursor while preserving normalized line breaks.
4. **Given** completed edits, **When** Undo is invoked, **Then** the latest value and selection snapshot is restored.
5. **Given** an undone edit, **When** Redo is invoked before another edit, **Then** the undone snapshot is restored; a new edit clears the redo sequence.

---

### User Story 4 - Enter International Text (Priority: P1)

As a user, I can compose multi-line text with the host platform's input method editor.

**Why this priority**: Composition is essential for languages including Korean, Japanese, and Chinese.

**Independent Test**: Start, update, commit, and cancel compositions in empty text and over an existing selection.

**Acceptance Scenarios**:

1. **Given** a focused editable editor, **When** composition text is updated, **Then** it is displayed as provisional content distinct from the committed value.
2. **Given** an active composition, **When** it is committed, **Then** it becomes committed text exactly once and is recorded as one reversible edit.
3. **Given** an active composition, **When** it is cancelled or the editor becomes read-only or disabled, **Then** the original committed value and selection are restored.
4. **Given** platform text ranges, **When** the input method queries or replaces text, **Then** offsets are converted without splitting encoded characters.

---

### User Story 5 - Configure and Scroll the Editor (Priority: P2)

As an application developer, I can configure value, placeholder, enabled, read-only, wrapping preference, and theme while users can scroll documents taller than the viewport.

**Why this priority**: Reuse requires explicit state controls and a usable viewport for multi-line content.

**Independent Test**: Configure two independent editors, exercise read-only and disabled permissions, and navigate and scroll content beyond a fixed viewport.

**Acceptance Scenarios**:

1. **Given** an empty editor with a placeholder, **When** it is rendered, **Then** the placeholder is visible but is not part of the value or selection.
2. **Given** a read-only editor, **When** the user navigates, selects, scrolls, or copies, **Then** those operations remain available while mutations are rejected.
3. **Given** a disabled editor, **When** the user attempts pointer focus or editing commands, **Then** it does not accept normal interaction or mutate its state.
4. **Given** content taller than the viewport, **When** the user scrolls or drags the vertical scroll thumb, **Then** another valid portion of the logical-line content becomes visible.
5. **Given** navigation or editing moves the cursor outside the vertical viewport, **When** the operation completes, **Then** the editor adjusts its vertical offset to reveal the cursor.
6. **Given** a completed user edit, **When** the value changes, **Then** the host receives one change event containing the committed value; programmatic replacement and non-mutating actions emit no user-change event.
7. **Given** a focused, enabled editor with an empty selection, **When** it remains idle, **Then** its cursor blinks and resets to visible after interaction.

### Edge Cases

- Empty documents, consecutive blank lines, trailing line breaks, and the final empty line remain editable.
- Navigation and deletion at document boundaries are safe no-ops.
- Multi-code-point user-perceived characters are not split by horizontal navigation or deletion.
- Vertical movement clamps columns to valid character boundaries on the target logical line.
- Empty selections do not copy or cut content.
- Programmatic value replacement clears composition and edit history and places the cursor at the end.
- A viewport shorter than one text line cannot vertically clip the editor's text, selection, composition, or cursor.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: TextEditor MUST store and expose multi-line plain text with carriage-return line endings normalized to line feeds.
- **FR-002**: An enabled, non-read-only editor MUST support insertion, newline insertion, selection replacement, backward deletion, and forward deletion.
- **FR-003**: Horizontal navigation and deletion MUST use valid user-perceived character boundaries.
- **FR-004**: The editor MUST support character, word, logical-line start/end, adjacent logical-line, document start/end, and fixed ten-logical-line page movement.
- **FR-005**: Every supported navigation family MUST provide a selection-extending form.
- **FR-006**: The editor MUST support pointer placement, pointer drag selection, and Select All.
- **FR-007**: The editor MUST visually distinguish committed text, selection, provisional composition, placeholder, and the visible cursor where applicable.
- **FR-008**: The editor MUST support Copy, editable Cut and Paste, Undo, and Redo while preserving multi-line content.
- **FR-009**: Each completed value-changing user operation MUST produce one reversible snapshot and exactly one committed-value change event.
- **FR-010**: A new edit after Undo MUST discard the superseded redo sequence.
- **FR-011**: The editor MUST support input-method queries, provisional marked text, commit, and cancellation with correct encoded-range conversion.
- **FR-012**: The editor MUST provide a vertically scrollable viewport and a draggable vertical scroll thumb when logical-line content exceeds the visible height.
- **FR-013**: Cursor movement and editing MUST attempt to reveal the cursor vertically and keep the vertical offset within valid content bounds.
- **FR-014**: The host MUST be able to set and read value, placeholder, enabled state, read-only state, wrapping preference, current selection, scroll offset, caret visibility, and theme.
- **FR-015**: Programmatic value replacement MUST reset the cursor to the end, clear composition and edit history, and emit no user-change event.
- **FR-016**: Read-only mode MUST permit navigation, selection, scrolling, and copying but reject mutations and committed composition.
- **FR-017**: Disabled mode MUST reject normal focus acquisition, editing, navigation, selection, clipboard commands, and composition.
- **FR-018**: The cursor MUST blink only while the editor is focused, enabled, and has an empty selection; eligible interaction MUST restart it in the visible phase.
- **FR-019**: The rendered editor MUST preserve at least one full text line plus its normal vertical chrome under smaller height constraints.
- **FR-020**: TextEditor MUST remain reusable without application-specific business rules.

### Key Entities

- **Committed value**: The normalized multi-line plain text visible to the host; provisional composition and placeholder text are excluded.
- **Text position**: A valid UTF-8 byte boundary used by the cursor and selection.
- **Selection**: A normalized range plus direction indicating its active endpoint.
- **Composition**: Provisional input-method text and the original snapshot used for commit or cancellation.
- **Edit snapshot**: A committed value, selection, and direction stored for Undo or Redo.
- **Viewport**: The visible logical-line area and tracked vertical offset.
- **Interaction state**: The enabled, read-only, focused, wrapping-preference, and caret-visibility configuration of one editor.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Automated checks pass 100% of covered multi-line insertion, replacement, read-only rejection, and cursor-reveal scenarios.
- **SC-002**: Every tested user-originated value mutation emits exactly one change event, while programmatic and non-mutating operations emit zero.
- **SC-003**: In interaction testing, editing, navigation, selection, and scroll feedback becomes visible within 100 milliseconds for ordinary documents.
- **SC-004**: At least two independently configured editor instances coexist without sharing value, selection, history, or configuration state.
- **SC-005**: All tested Unicode navigation, deletion, and input-method ranges remain valid and preserve complete encoded characters.

## Assumptions

## As-Built Alignment

- TextEditor remains a reusable editable control; the read-only Git diff view is a separate DiffEditor and does not change TextEditor editing semantics.
- Keyboard bindings, scrolling, selection, undo/redo, and IME behavior remain owned by the reusable widget implementation.

- The synchronized implementation lays out logical lines without soft-wrapping them. The wrapping preference is publicly configurable, but visual soft wrapping and wrapped-line navigation are outside the current feature scope.
- Vertical movement preserves a logical byte column rather than a measured visual x-position; Page Up and Page Down move at most ten logical lines rather than one measured viewport height.
- Conventional word-selection pointer gestures, pointer-drag autoscroll, horizontal wheel scrolling, and optimized rendering for million-character documents are outside the synchronized scope.
- Horizontal content may be shifted to reveal the cursor, but a user-operated horizontal scrollbar is not part of the current feature.
- Rich text, syntax highlighting, diagnostics, search/replace, file persistence, and collaborative editing are outside scope.
- The widget uses the reusable project widget library and the UI foundation required by the project constitution.
