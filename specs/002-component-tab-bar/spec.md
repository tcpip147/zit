# Feature Specification: Component Tab Bar

**Feature Branch**: Not created; no branch-creation hook is configured.
**Created**: 2026-09-05
**Status**: Draft
**Input**: User description (translated): "Add a tab bar to component. The tab bar owns tabs, each tab owns its page, and selecting a tab navigates to that page, similar to the VS Code editor. Each tab contains an SVG image, a project name, and an X-image close button. When tabs fill and overflow the available horizontal width, the mouse wheel scrolls them horizontally. Tab-bar text uses text_xs."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Switch Between Project Pages (Priority: P1)

As a user working with several projects, I can select a project tab and immediately see the page associated with it.

**Why this priority**: Selecting a tab and displaying its page is the component's primary purpose.
**Independent Test**: Create three tabs with distinct pages, select each tab, and verify that exactly one tab is active and only its page is shown.

**Acceptance Scenarios**:

1. **Given** multiple tabs, **When** I select an inactive tab, **Then** it becomes active and its page replaces the previously visible page.
2. **Given** an active tab, **When** I select it again, **Then** it remains active without duplicating or recreating its page.
3. **Given** a populated tab bar, **When** it first appears, **Then** exactly one associated page is visible.

---

### User Story 2 - Identify and Close Tabs (Priority: P1)

As a user, I can identify a tab by its project icon and name and close it with its X button.

**Why this priority**: Clear identity and predictable closure are essential for managing multiple pages.
**Independent Test**: Inspect each tab's icon, name, and close control; close inactive and active tabs and verify selection and page results.

**Acceptance Scenarios**:

1. **Given** any tab, **When** I inspect it, **Then** it contains an SVG project image, project name, and X-image close button in a consistent order.
2. **Given** an inactive tab, **When** I close it, **Then** only that tab and page are removed and the active page remains unchanged.
3. **Given** an active tab with a right neighbor, **When** I close it, **Then** the right neighbor becomes active and its page is shown.
4. **Given** an active tab without a right neighbor, **When** I close it, **Then** the nearest left neighbor becomes active and its page is shown.
5. **Given** the only tab, **When** I close it, **Then** the tab bar and page area show an empty state.
6. **Given** a close button, **When** I activate it, **Then** it closes the tab without selecting the tab or starting a scroll gesture.

---

### User Story 3 - Reach Overflowing Tabs (Priority: P2)

As a user with many open projects, I can scroll the tab row horizontally with the mouse wheel so every tab remains reachable.

**Why this priority**: Overflow handling supports realistic multi-project workloads.
**Independent Test**: Add enough tabs to overflow, wheel-scroll in both directions, and select tabs initially outside the viewport.

**Acceptance Scenarios**:

1. **Given** overflowing tabs, **When** I rotate the mouse wheel over the row, **Then** the row scrolls horizontally in the corresponding direction.
2. **Given** either scroll boundary, **When** I continue outward, **Then** scrolling remains within the content bounds.
3. **Given** an off-screen tab becomes active through host state, **When** selection changes, **Then** the active tab is brought fully into view.
4. **Given** all tabs fit, **When** I rotate the wheel, **Then** the row does not shift unnecessarily.

---

### User Story 4 - Operate Tabs Accessibly (Priority: P2)

As a keyboard or assistive-technology user, I can identify, select, and close tabs and understand which tab is active.

**Why this priority**: The primary navigation control must provide equivalent input paths.
**Independent Test**: Navigate without a pointer, select and close tabs, and inspect names, positions, and selected states.

**Acceptance Scenarios**:

1. **Given** focus in the tab bar, **When** I navigate and select a tab, **Then** focus and the visible page follow it.
2. **Given** assistive technology, **When** I inspect a tab, **Then** its name, role, position, and selected state are available.
3. **Given** focus on a close control, **When** I activate it, **Then** normal close behavior occurs and focus moves to a valid remaining tab.

### Edge Cases

- Duplicate project names do not affect stable tab identity.
- Empty and long names do not overlap the close button; complete names remain available non-visually.
- A missing or invalid SVG uses a consistent fallback while preserving all tab actions.
- Rapid selection and closure never show a page whose tab was removed.
- Resizing recalculates overflow without changing the active tab.
- Wheel and trackpad input over an overflowing row produce bounded horizontal movement.
- Closing a tab while scrolled keeps the remaining offset valid.
- The tab bar remains usable in light/dark appearances and supported display scales.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The component MUST represent an ordered collection of zero or more tabs.
- **FR-002**: Every tab MUST have stable identity, one associated page, a project name, SVG image, and X-image close control.
- **FR-003**: At most one tab MUST be active, and only its associated page MUST be visible.
- **FR-004**: Selecting a tab MUST activate it and navigate to its page.
- **FR-005**: Selecting the active tab MUST be idempotent and MUST NOT duplicate its page.
- **FR-006**: Closing an inactive tab MUST remove only that tab and page without changing the active tab.
- **FR-007**: Closing the active tab MUST select the nearest right tab when available, otherwise the nearest left tab; closing the last tab MUST produce an empty state.
- **FR-008**: Closing a tab MUST NOT also select it or trigger another tab action.
- **FR-009**: Tabs MUST preserve readable name and close-control space rather than shrinking indefinitely on overflow.
- **FR-010**: Overflowing rows MUST convert wheel and horizontal trackpad input into bounded horizontal scrolling.
- **FR-011**: A non-overflowing row MUST NOT acquire an unnecessary scroll offset.
- **FR-012**: The active tab MUST be fully visible after any selection change.
- **FR-013**: Tab-bar labels MUST use the project's `text_xs` typography token.
- **FR-014**: Active, inactive, hovered, and focused tabs MUST be visually distinguishable in light and dark appearances.
- **FR-015**: Actionable tab bodies and close controls MUST show hover feedback and an appropriate hand pointer.
- **FR-016**: Users MUST be able to reach, select, and close tabs without pointer input.
- **FR-017**: Tabs MUST expose name, ordered position, and selected state; close controls MUST expose an action-specific accessible name.
- **FR-018**: Missing or invalid SVG images MUST use a consistent fallback without blocking tab behavior.
- **FR-019**: Resizing MUST update overflow bounds while preserving the active tab and page.
- **FR-020**: The tab bar MUST be reusable without depending on repository operations, a particular page layout, or global application state.

### Key Entities

- **Tab Bar**: Ordered navigation container with active identity and a horizontal viewport.
- **Tab**: Stable item containing project identity, name, SVG, close action, selected state, and one page association.
- **Page**: Content associated with one tab and visible only while that tab is active.
- **Tab Viewport**: Visible horizontal portion of the tab row and its bounded scroll position.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: With at least 20 tabs, 100% of selections show the matching page and leave exactly one tab active.
- **SC-002**: Active and inactive closure tests at every row position remove exactly one tab and apply the specified next-selection rule in 100% of cases.
- **SC-003**: Users can bring any tab in a 30-tab overflowing row into view and select it using mouse-wheel scrolling without resizing the window.
- **SC-004**: Scrolling exposes no blank space beyond either content boundary across tested viewport sizes and scales.
- **SC-005**: Names and close controls have zero overlaps for empty, typical, duplicate, and long-name cases.
- **SC-006**: All labels use extra-small typography and every actionable tab body and close control provides visible hover feedback in light and dark checks.
- **SC-007**: Keyboard and assistive-technology tests can select and close every tab and report the correct active tab with zero pointer-only steps.
- **SC-008**: Selection, scrolling, and closure feedback becomes visible within 100 milliseconds in the reference environment.

## Assumptions

- The first tab is active unless the host supplies another valid active identity.
- Tabs have a practical minimum width and may visually truncate long names while preserving the complete accessible name.
- The host decides whether unsaved work permits closure before requesting tab removal.
- Reordering, pinning, preview tabs, split editors, drag-and-drop, context menus, badges, dirty indicators, and reopening closed tabs are outside scope.
- The component owns navigation state and tab-to-page associations but does not prescribe page content.
- The existing component module remains in the Rust workspace and complies with the GPUI and English-specification constitution rules.
