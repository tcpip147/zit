# Feature Specification: New Repository

**Feature Branch**: `[011-new-repository]`
**Created**: 2026-09-08
**Status**: Draft
**Input**: User description: "Enable New Repository in the menu bar and create a new Git repository from a project name and folder path."

## User Scenarios & Testing

### User Story 1 - Create a Repository (Priority: P1)

As a user, I can choose File > New Repository, enter a project name and parent folder, and create a new Git repository that is immediately opened as the active project.

**Independent Test**: Submit valid values and verify a new folder containing a Git repository is created and opened in one active project tab.

**Acceptance Scenarios**:

1. **Given** the File menu is open, **When** the user selects New Repository, **Then** the new-repository dialog opens with name, folder, Create, and Cancel controls.
2. **Given** a valid project name and writable parent folder, **When** the user selects Create, **Then** a child folder is created, initialized as a Git repository, persisted as a project, and activated.
3. **Given** the user cancels or presses Escape, **When** the dialog closes, **Then** no folder or project is created.

### User Story 2 - Choose a Destination (Priority: P1)

As a user, I can browse for the parent folder and retain my entered project name when the chooser is cancelled.

**Independent Test**: Open the chooser, select a folder, cancel it, and verify the form remains usable with its values intact.

**Acceptance Scenarios**:

1. **Given** the dialog is open, **When** the user selects Browse and chooses a folder, **Then** the folder field is populated.
2. **Given** the chooser is open, **When** the user cancels it, **Then** the dialog remains open and existing form values are unchanged.

### User Story 3 - Recover from Invalid Input (Priority: P2)

As a user, I receive an actionable error while the dialog remains open when the name or destination is invalid, unavailable, or already occupied.

**Independent Test**: Try blank input, an invalid name, a missing parent, an existing destination, and an unwritable folder.

**Acceptance Scenarios**:

1. **Given** a required value is missing or the name is invalid, **When** the user selects Create, **Then** no filesystem change occurs and an actionable error is shown.
2. **Given** the destination folder already exists, **When** the user selects Create, **Then** creation is rejected without overwriting it.
3. **Given** repository initialization fails, **When** the user selects Create, **Then** the error is shown and any safely removable partial directory is cleaned up.

## Requirements

### Functional Requirements

- **FR-001**: The File menu MUST enable and dispatch New Repository.
- **FR-002**: The dialog MUST collect a non-empty trimmed project name and an existing parent folder.
- **FR-003**: The system MUST initialize the selected folder itself without creating a child folder or overwriting existing files.
- **FR-004**: The system MUST create a `.git` directory inside the selected folder using the application Git backend.
- **FR-005**: A successful creation MUST add, persist, and activate the new project.
- **FR-005a**: After successful creation, the new project's History view MUST begin loading immediately and show the repository's initial empty or loaded state.
- **FR-006**: Cancellation MUST make no project or filesystem change.
- **FR-007**: Invalid input and creation failures MUST remain recoverable and preserve entered values.
- **FR-008**: The dialog MUST expose accessible labels and controls for name, folder, Browse, Create, and Cancel.
- **FR-009**: After a successful commit, the active project's History MUST reload and include the newly created commit when it is reachable.

### Key Entities

- **New Repository Request**: Project name and parent folder supplied by the user.
- **Created Repository**: The newly initialized child folder and its project metadata.

## Success Criteria

### Measurable Outcomes

- **SC-001**: A valid repository is created and opened within one user submission in 100% of supported local-folder cases.
- **SC-002**: Invalid or conflicting requests produce no overwritten files in 100% of acceptance cases.
- **SC-003**: Users can complete creation in under one minute after opening the dialog.
- **SC-004**: Cancellation produces zero new project tabs and zero new repository folders.

## Assumptions

- The selected folder is the repository directory; the project name is used only as its display name.
- Existing files and existing `.git` directories are never overwritten.
- Git initialization is local and does not create an initial commit.
- The existing project persistence, activation, and folder chooser flows are reused.
