# Research: Git Graph History

## Multi-Branch Roots

**Decision**: Collect readable local and remote branches, shorten their names, peel each to a commit, and pass every resulting ID to one revision walk.

**Rationale**: Matches the supplied code, exposes branch-only commits, and lets the walker de-duplicate shared ancestry.

**Alternatives considered**: HEAD-only walking omits other branches; per-branch walks duplicate merging logic; tags/all refs exceed the supplied scope.

## Ordering and Limit

**Decision**: Use commit-time ordering newest first, return at most 100 tracks per page, and continue traversal from an explicit page boundary for bottom-triggered loading.

**Rationale**: Preserves the bounded batch behavior while allowing users to browse older history. A continuation boundary avoids repeatedly returning the same prefix.

**Alternatives considered**: Topological-only order differs from the reference; unbounded loading is not bounded by UI virtualization; a separate load button adds friction; restarting from branch tips duplicates page work.

## Bottom-Triggered Loading

**Decision**: Treat the initial result and each subsequent result as a page owned by the repository/filter session. Trigger one additional request near the grid bottom, serialize requests, deduplicate by commit identity, and mark the session exhausted when no new rows remain or a short page is returned.

**Rationale**: The grid can append older commits without losing project isolation or scroll position. Explicit loading and exhaustion state makes repeated bottom events idempotent and gives failures a retry path.

**Alternatives considered**: Loading the entire repository at once violates the bounded-load goal; replacing the visible vector causes scroll jumps; concurrent requests risk duplicates and out-of-order graph state.

## Algorithm Boundary

**Decision**: Implement lane transitions as a pure sequential builder over ordered commit descriptors. A track consumes only the preceding track's output edges and its own parent IDs.

**Rationale**: Preserves the algorithm and enables exact topology tests without repository I/O.

**Alternatives considered**: UI-side computation couples topology to GPUI; full-prefix recomputation is quadratic; an external layout algorithm would violate the supplied-code constraint.

## Object IDs

**Decision**: Keep `gix::ObjectId` during repository operations and convert IDs to strings in public project models and graph edges.

**Rationale**: Existing models, UI, and fixtures use strings; the adaptation contains backend types at the repository boundary.

**Alternatives considered**: Public gix IDs spread backend coupling; reparsing strings inside the builder discards already available typed IDs.

## Model Integration

**Decision**: Add marker and edge records and enrich existing `CommitSummary` with body, graph, HEAD, and branches. Keep `ProjectData.commits`.

**Rationale**: The enriched summary is the project-adapted `Track` without duplicating metadata or disrupting existing page contracts.

**Alternatives considered**: Replacing summaries with `Track` causes wider churn; parallel graph vectors risk positional desynchronization.

## Failure Policy

**Decision**: Skip an invalid individual branch reference, but fail the load when enumeration, walking, a selected commit, or required metadata cannot be read. Replace reference-code `unwrap` calls with checked invariant handling.

**Rationale**: Mixed-validity refs remain useful, incomplete topology is not silently published, and repository corruption cannot panic the process.

**Alternatives considered**: First-bad-ref failure is too broad; silently skipping commits breaks lane continuity; unchecked indexing violates the recovery requirement.

## Presentation

**Decision**: Use GPUI canvas paths in `ProjectPage`, mapping numeric colors through a cycling palette. Inputs run top-to-center, outputs center-to-bottom, with the marker painted above paths.

**Rationale**: Canvas represents straight and diagonal transitions for arbitrary lanes while visual policy remains outside domain data.

**Alternatives considered**: Rectangular divs cannot faithfully draw diagonals; DataGrid-owned drawing creates Git coupling; bitmaps cannot adapt to lane geometry.
