# Feature Specification: Diff Editor

**Feature Branch**: `[008-diff-editor]`
**Created**: 2026-09-07
**Status**: Draft

## User Scenarios & Testing

### User Story 1 - Read Parsed File Changes (Priority: P1)

As a user viewing a changed file, I can read additions, deletions, context lines, hunk boundaries, and before/after line numbers without raw diff metadata obscuring the content.

**Acceptance Scenarios**:

1. Unified diff headers and `+`/`-` prefixes are hidden from displayed content.
2. Additions, deletions, and context rows use distinct visual treatments.
3. Before and after line numbers are calculated from hunk ranges and displayed in separate left-aligned columns.
4. The first hunk has no leading separator; later hunks are visually separated.

### User Story 2 - Navigate Large and Wide Diffs (Priority: P1)

As a user, I can navigate large diffs vertically and long lines horizontally while the line-number gutter remains fixed.

**Acceptance Scenarios**:

1. Only visible rows plus bounded overscan are rendered.
2. Vertical scrolling keeps gutter and code rows synchronized.
3. Horizontal scrolling moves only the code pane.
4. The final row remains visible above the horizontal scrollbar.
5. Selecting a new file resets both axes to the top-left.

### User Story 3 - Control Scrollbars Directly (Priority: P2)

As a user, I can use visible scrollbar tracks to drag thumbs or jump to a clicked track position.

**Acceptance Scenarios**:

1. Vertical and horizontal thumbs use the actual viewport and content ranges.
2. Dragging continues when the pointer leaves the thumb and ends on mouse release.
3. Clicking empty track space centers the thumb at the clicked position.
4. Hover and drag colors match the reusable grid scrollbar convention.
5. Scroll offsets never exceed valid bounds.

## Functional Requirements

- **FR-001**: The editor MUST parse unified diff text into context, addition, deletion, and hunk-boundary rows.
- **FR-002**: The editor MUST hide raw diff headers and line prefixes from displayed code content.
- **FR-003**: Before and after line numbers MUST be calculated independently from hunk ranges.
- **FR-004**: The line-number gutter MUST be split into before and after columns, left-aligned, dynamically sized to the largest displayed number, with a border only after the after column.
- **FR-005**: The gutter MUST remain fixed during horizontal code scrolling.
- **FR-006**: Vertical gutter and code scrolling MUST remain synchronized.
- **FR-007**: The editor MUST support bounded visible-row virtualization with overscan and stale-index protection.
- **FR-008**: The editor MUST provide vertical and horizontal tracks and thumbs with proportional geometry.
- **FR-009**: Thumb dragging MUST continue outside the thumb and track clicks MUST center the thumb at the clicked location.
- **FR-010**: Hover and drag colors MUST use default `#94a3b8`, active `#64748b`, and track `#e2e8f0`.
- **FR-011**: Loading a new diff MUST reset both scroll axes and render the new content immediately.
- **FR-012**: The final content row MUST have clearance from the horizontal scrollbar.

## Success Criteria

- **SC-001**: Representative diffs with additions, deletions, multiple hunks, and line-number changes render correctly in 100% of parser acceptance cases.
- **SC-002**: In a 10,000-row diff, only the visible range plus bounded overscan is materialized.
- **SC-003**: Vertical and horizontal scrolling reach both valid boundaries without overscroll or stuck drag states.
- **SC-004**: Horizontal scrolling leaves the complete line-number gutter visible in 100% of interaction cases.
- **SC-005**: Switching files resets the view to the new file's first row in 100% of cases.

## Assumptions

- DiffEditor is a reusable widget in `crates/widget` and does not perform repository I/O.
- Project pages provide already-loaded diff text and own asynchronous loading/error presentation.
- Rows use a fixed height for virtualization and scrollbar mapping.
- Shared ScrollContainer provides the same vertical scrollbar interaction behavior where used.
