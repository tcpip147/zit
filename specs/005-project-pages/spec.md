# Feature Specification: Project Pages

**Feature Branch**: `[005-project-pages]`

**Created**: 2026-09-06

**Status**: Draft

**Input**: User description: "Add project pages. Project metadata is managed globally, each page has a one-to-one relationship with a tab, all open projects appear as tabs at startup, and heavyweight data such as history is loaded lazily when its tab becomes active. A project means a Git repository."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Restore Open Project Tabs (Priority: P1)

As a user returning to the application, I see one tab and one page for every Git repository that was open, so I can resume work without reopening each project manually.

**Why this priority**: Restoring the user's workspace and maintaining a clear one-project-to-one-tab relationship is the foundation for all project-page interactions.

**Independent Test**: Start the application with multiple previously open repositories and verify that every valid repository is represented exactly once by a tab and its associated page before any project is selected manually.

**Acceptance Scenarios**:

1. **Given** three repositories were open when the prior session ended, **When** the application starts, **Then** three distinct project tabs are displayed and each tab is associated with exactly one project page.
2. **Given** the saved open-project list contains the same repository more than once, **When** the application starts, **Then** only one tab and one page are created for that repository.
3. **Given** no projects were previously open, **When** the application starts, **Then** no project tab is displayed and the application presents an empty-project state.

---

### User Story 2 - Activate a Project and Load Its Data (Priority: P1)

As a user, I can activate a project tab and view that repository's page while heavyweight project data is loaded only when I need it.

**Why this priority**: Deferring costly work keeps startup responsive while still making full repository information available on demand.

**Independent Test**: Start with at least two open projects, activate only one tab, and verify that the active page progresses through loading to ready while the never-activated page does not request heavyweight data.

**Acceptance Scenarios**:

1. **Given** multiple project tabs exist and none has loaded heavyweight data, **When** one tab becomes active, **Then** its page immediately shows a loading state and begins loading its repository history and other heavyweight data.
2. **Given** one project is active, **When** another tab remains inactive, **Then** the inactive project retains its globally available metadata without loading heavyweight data.
3. **Given** a project's heavyweight data loaded successfully once, **When** the user switches away and later returns, **Then** the existing loaded data is shown without an unnecessary duplicate load.
4. **Given** a project's heavyweight data is still loading, **When** the user switches to another tab, **Then** the new active page remains usable and the original load cannot replace or corrupt the new page's content.

---

### User Story 3 - Handle Unavailable and Failed Repositories (Priority: P2)

As a user, I receive a clear, recoverable project-specific status when a repository is missing, invalid, or cannot load, without losing access to other open projects.

**Why this priority**: Repositories can move or become unreadable between sessions; one failure must not prevent the rest of the workspace from opening.

**Independent Test**: Restore a mix of valid and unavailable repositories and force a history-load failure, then verify that valid pages work, errors stay scoped to their project, and a retry is available.

**Acceptance Scenarios**:

1. **Given** one saved repository path is unavailable and another is valid, **When** the application starts, **Then** the valid project remains usable and the unavailable project is identified without crashing the application.
2. **Given** heavyweight data loading fails for the active project, **When** the failure is reported, **Then** that page displays a meaningful error and an action to retry.
3. **Given** a load previously failed and the underlying problem is resolved, **When** the user retries, **Then** the page can reach the ready state without opening a duplicate tab.

### Edge Cases

- A repository is moved, deleted, becomes unreadable, or ceases to be a valid Git repository between sessions.
- Different path spellings, symbolic links, or case differences refer to the same repository.
- A project tab is closed while its heavyweight data is loading.
- A repository has no commits, an unusually large history, detached HEAD, or no currently checked-out branch.
- The active project is removed from the global project collection while its page is visible.
- A late completion or failure arrives from a load initiated for a page that has since been closed.
- Multiple activation events occur while a project's first load is already in progress.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST treat a project as a locally accessible Git repository with a stable identity independent of its display name.
- **FR-002**: The system MUST maintain a single application-wide collection of project metadata that is the authoritative source for open projects.
- **FR-003**: Project metadata MUST include, at minimum, stable project identity, repository location, display name, and current availability status.
- **FR-004**: The system MUST create exactly one project tab and exactly one retained project page for each open project.
- **FR-005**: Each project tab MUST remain associated with the same project page for the lifetime of that open-project entry.
- **FR-006**: The system MUST prevent duplicate open-project entries when two references resolve to the same repository.
- **FR-007**: At startup, the system MUST restore all projects recorded as open in the preceding session and display their tabs without waiting for heavyweight project data.
- **FR-008**: The system MUST restore the previously active project when it remains available; otherwise it MUST activate the first available restored project, or show the empty-project state when none exist.
- **FR-009**: Globally managed project metadata MUST be available to tab labels and project-page summaries without triggering heavyweight data loading.
- **FR-010**: Heavyweight project data, including commit history, MUST NOT begin loading for a restored page until that page first becomes active.
- **FR-011**: On first activation, a project page MUST enter a visible loading state and initiate at most one active heavyweight-data load for that project.
- **FR-012**: A successful heavyweight-data load MUST be retained for subsequent tab switches during the session.
- **FR-013**: Switching tabs MUST display only the page associated with the selected project and MUST NOT allow another project's delayed result to alter it.
- **FR-014**: Failures to inspect or load one repository MUST remain isolated to that project and MUST NOT block other project tabs or pages.
- **FR-015**: A failed active page MUST display the project-specific failure in understandable language and provide a retry action.
- **FR-016**: Retrying MUST reuse the existing project tab and page, and MUST transition that page through loading to either ready or failed.
- **FR-017**: Closing a project tab MUST remove its tab and page from the open-project collection and prevent obsolete load results from updating visible application state.
- **FR-018**: Changes to globally managed project metadata MUST be reflected consistently in the corresponding tab and page without creating a duplicate page.
- **FR-019**: The project page MUST distinguish at least four heavyweight-data states: not loaded, loading, ready, and failed.
- **FR-020**: Git repository inspection and history retrieval MUST comply with the project's approved Git backend foundation; any fallback outside that foundation requires a separately approved architectural exception.

### Key Entities

- **Project Metadata**: Lightweight, globally managed information identifying an open Git repository, including stable identity, location, display name, and availability. It exists independently of whether heavyweight data has loaded.
- **Project Page**: The retained user-facing workspace for one project. It has exactly one associated project and exactly one associated tab, and owns the presentation of that project's load state and data.
- **Project Tab**: The navigation representation of one open project. Selecting it activates its associated project page; closing it removes that open-project entry.
- **Project Data**: Heavyweight, lazily acquired repository information such as commit history. It belongs to one project and has a not-loaded, loading, ready, or failed state.
- **Open Project Session**: The persisted ordered set of projects that should be restored as tabs, together with the identity of the active project.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: With 20 previously open projects, all 20 tabs become visible and interactive within 2 seconds on a typical supported development machine, excluding repositories on unavailable storage.
- **SC-002**: Before a tab is first activated, zero heavyweight-data loads are initiated for its project.
- **SC-003**: Activating a project provides visible loading or ready feedback within 100 milliseconds of the user action.
- **SC-004**: Repeatedly switching among already loaded projects initiates no duplicate heavyweight-data loads during the same session.
- **SC-005**: In a startup set containing both valid and unavailable repositories, 100% of valid projects remain independently accessible despite failures in other projects.
- **SC-006**: For every open project, automated acceptance checks observe exactly one tab and one retained page throughout activation and switching.
- **SC-007**: In usability validation, at least 90% of participants can identify whether an active project is loading, ready, or failed and can retry a failed load without assistance.

## Assumptions

- The ordered list of open projects and the active project are preserved between normal application sessions.
- Lightweight metadata may be inspected during startup, but commit history and similarly expensive repository data are heavyweight and deferred.
- The first restored project becomes active when the previously active project cannot be restored; activating it may therefore start exactly one lazy load during startup.
- Heavyweight data is cached for the current application session. Detecting and refreshing external repository changes is outside this feature's scope.
- Opening new repositories through a picker or recent-project list is outside this feature's scope; this feature defines the model, restoration, page lifecycle, and tab behavior once projects are open.
- A closed project's in-flight work may finish internally, but its result is discarded and cannot recreate the project or update another page.
- Authentication, remote hosting services, network fetching, and repository mutation are outside this feature's scope.
