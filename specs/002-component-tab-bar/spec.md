# Feature Specification: Component Tab Bar

**Feature Branch**: Not created; no branch-creation hook is configured.
**Created**: 2026-09-05
**Status**: Implemented and synchronized with the current component
**Input**: User description (translated): "Add a reusable tab bar in which each tab owns a retained page, selecting a tab navigates to that page, and each tab displays a project icon, project name, and close control."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Switch Between Retained Pages (Priority: P1)

As a user, I can select a project tab and immediately see its associated page without recreating that page.

**Why this priority**: Selection and retained page navigation are the component's primary purpose.

**Independent Test**: Create three tabs with distinct page identities, select each tab repeatedly, and verify that only the matching retained page is visible.

**Acceptance Scenarios**:

1. **Given** multiple tabs, **When** the user selects an inactive tab, **Then** it becomes active and its associated page replaces the previously visible page.
2. **Given** an active tab, **When** it is selected again, **Then** it remains active without recreating its page.
3. **Given** no valid initial active identity, **When** a non-empty tab bar is created, **Then** the first tab becomes active.
4. **Given** duplicate stable identities, **When** construction or insertion is attempted, **Then** the duplicate is rejected without creating ambiguous navigation state.

---

### User Story 2 - Identify and Close Tabs (Priority: P1)

As a user, I can identify a project tab by icon and name and close it with a dedicated close control.

**Why this priority**: Predictable identity and closure are required to manage multiple project pages.

**Independent Test**: Close inactive, active, and final tabs and verify the remaining order, active identity, page identity, and emitted close events.

**Acceptance Scenarios**:

1. **Given** a tab, **When** it is displayed, **Then** it shows an SVG project icon, project name, and close button in that order.
2. **Given** an empty icon reference, **When** the tab is displayed, **Then** the bundled fallback project icon is used.
3. **Given** an inactive tab, **When** it is closed, **Then** the active page does not change.
4. **Given** an active tab, **When** it is closed, **Then** the right neighbor becomes active when present, otherwise the left neighbor becomes active, or no page remains when the final tab closes.
5. **Given** the host rejects a close request, **When** the close control is activated, **Then** the tab and active state remain unchanged.
6. **Given** tab metadata changes, **When** its name or icon is updated, **Then** the same retained page remains associated with the tab.

---

### User Story 3 - Reach Tabs in an Overflowing Row (Priority: P2)

As a user, I can reach tabs beyond the visible width through the row's horizontal scrolling behavior.

**Why this priority**: Fixed-width tabs must remain usable when many projects are open.

**Independent Test**: Display enough tabs to overflow, use supported horizontal scrolling input, and programmatically select tabs outside the current viewport.

**Acceptance Scenarios**:

1. **Given** tabs exceed the row width, **When** the user provides horizontal scrolling input, **Then** the row moves within its available bounds.
2. **Given** an off-screen tab is selected, **When** selection completes, **Then** the row requests that tab be revealed.
3. **Given** all tabs fit, **When** no overflow exists, **Then** the row remains at its normal position.

### Edge Cases

- Duplicate project names remain valid because stable tab identities are independent of labels.
- Long names are visually clipped without overlapping the close control, while the complete name remains the tab's accessible label.
- Closing a scrolled tab retains a valid remaining active identity and scroll position.
- Rapid repeated selection of the active tab does not duplicate events or pages.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The component MUST represent an ordered collection of zero or more tabs.
- **FR-002**: Every tab MUST have a unique stable identity, one retained page, a project name, an SVG icon reference, and a close control.
- **FR-003**: Duplicate stable identities MUST be rejected during construction and insertion.
- **FR-004**: At most one tab MUST be active, and only the active tab's page MUST be displayed.
- **FR-005**: A missing or unknown initial active identity MUST fall back to the first tab when one exists.
- **FR-006**: Selecting a tab MUST preserve its page identity, reveal the tab in the row, and emit a selection change only when the active identity changes.
- **FR-007**: Closing an inactive tab MUST leave the active identity unchanged.
- **FR-008**: Closing the active tab MUST select the right neighbor when available, otherwise the left neighbor; closing the last tab MUST leave the component empty.
- **FR-009**: The component MUST emit a close request before removal and MUST permit the host to veto removal.
- **FR-010**: Activating the close control MUST NOT also select the tab.
- **FR-011**: Updating a tab's name or icon MUST preserve its stable identity and retained page.
- **FR-012**: An empty icon reference MUST use the bundled fallback project SVG.
- **FR-013**: Tabs MUST use fixed readable space, extra-small text, clipping for long labels, and separate hoverable pointer targets for the tab and close control.
- **FR-014**: Overflowing tabs MUST be contained in a horizontally scrollable row, and selection MUST request the active item be scrolled into view.
- **FR-015**: The tab row, each tab, and each close control MUST expose appropriate roles, accessible names, ordered position, item count, and selected state where applicable.
- **FR-016**: The component MUST remain independent of repository operations, page layout, and global application state.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Contract tests preserve the same page identity across 100% of repeated selection and metadata-update checks.
- **SC-002**: Active and inactive close tests at the beginning, middle, and end remove exactly one tab and choose the specified neighbor in 100% of cases.
- **SC-003**: Duplicate identities are rejected in 100% of construction and insertion checks.
- **SC-004**: Selection, closure, and metadata feedback is visible within 100 milliseconds under normal desktop use.
- **SC-005**: Every rendered tab exposes its full name, ordered position, total count, and correct selected state.

## Assumptions

- Pointer selection and close activation are in scope. Dedicated keyboard navigation bindings and assistive-technology click handlers are not present in the synchronized component scope.
- The host is responsible for supplying valid non-empty custom asset paths; automatic fallback applies only to an empty icon reference.
- Horizontal scrolling uses the platform's supported horizontal scrolling behavior. Explicit conversion of vertical wheel input into horizontal movement is outside the synchronized scope.
- Reordering, pinning, preview tabs, drag-and-drop, context menus, badges, dirty indicators, and reopening closed tabs are outside scope.
- The component uses the project's theme values but cross-platform light/dark visual certification is outside this feature's automated contract.
