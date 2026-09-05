# Feature Specification: Platform-Specific Custom Title Bar and Menu Bar

**Feature Branch**: Not created; no Git repository or branch creation hook is configured.

**Created**: 2026-09-05

**Status**: In implementation

**Input**: User description (translated): "Open an application window with a title bar implemented by the application. Provide buttons with SVG icons identical to native controls for Windows, Linux, and macOS. Replace the title text with a menu bar. Initially provide File with New Repository, Open ..., Clone ..., and Exit, and Help with About. Use GPUI `text_xs` typography for the menu bar, a light title-bar-compatible background, and pointer cursors plus color feedback on menu-button hover."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Open and Move the Application Window (Priority: P1)

As a user, I can launch Zit and see a working application window with a custom title bar,
then move and resize it without opening a Git repository.

**Why this priority**: The application window is the foundation for the Git client.

**Independent Test**: Launch the application without a repository and verify its title bar,
content area, movement, and resizing.

**Acceptance Scenarios**:

1. **Given** the application is not running on a supported platform, **When** I launch it,
   **Then** one usable window appears with a File/Help menu bar in place of title text and a content area.
2. **Given** an open window, **When** I inspect its top edge, **Then** exactly one
   application-rendered title bar and one set of window controls are visible.
3. **Given** a normal window, **When** I drag an empty area of its title bar,
   **Then** the window moves without triggering a window control.
4. **Given** a normal window, **When** I drag its edges or corners,
   **Then** it resizes and the title bar and content follow the new dimensions.

---

### User Story 2 - Use Familiar Platform Window Controls (Priority: P1)

As a user, I can recognize and operate window controls that match my platform's native
appearance and placement, including the correct control for the current window state.

**Why this priority**: Native-looking SVG controls are the central requested behavior.

**Independent Test**: On each reference platform, compare the controls with native reference
screenshots and exercise every supported window action.

**Acceptance Scenarios**:

1. **Given** a Windows window, **When** I inspect its controls, **Then** minimize,
   maximize or restore, and close appear in that order on the right.
2. **Given** the Linux reference profile, **When** I inspect its controls, **Then** their
   icons and placement match the GNOME/Adwaita profile defined in Assumptions.
3. **Given** a macOS window, **When** I inspect its controls, **Then** close, minimize,
   and full-screen controls appear on the left with the reference colors and state-dependent symbols.
4. **Given** a normal Windows or Linux window, **When** I click maximize,
   **Then** the window maximizes and shows restore; clicking restore returns it to its previous size.
5. **Given** a normal macOS window, **When** I activate the full-screen control,
   **Then** the window enters full screen and can return to its previous windowed state.
6. **Given** an open window, **When** I minimize it and reactivate it through the operating system,
   **Then** the same window returns with controls matching its actual state.
7. **Given** an open window, **When** I click close, **Then** that window closes without
   triggering another control or a drag operation.
8. **Given** an open window, **When** I change its state through operating-system controls or
   shortcuts, **Then** the title bar reflects the actual state.

---

### User Story 3 - Use Controls Across Display and Input Conditions (Priority: P2)

As a user, I can identify and activate window controls across display scales, light and dark
appearances, and active and inactive windows, including through assistive technology.

**Why this priority**: Custom controls must remain usable in everyday display and input conditions.

**Independent Test**: Change display scale, appearance, and focus; inspect control states and
use accessibility tools to identify and activate the controls.

**Acceptance Scenarios**:

1. **Given** an open window, **When** it is displayed at supported 100%, 150%, and 200% scales,
   **Then** icons remain sharp, do not overlap or clip, and match their interactive areas.
2. **Given** an open window, **When** I switch light/dark appearance or active/inactive state,
   **Then** each control matches the corresponding reference appearance and remains identifiable.
3. **Given** a control, **When** I press it and release outside its interactive area,
   **Then** its action is canceled and its pressed appearance clears.
4. **Given** assistive technology is active, **When** I navigate to a window control,
   **Then** its button role and current action name are available and its enabled action is executable.
5. **Given** a narrow window, **When** the available width decreases,
   **Then** File, Help, and all window controls remain visible without overlap at the minimum window size.

---

### User Story 4 - Access File and Help Menus (Priority: P1)

As a user, I can open File and Help directly in the custom title bar to discover repository
commands, exit the application, and view application information.

**Why this priority**: The menu bar replaces the title text and provides the requested command entry points.

**Independent Test**: Open both menus with a pointer and keyboard, verify their exact labels and
order, dismiss them, view About, and invoke Exit in a separate application run.

**Acceptance Scenarios**:

1. **Given** an open window on any supported platform, **When** I inspect the title bar,
   **Then** File and Help appear in that order instead of visible application title text,
   alongside the existing platform-specific window controls.
2. **Given** the menu bar, **When** I open File, **Then** it lists New Repository,
   Open ..., Clone ..., and Exit in that order, with exactly those labels.
3. **Given** the menu bar, **When** I open Help, **Then** it contains About.
4. **Given** an open menu, **When** I press Escape, click outside, or the window loses focus,
   **Then** the menu closes without executing a command; Escape returns focus to its menu heading.
5. **Given** File is open, **When** I open Help, **Then** File closes and only Help remains open.
6. **Given** keyboard focus on the menu bar, **When** I use arrow keys and Enter,
   **Then** I can switch headings, open menus, navigate enabled items, and activate a selected item.
7. **Given** Help is open, **When** I activate About, **Then** a dismissible dialog shows
   the application name Zit and its version, and closing it returns me to the main window.
8. **Given** File is open, **When** I activate Exit, **Then** the application terminates,
   including on platforms where closing a window alone would leave the application running.
9. **Given** this initial menu-only scope, **When** I inspect the repository commands,
   **Then** New Repository, Open ..., and Clone ... are visibly disabled and cannot execute.
10. **Given** the File or Help heading or a submenu item, **When** I move the pointer over it,
    **Then** its background changes and the pointer uses the hand cursor without initiating a window drag.
11. **Given** either supported light or dark appearance, **When** the menu bar is rendered,
    **Then** it uses GPUI `text_xs` typography and a lightened surface coordinated with the title bar.

### Edge Cases

- Moving a window between monitors with different scales updates icon rendering and hit areas.
- At minimum window dimensions, File, Help, all window controls, and an empty draggable region remain available.
- Unsupported window actions are visibly disabled and cannot execute.
- If full screen hides the title bar, the operating system's full-screen exit mechanism remains usable.
- Non-reference Linux themes use the documented reference profile consistently without causing failure.
- Double-clicking the draggable title bar follows the reference environment's configured title-bar action;
  double-clicking a control or menu heading does not trigger that title-bar action.
- Window creation failure produces a user-visible error identifying the failure instead of leaving
  an unexplained running application with no usable window.

- Menu popups remain within the visible window or screen bounds and do not become clipped at screen edges.
- Clicking a menu heading or item never initiates window dragging or a title-bar double-click action.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The application MUST open one usable main window on Windows, Linux, and macOS
  without requiring a repository to be selected.
- **FR-002**: The application MUST render its own title bar containing a File/Help menu bar in place of visible title text and
  window controls, without duplicate operating-system title bars or controls.
- **FR-003**: Window control icon assets MUST be SVG. The application MUST automatically select
  the asset set for the running platform without requiring manual platform selection.
- **FR-004**: Controls MUST match the selected native reference profile in icon shape, dimensions,
  stroke weight, color, spacing, placement, and default, hover, pressed, and inactive appearance.
  Visual comparison MUST verify each attribute; only rasterization antialiasing differences are allowed.
- **FR-005**: Windows and Linux MUST provide minimize, maximize/restore, and close. macOS MUST
  provide close, minimize, and enter/exit full screen, with placement defined in the reference profiles.
- **FR-006**: Controls MUST change the actual window state and reflect changes initiated outside
  the title bar. Close MUST close the window; process termination follows platform application lifecycle.
- **FR-007**: Empty title-bar space MUST support window dragging. Normal windows MUST retain
  edge and corner resizing. Title-bar double-click MUST follow the reference environment's configured action.
- **FR-008**: Control interactions MUST remain separate from dragging and title-bar double-click actions.
  Releasing a pressed control outside its interactive area MUST cancel its action.
  Unsupported actions MUST be visibly disabled and prevented from executing.
- **FR-009**: Controls MUST update for light/dark appearance, active/inactive state, and supported
  display-scale changes. The layout MUST preserve menu headings, window controls, their interactive areas,
  and an empty draggable region at the minimum window size.
- **FR-010**: Each control MUST expose its button role and current action name to assistive technology,
  permit accessible activation when enabled, and preserve operating-system window-control shortcuts.
- **FR-011**: Window creation failure MUST produce a user-visible error identifying the failure.

- **FR-012**: The custom title bar MUST display File followed by Help on all three platforms,
  replacing visible title text. On macOS the menu bar MUST sit after the left-side window controls;
  on Windows and Linux it MUST sit before the right-side controls without overlapping them.
- **FR-013**: File MUST contain exactly New Repository, Open ..., Clone ..., and Exit in that
  order. Help MUST contain About. Labels MUST preserve the specified capitalization and spaces.
- **FR-014**: Menus MUST support pointer activation, keyboard focus, arrow-key navigation,
  Enter activation, and Escape dismissal. Only one top-level menu popup may be open at a time.
  Clicking outside or losing window focus MUST dismiss the popup without activating an item.
- **FR-015**: Menu headings and items MUST expose their accessible names, menu roles, and
  expanded or disabled states where applicable. Menu interactions MUST NOT move or maximize the window.
- **FR-016**: Exit MUST terminate the application. About MUST open a dismissible dialog showing
  the application name and version and MUST return focus to the main window when dismissed.
- **FR-017**: In this initial scope, New Repository, Open ..., and Clone ... MUST be present but
  disabled until their workflows are implemented. They MUST NOT report a successful repository operation.
- **FR-018**: The menu bar and popup menu text MUST use GPUI's `text_xs` typography.
- **FR-019**: The title bar and popup menus MUST use lightened, appearance-aware surfaces with
  sufficient foreground and border contrast in both light and dark operating-system appearances.
- **FR-020**: File, Help, and every submenu row MUST use a hand pointer and visible background-color
  feedback on hover. Disabled repository rows remain non-executable even though they provide hover feedback.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In 10 launches per reference platform, all 10 display exactly one usable main window
  with one title bar and one set of controls.
- **SC-002**: All applicable minimize/reactivate, maximize/restore, full-screen enter/exit, move,
  resize, double-click, externally initiated state-change, and close scenarios pass on every reference platform.
- **SC-003**: All visual attributes listed in FR-004 match reference screenshots across default,
  hover, pressed, and inactive states in light and dark appearances, with zero missing control states.
- **SC-004**: At minimum window dimensions and supported 100%, 150%, and 200% display scales,
  there are zero clipped icons, overlapping controls, or mismatches between controls and interactive areas.
- **SC-005**: Every control exposes its role and current action name, and all enabled control
  actions can be completed through assistive technology.
- **SC-006**: A simulated window creation failure consistently produces an explanatory user-visible
  error; unsupported actions and canceled presses cause zero unintended window-state changes.

- **SC-007**: On each reference platform, File and Help and all five submenu items match the specified
  labels and order, and all Story 4 scenarios pass using their applicable pointer or keyboard interactions.
- **SC-008**: At all supported validation scales and minimum dimensions, there are zero clipped menu
  headings or popup items, overlaps with window controls, or unintended window actions from menu input.
- **SC-009**: File, Help, and all five submenu rows expose the specified pointer and hover feedback,
  and all menu text resolves to the GPUI `text_xs` size in light and dark appearance checks.

## Assumptions

- Scope is the main window, custom title bar, File/Help menus, Exit, and About. Repository creation,
  opening, and cloning workflows, Git changes, commits, tabs, and search are outside this feature.
  The three repository menu entries are initially disabled; this is an explicit scope assumption.
  The content area may be empty. The operating-system window title may remain Zit for task switching
  and accessibility, but the custom title bar displays the menu bar instead of that text.
- The menu bar is inside the application window on every platform, including macOS. Integration with
  the operating system's global application menu is not required by this feature.
- Proposed visual references are Windows 11 default window controls, macOS default window
  controls, and GNOME/Adwaita with minimize, maximize, and close enabled in that order on the right.
  Reproducing every Linux desktop, user theme, or customized button arrangement is outside this feature.
- Planning MUST fix the exact OS/desktop versions, appearance settings, display scales, and native
  reference screenshots used for validation. Screenshots from different versions MUST NOT be mixed.
- Modifier-key variants of macOS controls, platform-specific popup menus, and custom recreation of
  snap suggestion interfaces are outside scope. Basic window actions and existing operating-system
  interaction mechanisms remain available.
- Planning will select minimum window dimensions that preserve the menu bar, all controls, and a draggable region, and record which requested
  display scales each reference environment supports. Unsupported scales are explicitly recorded in validation.
- Implementation MUST comply with Constitution v1.2.0, including the official Zed GPUI foundation,
  the gix Git implementation layer, and English specification prose. SVG is a user-mandated asset format;
  rendering APIs, module layout, and dependency configuration belong in the implementation plan.
- The `zit` executable starts the application directly through `gpui_platform::application()`.
  Title-bar and menu code belongs to `crates/zit`; this feature does not introduce a
  `window_shell` crate or another application bootstrap layer.
- Final validation requires executable environments for all three platforms and the fixed reference images.
- The current Windows implementation maximizes through GPUI `zoom_window()`. Because the pinned GPUI
  Windows backend only issues `SW_MAXIMIZE`, restoration uses a target-only Win32 `SW_RESTORE` adapter
  obtained from GPUI's raw window handle. This is a bounded exception for a capability missing from GPUI.



