# Data Model: Project Pages

## ProjectId

Stable identifier for one local Git repository.

| Field | Type | Rules |
|-------|------|-------|
| identity_key | string | Non-empty normalized canonical Git-directory location; equality follows platform path semantics |

Validation:

- Constructed only after repository discovery succeeds or from a previously validated persisted identity.
- Two discovered paths with the same identity key map to one project.
- Never derived from display name, branch, or HEAD.

## ProjectMetadata

Lightweight, globally authoritative description of an open repository.

| Field | Type | Rules |
|-------|------|-------|
| id | ProjectId | Unique in the open-project collection |
| repository_path | path | User-facing worktree path used to reopen the project |
| display_name | string | Non-empty; defaults to the worktree directory name |
| availability | ProjectAvailability | Updated by lightweight inspection; does not require history |

### ProjectAvailability

- `Available`: discovery and lightweight inspection succeeded.
- `Unavailable { message }`: path is missing, unreadable, or not currently discoverable as a Git repository.

Transitions:

```text
unknown/restored → Available
unknown/restored → Unavailable
Unavailable → Available       (successful retry/reinspection)
Available → Unavailable       (later failed inspection)
```

## CommitSummary

Immutable page-ready representation of one commit.

| Field | Type | Rules |
|-------|------|-------|
| object_id | string | Full Git object ID; unique within the project history |
| summary | string | First-line commit summary; may be empty for malformed legacy data |
| author_name | string | Lossily displayable when source bytes are not valid text |
| author_email | string | Lossily displayable; may be empty |
| authored_at | timestamp | Preserves Git timestamp and offset when available |

Commit summaries are session memory only and are never written to the open-project session file.

## ProjectData

Heavyweight data belonging to exactly one project.

| Field | Type | Rules |
|-------|------|-------|
| project_id | ProjectId | Must match the owning page/store entry |
| head | optional object ID | Absent for an unborn branch or empty repository |
| commits | ordered list of CommitSummary | Newest-first according to the selected history walk |

## ProjectLoadState

Per-page lifecycle for heavyweight project data.

| State | Payload | Allowed next states |
|-------|---------|---------------------|
| NotLoaded | none | Loading |
| Loading | generation | Ready, Failed, or discarded on close/new generation |
| Ready | ProjectData | Remains Ready during tab switches |
| Failed | generation, user-facing message | Loading on retry |

Rules:

- Activation starts loading only from `NotLoaded`.
- Retry starts loading only from `Failed`.
- Activation while `Loading` or `Ready` does not issue another request.
- A completion mutates state only when its generation equals the current generation.
- Closing the owning project makes every later completion ineligible.

## ProjectPageRecord

Maintains the one-to-one project/page/tab relationship.

| Field | Type | Rules |
|-------|------|-------|
| project_id | ProjectId | References exactly one `ProjectMetadata` entry |
| page | retained page identity | Created once when the project becomes open |
| tab_id | tab identity | Deterministically derived from ProjectId; unique |

Cardinality:

```text
ProjectMetadata 1 ─── 1 ProjectPageRecord 1 ─── 1 ProjectTab
       │
       └── 1 ProjectLoadState ─── 0..1 ProjectData
```

## ProjectStore

Single application-wide aggregate and source of truth.

| Field | Type | Rules |
|-------|------|-------|
| projects | ordered map ProjectId → ProjectMetadata | Unique IDs; order matches tabs/session |
| pages | map ProjectId → ProjectPageRecord | Same key set as projects after restoration completes |
| active_project | optional ProjectId | Must reference an open project |
| session_path | path | Fixed per-user application data location |

Invariants:

- `projects`, `pages`, and visible tabs have identical project ID sets.
- At most one project is active.
- Removing a project removes the corresponding page and tab in the same application update.
- Metadata updates preserve page identity.

## OpenProjectSession

Versioned persisted restoration state.

| Field | Type | Rules |
|-------|------|-------|
| version | integer | Current schema version; unsupported versions fail safely |
| projects | ordered list of SessionProject | Duplicates removed during restoration, first occurrence wins |
| active_project | optional persisted identity | Falls back to first available project if missing/invalid |

### SessionProject

| Field | Type | Rules |
|-------|------|-------|
| identity_key | string | Last validated stable identity |
| repository_path | path string | Must round-trip supported platform paths |
| display_name | string | Startup label fallback before reinspection |

Persistence rules:

- Write to a sibling temporary file, flush, then replace the prior session file.
- A missing file produces an empty session.
- Malformed or unsupported content produces an empty session plus a diagnostic; it does not stop application startup.
- Persist open order and active identity after successful state changes, not heavyweight data.
