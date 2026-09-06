# Data Model: Git Graph History

## CommitGraphMarker

| Field | Type | Rules |
|---|---|---|
| `lane` | `usize` | Zero-based active lane position. |
| `color` | `usize` | Stable palette identity within one load. |

The default is lane 0/color 0, matching the supplied algorithm's first track.

## CommitGraphEdge

| Field | Type | Rules |
|---|---|---|
| `waiting_commit_id` | `String` | Full identity this lane must reach. |
| `from_lane` | `usize` | Lane at the segment's start. |
| `to_lane` | `usize` | Lane at the segment's end. |
| `color` | `usize` | Preserved while a lane continues. |
| `tip` | `bool` | Commit-connected edge when true; uninterrupted carry when false. |

Inputs describe top-to-marker connections; outputs describe marker-to-bottom connections used by the next track.

## CommitSummary (Enriched Track)

| Field | Type | Rules |
|---|---|---|
| `object_id` | `String` | Full unique commit identity. |
| `parent_ids` | `Vec<String>` | Ordered full parent IDs; first parent has continuation semantics. |
| `summary` | `String` | Message title. |
| `message_body` | `String` | Body, empty when absent. |
| `author_name` / `author_email` | `String` | Repository-decoded author identity. |
| `authored_at_seconds` / `authored_at_offset_minutes` | integer | Existing authored time. |
| `commit_lane` | `CommitGraphMarker` | Row marker geometry. |
| `input_edges` / `output_edges` | edge vectors | Top and bottom row connections; outputs sorted by destination. |
| `is_head` | `bool` | True only when equal to resolved HEAD. |
| `branches` | `Vec<String>` | Shortened local/remote branch labels at this commit. |

### Invariants

- IDs are non-empty and returned commit IDs are unique.
- Outputs are sorted by destination lane.
- A non-tip continuation preserves waiting ID, lane, and color.
- First parent continues from the marker lane/color.
- Labels attach only to their peeled commit.
- At most one returned track is HEAD and at most 100 tracks are returned.

## Repository-Private State

- **BranchTip**: shortened name plus typed object ID; forms traversal roots and the label map.
- **GraphColorPool**: per-load monotonically increasing numeric color allocator, starting with the supplied default color behavior.
- **OrderedCommitDescriptor**: typed ID, ordered parents, and parsed metadata passed from gix I/O into the pure builder.

## Relationships

```text
ProjectData
└── commits: Vec<CommitSummary / graph track>
    ├── commit_lane: CommitGraphMarker
    ├── input_edges: Vec<CommitGraphEdge>
    ├── output_edges: Vec<CommitGraphEdge>
    └── branches: labels derived from BranchTip
```

## Lane State Transition

1. First commit uses lane 0/color 0 with no inputs.
2. Later inputs derive from the preceding outputs' distinct destination lanes in order.
3. Carried lanes compact left-to-right while edges waiting for the current commit are detected.
4. The first reaching edge selects marker lane/color; if none reaches, allocate a new lane/color.
5. Each non-tip input becomes a straight non-tip output.
6. First parent continues on marker lane/color.
7. Additional parents reuse a matching waiting lane/color or allocate a new destination/color.
8. Sort outputs by destination and retain the track as state for the next commit.

Published tracks are immutable; builder state is discarded after the bounded load.
