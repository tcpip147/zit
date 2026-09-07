# Feature Specification: Git Graph History

**Feature Branch**: `[007-git-graph-history]`

**Created**: 2026-09-06

**Status**: Draft

> DiffEditor requirements were extracted into [008-diff-editor](../008-diff-editor/spec.md). This feature now owns the reusable diff widget; 007 only consumes the selected-file/diff loading boundary.

**Input**: User description: "Add feature 007 to provide Git graph history. It must use gix, and its algorithm and data structures must match the supplied code while being adapted appropriately for this project."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Read Repository-Wide Graph History (Priority: P1)

As a user viewing project history, I can see a single newest-first history assembled from commits reachable from local and remote branch tips, so relevant work remains visible even when it is not reachable from the current branch.

**Why this priority**: A complete, deterministic history is the foundation for every graph lane and label shown by this feature.

**Independent Test**: Create a repository with diverged local and remote branches, load its history, and verify that reachable commits from every valid branch tip appear once in newest-first commit-time order, up to the configured 100-row limit.

**Acceptance Scenarios**:

1. **Given** local and remote branches point to overlapping histories, **When** history is loaded, **Then** every reachable commit appears at most once and the result is ordered by newest commit time first.
2. **Given** a commit is reachable from a non-current branch only, **When** history is loaded, **Then** that commit is included.
3. **Given** more than 100 reachable commits, **When** history is loaded, **Then** exactly the newest 100 traversed commits are returned.
4. **Given** no valid branch tip exists, **When** history is loaded, **Then** an empty graph history is returned without a graph-processing failure.

---

### User Story 2 - Understand Branch and Merge Topology (Priority: P1)

As a user scanning history, I can follow colored lanes into and out of each commit, including diverging parents and converging branches, so I can understand the repository topology without opening individual commits.

**Why this priority**: Lane continuity and merge representation are the primary value of a Git graph rather than a plain commit list.

**Independent Test**: Build linear, branch, two-parent merge, and multi-parent merge histories and verify each row's commit marker, incoming edges, outgoing edges, lane positions, colors, continuation state, and parent targets.

**Acceptance Scenarios**:

1. **Given** a linear history, **When** graph tracks are generated, **Then** the commit markers and connecting edges remain on one continuous lane and retain one color.
2. **Given** a commit with multiple parents, **When** its track is generated, **Then** its first parent continues the commit lane and each additional parent receives or rejoins a distinct appropriate lane.
3. **Given** an active lane is waiting for a later commit, **When** intervening commits are processed, **Then** a non-tip continuation edge carries that lane forward without changing its waiting commit or color.
4. **Given** multiple prior edges target the current commit, **When** its track is generated, **Then** one lane is selected for the commit marker and every converging input is retained without duplicate occupied output lanes.

---

### User Story 3 - Identify Commit Context (Priority: P2)

As a user reading the graph, I can identify the current HEAD, all branch names pointing at a commit, and the commit's message, author, time, and identity alongside its lane information.

**Why this priority**: Graph geometry becomes actionable only when users can associate it with repository references and commit details.

**Independent Test**: Point several local and remote branches and HEAD at known commits, load graph history, and compare every track's labels and metadata with the repository.

**Acceptance Scenarios**:

1. **Given** HEAD points to a commit present in the result, **When** tracks are generated, **Then** exactly that track is marked as HEAD.
2. **Given** multiple local or remote branches point to one commit, **When** its track is generated, **Then** all shortened branch names are attached to that track.
3. **Given** a commit contains a title, body, author identity, and authored time, **When** its track is generated, **Then** those values and its complete object identity are available to the history presentation.

---

### User Story 4 - View the Graph in Project History (Priority: P2)

As a project user, I see the generated topology in the existing History grid while retaining the established loading, empty, error, retry, scrolling, and project-isolation behavior.

**Why this priority**: The graph must integrate into the existing history experience without regressing repository loading or reusable grid behavior.

**Independent Test**: Open History for repositories containing linear, branched, merged, empty, and unreadable histories, then verify graph rendering and all existing page states.

**Acceptance Scenarios**:

1. **Given** graph tracks load successfully, **When** History is displayed, **Then** the Graph column renders the marker and all incoming and outgoing lane segments for each visible row.
2. **Given** history is empty or loading fails, **When** History is displayed, **Then** the existing empty or retry presentation remains responsible for feedback.
3. **Given** two project pages are open, **When** their histories are loaded and scrolled independently, **Then** graph data and view state do not leak between projects.

### User Story 5 - Load More History at the Grid Bottom (Priority: P1)

As a user browsing project history, I can continue scrolling past the initially loaded commits and have the next commit list added automatically to the bottom of the grid, so I can inspect older history without restarting the view or pressing a separate load button.

**Why this priority**: A bounded initial result must not prevent users from reaching older commits that are relevant to investigation.

**Independent Test**: Use a repository with more commits than one history page, scroll the grid to its bottom repeatedly, and verify that additional older rows are appended in order, loading is not duplicated, and the grid stops requesting data after the history is exhausted.

**Acceptance Scenarios**:

1. **Given** more history is available and the user reaches the bottom threshold of the grid, **When** the bottom condition is detected, **Then** the next page of commits is loaded automatically and appended after the existing rows.
2. **Given** a page is being loaded, **When** the user remains at or repeatedly reaches the bottom, **Then** only one request is active and no duplicate rows are appended.
3. **Given** the next page overlaps commits already displayed, **When** it is appended, **Then** each commit remains represented by one row and graph lanes are rebuilt or extended consistently for the combined ordered history.
4. **Given** no more commits are available, **When** the user reaches the bottom, **Then** no further load is started and the grid remains stable.
5. **Given** an additional page fails to load, **When** the user reaches the bottom again or retries, **Then** the existing rows remain visible and the user receives a recoverable retry state for the additional page.
6. **Given** additional rows are appended, **When** the user is already viewing the bottom of the grid, **Then** the current visible position remains stable and the newly appended rows can be reached by continued scrolling.

### User Story 6 - Inspect File Diffs in a Diff Editor (Priority: P2)

As a user selecting a changed file in a commit, I can inspect a readable, navigable diff with before and after line numbers while keeping the line-number gutter fixed during horizontal scrolling.

**Why this priority**: File history is only useful when the selected change can be read and navigated efficiently.

**Independent Test**: Select changed text files containing additions, deletions, multiple hunks, long lines, and enough content to overflow both axes; verify parsed rows, line numbers, fixed gutter behavior, scrolling, thumb interaction, and file switching.

**Acceptance Scenarios**:

1. **Given** a unified diff with additions, deletions, context lines, and multiple hunks, **When** it is displayed, **Then** diff headers and prefixes are hidden, content rows are color-coded, and hunk boundaries are visually separated without a leading separator.
2. **Given** a diff with before and after line numbers, **When** it is displayed, **Then** before and after numbers are shown in separate left-aligned columns with dynamically sized gutters, only the after column has a right border, and the final row remains visible above the horizontal scrollbar.
3. **Given** a diff containing long lines, **When** the user horizontally scrolls, **Then** only the code pane moves and the line-number gutter remains visible.
4. **Given** a diff taller than the viewport, **When** the user uses the wheel, drags either vertical thumb, or clicks empty vertical track space, **Then** the gutter and code panes remain vertically synchronized and scrolling is clamped to valid bounds.
5. **Given** a diff wider than the viewport, **When** the user drags the horizontal thumb or clicks empty horizontal track space, **Then** the code pane moves to the requested position and the thumb movement matches the actual content range.
6. **Given** the pointer enters a scrollbar track or thumb, **When** the pointer is over the relevant axis, **Then** that axis's thumb changes to the established hover color without changing the other axis's thumb.
7. **Given** the user selects another changed file, **When** its diff finishes loading, **Then** both panes reset to the top-left and the new content is rendered immediately without stale indices or scroll positions.
8. **Given** a large diff, **When** the editor renders or the window moves, **Then** only rows near the visible vertical range are materialized and the editor remains responsive.

### Edge Cases

- Several local and remote branch names point to the same commit.
- A branch reference cannot be read or cannot be resolved to a commit while other branches remain valid.
- HEAD is unborn, detached, unavailable, or points to a commit outside the 100 returned tracks.
- The repository has no local or remote branches.
- A commit has no parents, one parent, two parents, or more than two parents.
- Multiple pending edges wait for the same commit or parent.
- Commit messages have no body or contain non-UTF-8 data requiring the repository backend's normal lossy text representation.
- Author information or commit-time parsing fails after traversal has begun.
- Two commits have the same commit timestamp.
- The grid reaches its bottom while an additional history page is already loading.
- The repository ends exactly at a page boundary or returns fewer commits than the requested page size.
- An additional page contains commits already present because traversal boundaries overlap.
- The repository changes or becomes unavailable while an additional page is loading.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST produce graph history for a selected available repository using the project's Rust-native Git backend and MUST NOT invoke the Git command-line executable.
- **FR-002**: Graph traversal MUST begin from every readable local and remote branch tip that resolves to a commit.
- **FR-003**: Unreadable or non-commit branch references MUST be skipped independently so one invalid reference does not suppress valid history.
- **FR-004**: Reachable commits MUST appear at most once in newest-first commit-time traversal order.
- **FR-005**: One graph-history load MUST return no more than 100 commit tracks.
- **FR-005a**: The History grid MUST load an initial page and MUST automatically request the next page when the user reaches the configured bottom threshold while more history is available.
- **FR-005b**: Each additional page MUST be appended after the existing rows in older-to-newer continuation order, preserving the overall newest-first ordering.
- **FR-005c**: The system MUST NOT start more than one initial or additional history request for the same repository and filter state at a time.
- **FR-005d**: Additional pages MUST deduplicate commit identities against all already displayed rows before they are appended.
- **FR-005e**: When an additional page returns no new commits or fewer than the page limit, the system MUST mark that history/filter result exhausted and MUST NOT issue further automatic requests.
- **FR-005f**: When an additional page fails, the system MUST preserve already displayed rows, expose a retryable load-more failure state, and permit retry without duplicating successful rows.
- **FR-005g**: Appending rows MUST preserve the user's current grid scroll position; if the user is at the bottom, the appended rows MUST remain available through continued scrolling rather than forcing an unrelated jump.
- **FR-006**: Each commit track MUST contain the commit object identity, commit time, message title, message body, author name, author email, commit-lane marker, incoming edges, outgoing edges, HEAD status, and attached branch names.
- **FR-007**: A lane marker MUST contain a zero-based lane number and a reusable color identity.
- **FR-008**: An edge MUST contain the commit it is waiting to reach, source lane, destination lane, reusable color identity, and whether it is a commit-tip edge or a continuing non-tip edge.
- **FR-009**: The first processed track MUST start in lane zero with the initial graph color.
- **FR-010**: For each later track, incoming edges MUST be derived from the preceding track's distinct destination lanes in destination-lane order.
- **FR-011**: When an incoming edge reaches the current commit, the first reaching edge MUST determine the commit marker's lane and color; other reaching edges MUST remain represented as converging inputs.
- **FR-012**: Incoming edges not reaching the current commit MUST retain their waiting commit and color while being compacted into currently occupied lane positions.
- **FR-013**: A commit not reached by any pending input edge MUST receive the next free lane position and a newly allocated color identity.
- **FR-014**: Every non-tip incoming edge MUST produce a straight outgoing continuation with the same lane, waiting commit, and color.
- **FR-015**: A commit's first parent MUST continue directly from the commit lane with the commit marker's color.
- **FR-016**: Each additional parent already represented by an incoming waiting edge MUST connect to that edge's destination lane and reuse its color.
- **FR-017**: Each additional parent without an existing waiting edge MUST receive a new output destination lane and a newly allocated color identity.
- **FR-018**: Outgoing edges MUST be ordered by destination lane before the track is returned.
- **FR-019**: A track MUST be marked as HEAD only when its commit identity equals the repository's current resolved HEAD identity.
- **FR-020**: Every readable local and remote branch pointing to a returned commit MUST contribute its shortened display name to that commit's track.
- **FR-021**: The graph data MUST remain repository-domain data independent of GPUI elements or DataGrid cell types.
- **FR-022**: The existing Project History flow MUST consume graph tracks without regressing lazy loading, loading status, empty history, failure messages, retry, virtual scrolling, or per-project isolation.
- **FR-023**: The History Graph column MUST render enough marker and edge information to distinguish straight continuation, convergence, first-parent continuation, and additional-parent divergence.
- **FR-023a**: Lane-changing convergence and divergence edges MUST use smooth curves rather than angular corner transitions; same-lane continuations MUST remain straight.
- **FR-024**: Repository discovery, traversal, commit reading, and metadata-reading failures MUST be translated into user-safe repository errors without panicking.
- **FR-025**: The implementation MUST use `gix` for repository access and traversal, consistent with the project constitution and the supplied algorithm.
- **FR-026**: The History Author cell MUST display the author name and email as `name <email>`.
- **FR-027**: The History Date cell MUST display the committer time (not the author time) with its recorded timezone offset in `YYYY-MM-DD HH:MI` format.
- **FR-028**: The History Hash cell MUST display the first eight characters of the commit object identity.
- **FR-029**: The initial History column widths MUST be Graph 200 pixels, Author 200 pixels, Date 120 pixels, and Hash 100 pixels; Description MUST fill the remaining width proportionally.
- **FR-030**: The commit-summary column MUST use the visible label `Description` while retaining its existing commit-summary content.
- **FR-031**: Graph commit markers MUST be filled with their lane color except for HEAD, which MUST use a hollow marker.
- **FR-032**: Description MUST represent HEAD with a circular indicator instead of a text label and MUST place that indicator before every branch label.
- **FR-033**: History MUST place a filter toolbar directly above the grid containing a branch selector with `All branches` and `Current branch` options and a checked-by-default `Remote branches` checkbox.
- **FR-034**: `Current branch` MUST show only commits reachable from the resolved HEAD, while `All branches` MUST show commits reachable from local branches and, when enabled, remote branches.
- **FR-035**: Clearing `Remote branches` MUST exclude commits reachable only from remote branch tips, remove remote branch labels such as `origin/...` from Description cells, and rebuild graph lanes for the remaining displayed rows.
- **FR-036**: The ready History page MUST be divided vertically into equal upper and lower panes; the existing filter toolbar and grid MUST occupy the upper pane, while the lower pane MUST remain an empty reserved container separated by a visible boundary.
- **FR-037**: The lower History pane MUST be divided into equal left and right panels; selecting a grid row MUST show that commit's full Hash, Parent Hash values when present, Author, committer-time Date, Committer, and complete message content in the left panel, while the right panel remains reserved for future content.
- **FR-038**: The lower-left History panel MUST be divided into equal upper and lower sections; the upper section MUST retain the selected commit metadata, while the lower section asynchronously lists paths changed between the selected commit and its first parent using gix, or the empty tree for a root commit.
- **FR-038a**: The History changed-file list panel MUST have zero outer padding so file rows align directly with the panel boundaries; row-level padding MAY remain for readable labels and hit targets.
- **FR-048**: Repository history loading MUST extract the committer identity separately from the author identity using gix and preserve committer name and email on every returned commit record.
- **FR-049**: The selected commit details view MUST display the committer as `name <email>` independently from the author field.
- **FR-050**: A commit whose author and committer differ MUST retain both identities without substitution, merging, or loss of either value.
- **FR-051**: The complete selected commit message MUST be displayed without a separate `Message` label; the message title and body remain visible as content.

### Diff Editor Requirements

- **FR-039**: The selected changed-file panel MUST parse unified diff text into context, addition, deletion, and hunk-boundary rows rather than displaying raw diff headers or prefixes.
- **FR-040**: Diff rows MUST calculate independent before and after line numbers from each hunk range; additions MUST advance only after numbering, deletions only before numbering, and context rows both.
- **FR-041**: The diff editor MUST render before and after line-number gutters as separate left-aligned columns with width based on the largest displayed line number; only the after gutter MUST have a right border.
- **FR-042**: The diff editor MUST keep the line-number gutter fixed while horizontal scrolling is applied only to the code pane.
- **FR-043**: The diff editor MUST provide vertical and horizontal scrolling, visible tracks and thumbs, thumb dragging, track-page clicking centered on the thumb, hover/drag colors, and window-level drag continuation outside the thumb.
- **FR-044**: Diff scrollbar movement MUST clamp to the valid content range and use the actual viewport, track, thumb, and content dimensions for position and scale calculations.
- **FR-045**: Selecting a new file MUST reset both vertical and horizontal offsets before rendering the new diff and MUST prevent stale virtualization indices from indexing beyond the new row collection.
- **FR-046**: Diff virtualization MUST render only visible rows plus a bounded overscan range while preserving total scrollable content height and bottom clearance for the horizontal scrollbar.
- **FR-047**: Shared scroll-container behavior MUST support vertical thumb dragging, track-page clicking, hover/drag thumb colors, and window-level drag continuation without retaining invalid frame-scoped UI elements.
- **FR-052**: History MUST provide an 8px centered resize hit area for the Grid/lower-pane, lower-pane left/right, and lower-left metadata/files boundaries without consuming layout space.
- **FR-053**: History resize MUST measure the active container bounds and convert pointer movement from the drag start position without window-size correction constants or arbitrary scale factors.
- **FR-054**: History resize MUST store the boundary start position and mouse start position, then calculate the new boundary as `boundary_start + (mouse_position - mouse_start)`.

### Key Entities

- **Graph Track**: One history row combining commit metadata with its graph marker, incoming edges, outgoing edges, HEAD state, and branch labels.
- **Lane Marker**: The lane number and color assigned to the commit represented by a track.
- **Graph Edge**: A directed visual connection carrying a waiting commit identity between source and destination lanes, with a color and tip/continuation classification.
- **Color Pool**: A per-history allocation sequence that assigns stable identities to newly created graph lanes.
- **Branch Tip**: A shortened local or remote branch name paired with the commit identity from which traversal begins.
- **History Filter**: The selected branch scope and remote-branch inclusion flag used to derive visible history rows.
- **Graph History**: The ordered, bounded collection of graph tracks produced for one repository load.
- **History Page**: One ordered batch of graph tracks and its continuation state, including whether another batch can be requested, is loading, failed, or exhausted.
- **History Load State**: The per-repository and per-filter state that prevents duplicate requests and records initial/additional loading, retryable failure, and exhaustion.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In linear, diverged, merged, and multi-parent acceptance repositories, 100% of returned commits have graph connections that lead to the correct parent tracks when those parents are included in the result.
- **SC-002**: Across repositories with overlapping local and remote branch histories, every reachable returned commit appears exactly once and every valid branch-only history is represented.
- **SC-003**: A history load returns at most 100 tracks and completes within two seconds for a local repository containing 10,000 reachable commits on a typical supported development machine.
- **SC-004**: In automated topology fixtures, 100% of continuing lanes retain their assigned color until they reach their waiting commit.
- **SC-005**: HEAD and all branch labels are attached to the correct returned track in 100% of reference-label acceptance cases.
- **SC-006**: Existing loading, empty, failure, retry, scrolling, and multi-project History acceptance tests continue to pass without regression.
- **SC-007**: Invalid individual branch references do not prevent valid graph history from loading in 100% of mixed-validity acceptance cases.
- **SC-008**: Repository and metadata failures produce a recoverable user-visible error with zero process panics in all defined failure scenarios.
- **SC-009**: For a repository containing more than one page of commits, 100% of bottom-reaching test runs append the next available page without user-initiated refresh and without duplicate commit rows.
- **SC-010**: Repeated bottom events during one in-flight request result in exactly one additional request, and exhausted histories result in zero subsequent requests.
- **SC-011**: In load-more failure tests, all previously visible rows remain available and a retry can recover the next page without reordering or duplicating rows.

## Assumptions

- The supplied `Branch`, `Edge`, `Marker`, `Track`, and `ColorPool` concepts and the supplied lane-transition algorithm are authoritative; names and ownership may be adapted to existing project conventions without changing observable topology.
- The graph is bounded to the first 100 commits produced by newest-first commit-time traversal, matching the supplied code.
- Each history page contains at most 100 commits; the initial page and subsequent pages use a continuation boundary so users can browse beyond the first page until the repository history is exhausted.
- The bottom trigger uses a small prefetch threshold before the absolute final row to avoid requiring pixel-perfect scrolling, while preserving the user-visible order.
- Local and remote branches are graph roots; tags and other reference namespaces are outside this feature unless also reachable from those branches.
- Shortened branch names follow the repository backend's established reference-shortening behavior; identical display names from different namespaces remain separate labels if both references are present.
- Graph colors are stable numeric identities chosen per history load; visual color values belong to the presentation layer.
- Existing project repository discovery, background loading, error presentation, DataGrid virtualization, and History page ownership are reused.
- The diff editor uses fixed-height rows, a synchronized vertical gutter/code layout, and an independent horizontal code-pane scroll range.
- Scrollbar colors follow the existing DataGrid convention: default thumb `#94a3b8`, hover/drag thumb `#64748b`, and track `#e2e8f0`.
- Commit metadata exposed by this feature follows the supplied code: commit time, full message title and optional body, author name, author email, and complete object identity.
