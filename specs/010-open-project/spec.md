# Feature Specification: Open Project

**Feature Branch**: `[010-open-project]`
**Created**: 2026-09-07
**Status**: Draft

**Input**: User description: "메뉴의 Open을 활성화하고 프로젝트명과 폴더를 입력받아 git 저장소를 프로젝트 탭으로 연다."

## User Scenarios & Testing

### User Story 1 - Open a Git Project (Priority: P1)

As a user, I can choose File > Open, enter a project name and repository folder, and add that repository as a project tab.

**Independent Test**: Open the dialog, provide a valid Git repository and name, confirm, and verify a new tab shows the name and repository content.

**Acceptance Scenarios**:

1. **Given** the File menu is open, **When** the user selects Open ..., **Then** an overlay dialog appears with project name, folder, browse, confirm, and cancel controls.
2. **Given** valid name and Git folder, **When** the user confirms, **Then** the dialog closes and a tab with the entered name and selected folder is added and activated.

### User Story 2 - Choose a Folder (Priority: P1)

As a user, I can use Browse to select a folder through the operating system folder chooser.

**Independent Test**: Open the dialog, use Browse, select a folder, and verify its path appears in the folder field.

**Acceptance Scenarios**:

1. **Given** the open dialog is visible, **When** the user presses Browse and selects a folder, **Then** the selected path is shown in the folder input.
2. **Given** the chooser is cancelled, **When** it returns, **Then** the dialog remains open and existing input is unchanged.

### User Story 3 - Recover from Invalid Input (Priority: P2)

As a user, I receive a clear message when the name or folder is invalid.

**Independent Test**: Confirm with an empty name, nonexistent folder, or non-Git folder and verify no tab is added and an error is shown.

**Acceptance Scenarios**:

1. **Given** required input is missing or the folder is not a Git repository, **When** the user confirms, **Then** a recoverable validation message is shown and the dialog remains open.
2. **Given** the dialog is visible, **When** the user cancels, **Then** it closes without changing open projects.

## Edge Cases

- Selecting an already-open repository must not create a duplicate tab.
- A folder path containing spaces or non-ASCII characters must be accepted.
- A chooser failure must leave the dialog usable and report the failure.

## Requirements

### Functional Requirements

- **FR-001**: The File > Open ... menu item MUST be enabled.
- **FR-002**: Selecting Open ... MUST show a modal overlay with project name and folder text inputs, a folder browse control, and Confirm/Cancel controls.
- **FR-003**: Browse MUST invoke the operating system folder chooser and populate the folder input with the selected path.
- **FR-004**: Confirm MUST require a non-empty name and an existing folder containing a Git repository.
- **FR-005**: A valid confirmation MUST add one project tab using the entered name and folder path and activate it.
- **FR-006**: Invalid confirmation MUST show a user-visible recoverable error without adding a tab or crashing.
- **FR-007**: Cancel and Escape MUST close the overlay without changing project state.
- **FR-008**: The feature MUST prevent duplicate tabs for the same repository path.

### Key Entities

- **Open Project Form**: Temporary project name, folder path, and validation message entered in the dialog.
- **Project Tab**: An open repository identified by its path and displayed under the chosen name.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Users can open a valid repository in a new tab in under 30 seconds after selecting Open.
- **SC-002**: 100% of valid confirmations create exactly one active tab with the requested name and path.
- **SC-003**: 100% of invalid confirmations leave existing tabs unchanged and show an actionable message.
- **SC-004**: Users can cancel the dialog without changing open projects in every tested interaction.

## Assumptions

- The existing project store and project page are reused for the newly opened tab.
- The user provides the project name explicitly; automatic naming is outside this feature's scope.
- Folder selection is synchronous from the user's perspective; cancelling the native chooser is non-destructive.
