# Feature Specification: File Status

**Feature Branch**: `[009-file-status]`
**Created**: 2026-09-07
**Status**: Draft

## User Scenarios & Testing

### User Story 1 - Review Staged and Unstaged Changes (Priority: P1)

As a user, I can see staged files, unstaged files, and the selected file's diff in one split view.

**Acceptance Scenarios**:

1. File Status first divides the view horizontally into a left workflow column and a right DiffEditor column.
2. The left column contains Staged Changes and Unstaged Changes stacked vertically.
3. Selecting a changed file displays its diff in the right-side DiffEditor.
4. The commit panel spans the full layout width below both columns.
5. Empty staged or unstaged states are clearly shown.

### User Story 2 - Prepare a Commit (Priority: P1)

As a user, I can enter a commit message and activate a Commit action from the File Status view.

**Acceptance Scenarios**:

1. The commit panel shows the author in `Author <Email>` format at the top.
2. A multi-line Commit Message editor is shown below the author line.
3. The panel provides checkboxes for `Push immediately after commit` and `Amend last commit`.
4. The Commit action is aligned to the right below the options.
5. The action validates or reports missing message/status as appropriate and does not lose the current view state.

### User Story 3 - Manage Working-Tree State (Priority: P2)

As a user, I can stage and unstage files and see the lists and diff update after repository operations.

**Acceptance Scenarios**:

1. Staging a file moves it from unstaged to staged.
2. Unstaging a file moves it from staged to unstaged.
3. After a successful commit, the lists refresh and the committed changes leave the working tree state.

## Functional Requirements

- **FR-001**: File Status MUST first split horizontally into a left workflow column and a right DiffEditor column, with the DiffEditor receiving the larger two-part area.
- **FR-001a**: The left workflow column MUST split vertically into Staged Changes and Unstaged Changes panels.
- **FR-001b**: The commit message input and Commit action MUST be rendered in a full-width footer below the left and right columns.
- **FR-002**: Staged and unstaged file lists MUST remain visually distinct and show empty states.
- **FR-003**: Selecting a file MUST update the adjacent DiffEditor without losing the selected status context.
- **FR-004**: The full-width commit panel MUST show the author as `Author <Email>`.
- **FR-004a**: The commit panel MUST use the reusable multi-line TextEditor for Commit Message input.
- **FR-004b**: The commit panel MUST provide `Push immediately after commit` and `Amend last commit` checkboxes.
- **FR-004c**: The Commit action MUST be right-aligned below the message editor and checkboxes.
- **FR-005**: Commit MUST require a non-empty message and MUST present repository errors without panicking.
- **FR-006**: Stage, unstage, and commit operations MUST refresh status data after completion.
- **FR-007**: File Status MUST preserve project-page isolation and must not leak status or commit-message state between projects.
- **FR-008**: The left/right and staged/unstaged split boundaries MUST expose an 8px mouse hit area centered on the boundary, with 4px of effective trigger space on each side; this hit area MUST NOT consume layout space.
- **FR-008a**: Dragging either boundary MUST allow continuous resizing across the available container, limited only by the minimum usable size of the adjacent panels.
- **FR-008b**: Resize conversion MUST use the measured File Status container bounds and the drag start position; window-size constants or arbitrary scale factors MUST NOT be used.

## Success Criteria

- **SC-001**: Users can identify staged, unstaged, diff, and commit areas without assistance in usability review.
- **SC-002**: Selecting a file displays its diff without changing the staged/unstaged list ownership.
- **SC-003**: Successful stage, unstage, and commit operations update the displayed lists in 100% of integration cases.
- **SC-004**: Invalid commit attempts produce a recoverable user-visible message with zero process panics.

## Assumptions

- DiffEditor remains the reusable diff-rendering widget owned by feature 008.
- File Status owns repository status and commit workflows but reuses the existing project-page shell, TextEditor, and checkbox controls.
