# Feature Specification: Custom Application Chrome

**Feature Branch**: Not created; no branch-creation hook is configured.

**Created**: 2026-09-05

**Status**: Implemented on the current source; native visual validation remains pending

**Input**: User description (translated): "Open an application window with a title bar implemented by the application. Provide platform-appropriate SVG window controls, replace the title text with File and Help menus, and provide basic application commands."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Use the Application Window (Priority: P1)

As a user, I can launch Zit in a decoration-free window, move it from its custom title bar, resize it, and use controls appropriate to my platform.

**Why this priority**: The application window is the foundation for every other feature.

**Independent Test**: Launch Zit, inspect the single custom title bar, drag and resize the window, and exercise each enabled window control.

**Acceptance Scenarios**:

1. **Given** Zit is launched, **When** the main window opens, **Then** it has one application-rendered title bar, a content area, an initial size of 1024 by 720 logical pixels, and a minimum size of 480 by 320 logical pixels.
2. **Given** an empty area of the title bar, **When** the user drags it, **Then** the operating system moves the window without activating a menu or control.
3. **Given** Windows or Linux, **When** the title bar is displayed, **Then** minimize, maximize or restore, and close controls appear on the right.
4. **Given** macOS, **When** the title bar is displayed, **Then** close, minimize, and full-screen controls appear on the left.
5. **Given** an enabled control, **When** the user activates it by pointer or an accessible click action, **Then** the corresponding window action is requested.
6. **Given** an unsupported window capability, **When** its control is displayed, **Then** it is visibly subdued and cannot execute.

---

### User Story 2 - Use File and Help Menus (Priority: P1)

As a user, I can open File and Help from the title bar, discover available commands, exit Zit, and view application information.

**Why this priority**: The menus replace title text and provide the shell's command entry points.

**Independent Test**: Open both menus with pointer and keyboard commands, verify item order and availability, dismiss them, open About, and invoke Exit separately.

**Acceptance Scenarios**:

1. **Given** the custom title bar, **When** it is displayed, **Then** File and Help appear in that order.
2. **Given** File is open, **When** its contents are inspected, **Then** New Repository, Open ..., Clone ..., and Exit appear in that order; the first three are disabled and Exit is enabled.
3. **Given** Help is open, **When** its contents are inspected, **Then** About is available.
4. **Given** one menu is open, **When** the other heading is selected, Escape is pressed, the window loses activation, or the user clicks outside, **Then** the previous menu closes without an unintended command.
5. **Given** keyboard menu control is active, **When** Left, Right, Up, Down, Enter, or Escape is invoked, **Then** the open heading or highlighted enabled command changes consistently.
6. **Given** About is activated, **When** the dialog appears, **Then** it identifies Zit and its current package version and can be dismissed.
7. **Given** Exit is activated, **When** the command is dispatched, **Then** the application requests termination.

---

### User Story 3 - Recover From Main-Window Startup Failure (Priority: P2)

As a user, I receive a visible explanation if the main application window cannot be created.

**Why this priority**: A silent background process gives the user no way to understand startup failure.

**Independent Test**: Trigger the supported simulated startup failure and verify that a smaller explanatory error window appears or the process exits if even that window cannot open.

**Acceptance Scenarios**:

1. **Given** main-window creation fails, **When** Zit handles the failure, **Then** it attempts to open a visible error window containing the failure explanation.
2. **Given** both the main and error windows fail to open, **When** startup handling completes, **Then** Zit terminates instead of remaining invisibly active.

### Edge Cases

- Window capabilities may differ by platform or current window state.
- Reopening the application while no windows remain creates a new main window.
- Menu and control pointer presses do not initiate title-bar dragging.
- Disabled repository commands remain non-executable even when highlighted by pointer movement.
- Native appearance, scale behavior, and cross-platform interaction still require validation on each target platform.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Zit MUST open a decoration-free main window with one application-rendered title bar and a content area.
- **FR-002**: The main window MUST initially use 1024 by 720 logical pixels and MUST enforce a minimum size of 480 by 320 logical pixels.
- **FR-003**: The title bar MUST provide a dedicated draggable region that does not overlap menu headings or window controls.
- **FR-004**: Window-control icons MUST be SVG assets selected for Windows, Linux, or macOS according to the running platform.
- **FR-005**: Windows and Linux MUST show minimize, maximize or restore, and close controls on the right; macOS MUST show close, minimize, and full-screen controls on the left.
- **FR-006**: Enabled controls MUST dispatch the corresponding actual window command, and unavailable capabilities MUST be prevented from executing.
- **FR-007**: The maximize control MUST present the restore action while the window is maximized.
- **FR-008**: Window controls MUST expose button roles and action names and MUST accept accessible click activation when enabled.
- **FR-009**: The title bar MUST display File followed by Help instead of visible title text.
- **FR-010**: File MUST contain New Repository, Open ..., Clone ..., and Exit in order; Help MUST contain About.
- **FR-011**: New Repository, Open ..., and Clone ... MUST remain disabled in this feature; Exit MUST terminate the application and About MUST show the application name and version.
- **FR-012**: Only one menu popup MUST be open at once, and Escape, outside click, or window deactivation MUST dismiss it without executing a command.
- **FR-013**: Menu headings and rows MUST provide pointer hover feedback, and enabled rows MUST dispatch only their associated command.
- **FR-014**: The shell MUST support directional and Enter/Escape menu commands for switching headings, moving among enabled items, activating the selection, and dismissing overlays.
- **FR-015**: Main-window creation failure MUST produce a visible explanatory error window when possible and MUST otherwise terminate the application.
- **FR-016**: When application activation occurs with no open windows, Zit MUST attempt to reopen the main window.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Automated shell checks confirm that all unavailable repository commands are rejected and every implemented window command is recognized.
- **SC-002**: In 100% of manual checks on a supported host, enabled controls cause only their named window action and interactive title-bar elements do not start a drag.
- **SC-003**: File and Help expose exactly five total menu rows with the specified labels, ordering, and enabled state.
- **SC-004**: The simulated main-window failure produces a visible explanation in 100% of runs where the fallback window can be created.
- **SC-005**: Pointer and keyboard menu-state changes provide visible feedback within 100 milliseconds under normal desktop use.

## Assumptions

- Repository creation, opening, and cloning workflows are outside this feature and their menu rows are placeholders.
- Platform-specific SVGs are project-owned approximations; exact native visual equivalence is not claimed until reference-image validation is completed.
- The current shell uses a light visual palette. Automatic dark-appearance styling and multi-scale visual certification are outside the synchronized implementation scope.
- Operating-system resize behavior is retained by the windowing system; this feature defines the custom top chrome and commands.
- Implementation complies with the project's GPUI, Rust workspace, and executable-entry-point constitutional constraints.

