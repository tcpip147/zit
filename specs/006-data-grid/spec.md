# Feature Specification: Reusable Data Grid Widget

**Feature Branch**: `[006-data-grid]`

**Created**: 2026-09-06

**Status**: Draft

**Input**: User description (translated): "Extract the grid table currently used by `005-project-pages` into a new reusable widget named `006-data-grid`, implemented at `crates/widget/src/data-grid.rs`."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Display Structured Rows (Priority: P1)

As an application developer, I can configure a reusable data grid with columns and rows so users can scan structured information consistently across Zit views.

**Why this priority**: A reusable grid must first preserve the current project-history table's essential structure without depending on Git-specific data.

**Independent Test**: Configure a grid with five columns and several arbitrary rows, then verify that one header and the matching cell values appear in the declared column order.

**Acceptance Scenarios**:

1. **Given** a grid with declared columns and rows, **When** it is displayed, **Then** one fixed header presents every column label in declaration order and each row presents one corresponding cell per column.
2. **Given** cells containing text or custom visual content, **When** rows are rendered, **Then** each cell displays its supplied content without requiring the grid to understand the application's data model.
3. **Given** content wider than a cell, **When** it is displayed, **Then** it remains within the cell boundary and does not overlap neighboring columns.
4. **Given** no rows, **When** the grid is displayed, **Then** its header remains valid and its body is empty without an error or placeholder imposed by the widget.

---

### User Story 2 - Navigate a Large Row Set (Priority: P1)

As a user, I can scroll through a large grid while the header remains visible and the interface stays responsive.

**Why this priority**: The project history may contain thousands of commits, so rendering the entire data set at once would make extraction ineffective.

**Independent Test**: Display 10,000 uniform-height rows in a constrained viewport, scroll from the first to the last row and back, and verify that visible rows and the header remain correct.

**Acceptance Scenarios**:

1. **Given** more rows than fit in the viewport, **When** the user scrolls vertically, **Then** the body moves within valid bounds while the header remains fixed.
2. **Given** a large row set, **When** the grid is displayed or scrolled, **Then** it requests presentation content only for the currently needed row range rather than materializing every row simultaneously.
3. **Given** the first or last scroll boundary, **When** the user continues scrolling outward, **Then** the viewport remains within the data set without blank overscroll content.
4. **Given** all rows fit in the viewport, **When** the grid is displayed, **Then** no usable scroll range or unnecessary scroll thumb is presented.

---

### User Story 3 - Use the Grid Scroll Control (Priority: P2)

As a user, I can understand the approximate scroll position and drag the grid's vertical scroll thumb to reach distant rows quickly.

**Why this priority**: Direct thumb manipulation complements wheel input for long histories and other large data sets.

**Independent Test**: Overflow the grid, reveal and drag the thumb to multiple positions, release it inside and outside the grid, and verify bounded viewport movement and visibility transitions.

**Acceptance Scenarios**:

1. **Given** an overflowing grid, **When** the pointer hovers over the scroll track or the user scrolls the body, **Then** the vertical scroll control becomes visible.
2. **Given** a visible scroll thumb, **When** the user drags it, **Then** the body scroll position changes proportionally and remains within valid bounds.
3. **Given** a thumb drag continues beyond the grid boundary, **When** the pointer moves or is released, **Then** the drag remains coherent and ends without leaving a stuck interaction state.
4. **Given** the user stops interacting with the scroll control, **When** the visibility delay elapses, **Then** the control hides unless it is hovered or still being dragged.

---

### User Story 4 - Reuse the Grid in Project History (Priority: P2)

As a Zit maintainer, I can replace the project page's private history-table implementation with the reusable grid without changing the history users see or how they scroll it.

**Why this priority**: Successful extraction requires an existing consumer to prove that the widget is data-agnostic and behaviorally compatible.

**Independent Test**: Render a project history through the reusable grid and compare its columns, commit rows, scrolling, and empty-history behavior with the existing project-page contract.

**Acceptance Scenarios**:

1. **Given** loaded commit history, **When** the History section is displayed through the reusable grid, **Then** Graph, Commit, Author, Date, and Hash appear in the same order and every commit remains reachable.
2. **Given** project-specific commit content, **When** the grid requests visible rows, **Then** the project page supplies graph and commit cells without adding Git-specific behavior to the widget.
3. **Given** an empty repository, **When** History is displayed, **Then** the existing project-specific empty-history presentation remains owned by the project page rather than the generic grid.

### Edge Cases

- The grid has zero columns, zero rows, or columns but no rows.
- A supplied row has fewer or more cells than the declared column count.
- The viewport is smaller than the header or one body row.
- Row count changes while the viewport is scrolled near the end.
- The pointer leaves the grid while dragging the scroll thumb.
- Cell content is empty, very long, or contains a custom visual element.
- Multiple grids coexist and maintain independent scroll and drag state.
- The pointer presses within four pixels on either side of a header boundary and drags beyond a neighboring column's minimum width.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST provide a reusable data-grid widget at `crates/widget/src/data-grid.rs` and expose it through the reusable widget package.
- **FR-002**: The grid MUST accept an ordered column definition containing a user-visible label for each column.
- **FR-003**: The grid MUST accept a row count and a data-agnostic means of producing cell content for requested row ranges.
- **FR-004**: The grid MUST render one fixed header followed by a vertically scrollable body whose cells align with the declared columns.
- **FR-005**: Each rendered row MUST contain exactly one cell position per declared column; missing supplied cells MUST leave empty positions and excess supplied cells MUST NOT create undeclared columns.
- **FR-006**: Cell content MUST remain clipped to its cell and MUST NOT overlap adjacent columns.
- **FR-007**: The grid MUST support text and custom visual cell content without depending on project, repository, or commit types.
- **FR-008**: Body rows MUST use a consistent configurable height suitable for range-based rendering.
- **FR-009**: The grid MUST render only the row range needed for the current viewport plus any bounded presentation buffer, rather than eagerly rendering the complete row set.
- **FR-010**: Vertical wheel or trackpad input MUST scroll the body within valid content bounds while the header remains visible.
- **FR-011**: An overflowing grid MUST provide a proportional draggable vertical scroll thumb with a minimum usable size.
- **FR-012**: The scroll control MUST become visible during wheel interaction, hover, or thumb dragging and MUST hide after one second without interaction when it is not hovered or dragged.
- **FR-013**: Thumb dragging MUST continue and terminate correctly when the pointer moves or is released outside the grid bounds.
- **FR-014**: A grid without vertical overflow MUST NOT display a functional thumb or create a nonzero scroll range.
- **FR-015**: Each grid instance MUST maintain independent scroll, drag, and visibility state.
- **FR-016**: Column widths, header appearance, row appearance, cell spacing, dividers, and scroll-control colors MUST be configurable through reusable grid configuration or the shared theme.
- **FR-017**: The grid MUST expose appropriate table, row, header, and cell semantics to assistive technologies for the currently rendered content.
- **FR-018**: The project History section MUST use the reusable grid while preserving its existing column order, visible commit information, lazy history loading, empty state, and failure handling.
- **FR-019**: The generic grid MUST NOT contain Git-specific labels, commit formatting, repository operations, or project-page load-state behavior.
- **FR-020**: Each boundary between header columns MUST expose an 8-pixel-wide horizontal resize target centered on the divider and use a horizontal-resize pointer cursor.
- **FR-021**: Dragging a header boundary MUST resize the columns on both sides, keep their combined width stable, apply the same widths to header and body cells, and enforce a 40-pixel minimum column width.
- **FR-022**: Column resize dragging MUST continue and terminate when the pointer moves or is released outside the grid.
- **FR-023**: Sorting, filtering, column reordering, row selection, cell editing, and horizontal scrolling MUST remain outside this feature's scope.

### Key Entities

- **Data Grid**: A reusable structured-data viewport owning column configuration and independent scrolling interaction state.
- **Column Definition**: The ordered description of one header and corresponding cell position, including its label and width behavior.
- **Row Provider**: A data-agnostic producer that supplies cell content for requested visible row ranges.
- **Grid Row**: One uniform-height ordered collection of cells associated with a row index.
- **Grid Cell**: Text or custom visual content constrained to one declared column.
- **Grid Viewport**: The visible body region, current vertical offset, and measurable overflow bounds.
- **Scroll Control**: The track, proportional thumb, visibility state, and active drag information belonging to one grid.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In automated checks, grids containing 0, 1, 100, and 10,000 rows render the correct header and requested visible rows in 100% of cases.
- **SC-002**: With 10,000 rows, initial display and each scroll update provide visible feedback within 100 milliseconds on a typical supported development machine.
- **SC-003**: Scrolling and thumb dragging reach both data boundaries with zero blank overscroll regions and zero stuck drag states across the acceptance scenarios.
- **SC-004**: Two simultaneously displayed grids retain independent scroll and visibility state in 100% of interaction checks.
- **SC-005**: The project History section presents the same five columns and makes 100% of loaded commits reachable after migration to the reusable grid.
- **SC-006**: Automated dependency checks find zero references to project, repository, or commit domain types inside the reusable grid module.
- **SC-007**: Accessibility inspection reports table structure and the correct visible header and cell relationships for 100% of tested grid configurations.
- **SC-008**: Every internal header boundary can be dragged in either direction without either adjacent column becoming narrower than 40 pixels or header/body alignment diverging.

## Assumptions

- The first consumer is the project History section from `005-project-pages`, but the widget is designed for unrelated structured row data.
- Rows have a uniform configured height; variable-height rows are outside scope.
- The initial grid uses a single fixed header and vertical scrolling only.
- The row provider remains responsible for stable ordering, domain formatting, and producing content for requested indexes.
- Project-specific empty, loading, and failure presentations remain outside the grid.
- The widget follows the project's shared visual theme and GPUI-based reusable widget architecture.
