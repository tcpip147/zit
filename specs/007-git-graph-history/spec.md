# Feature Specification: Git Graph History

**Feature Branch**: `[007-git-graph-history]`

**Created**: 2026-09-06

**Status**: Draft

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

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST produce graph history for a selected available repository using the project's Rust-native Git backend and MUST NOT invoke the Git command-line executable.
- **FR-002**: Graph traversal MUST begin from every readable local and remote branch tip that resolves to a commit.
- **FR-003**: Unreadable or non-commit branch references MUST be skipped independently so one invalid reference does not suppress valid history.
- **FR-004**: Reachable commits MUST appear at most once in newest-first commit-time traversal order.
- **FR-005**: One graph-history load MUST return no more than 100 commit tracks.
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
- **FR-037**: The lower History pane MUST be divided into equal left and right panels; selecting a grid row MUST show that commit's full Hash, Parent Hash values when present, Author, committer-time Date, Committer, and complete Message in the left panel, while the right panel remains reserved for future content.
- **FR-038**: The lower-left History panel MUST be divided into equal upper and lower sections; the upper section MUST retain the selected commit metadata, while the lower section asynchronously lists paths changed between the selected commit and its first parent using gix, or the empty tree for a root commit.

### Key Entities

- **Graph Track**: One history row combining commit metadata with its graph marker, incoming edges, outgoing edges, HEAD state, and branch labels.
- **Lane Marker**: The lane number and color assigned to the commit represented by a track.
- **Graph Edge**: A directed visual connection carrying a waiting commit identity between source and destination lanes, with a color and tip/continuation classification.
- **Color Pool**: A per-history allocation sequence that assigns stable identities to newly created graph lanes.
- **Branch Tip**: A shortened local or remote branch name paired with the commit identity from which traversal begins.
- **History Filter**: The selected branch scope and remote-branch inclusion flag used to derive visible history rows.
- **Graph History**: The ordered, bounded collection of graph tracks produced for one repository load.

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

## Assumptions

- The supplied `Branch`, `Edge`, `Marker`, `Track`, and `ColorPool` concepts and the supplied lane-transition algorithm are authoritative; names and ownership may be adapted to existing project conventions without changing observable topology.
- The graph is bounded to the first 100 commits produced by newest-first commit-time traversal, matching the supplied code.
- Local and remote branches are graph roots; tags and other reference namespaces are outside this feature unless also reachable from those branches.
- Shortened branch names follow the repository backend's established reference-shortening behavior; identical display names from different namespaces remain separate labels if both references are present.
- Graph colors are stable numeric identities chosen per history load; visual color values belong to the presentation layer.
- Existing project repository discovery, background loading, error presentation, DataGrid virtualization, and History page ownership are reused.
- Commit metadata exposed by this feature follows the supplied code: commit time, full message title and optional body, author name, author email, and complete object identity.
