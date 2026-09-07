# Contract: Project Graph History

## Repository Boundary

The existing application interface remains:

```text
load_history(ProjectMetadata) -> Result<ProjectData, RepositoryError>
```

A successful available-repository result MUST preserve `project_id`, report resolved HEAD when available, return zero to 100 unique enriched tracks from all readable local/remote branch roots in newest-first commit-time order, and expose no GPUI/DataGrid type. Fatal enumeration, traversal, commit, or metadata errors return a user-safe `RepositoryError`; one bad branch reference is omitted without failing valid roots.

## Pure Graph Builder

Input is an ordered sequence of typed commit descriptors, optional HEAD, and labels grouped by peeled commit ID. Output preserves commit order and deterministically applies the transition in `data-model.md`.

The builder MUST NOT perform repository I/O, invoke GPUI, mutate published tracks, or panic because an optional parent/pending edge is absent.

## History Presentation

The History grid requests the next page at its bottom prefetch threshold, appends only new
commit identities, preserves scroll position, and stops after exhaustion. A repository/filter
or project change starts a fresh session. Repeated bottom events while loading are coalesced;
an additional-page failure retains existing rows and exposes retry.

The retained DataGrid column order remains Graph, Commit, Author, Date, Hash. The Graph cell MUST render the marker, all top-to-center inputs, all center-to-bottom outputs, and distinguish diagonal lane transitions. HEAD and branch context must remain associated with the correct row. Drawing remains clipped, supports arbitrary lane/color indices, and cycles palette lookup safely.

## Compatibility

- Existing lazy loading, empty state, alert/retry, and per-project ownership remain.
- DataGrid retains header, virtualization, scrolling, resizing, and accessibility responsibility.
- Existing summary, author, date, and short hash values remain visible.
- The reusable widget gains no Git-domain dependency.

## Verification Matrix

| Scenario | Repository result | Graph result | Page result |
|---|---|---|---|
| Linear | ordered unique commits | continuous lane/color | straight connected markers |
| Diverged | both branch paths | new and carried lane | both paths visible |
| Merge/multi-parent | all parents | continuation plus parent edges | transitions distinguishable |
| Multiple labels | labels grouped by commit | unchanged topology | labels on correct row |
| No branches | empty success | no tracks | existing empty state |
| Invalid one ref | valid roots load | valid topology | no false global failure |
| Fatal commit read | repository error | no partial publication | retry state |
| More history at bottom | continuation page | older unique tracks appended | position preserved |
| Additional page failure | existing rows retained | no duplicates | load-more retry |
| Exhausted history | no continuation | no further traversal | stable bottom |
