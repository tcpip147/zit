# Research: Project Pages

## Global project ownership

**Decision**: Store an `Entity<ProjectStore>` behind a small GPUI `Global` wrapper and install it once during application startup. Shells and pages obtain clones of the same entity and observe its events.

**Rationale**: GPUI explicitly supports globally registered state and entity observation. An entity provides mutation on the application thread, subscriptions, and deterministic test access while preserving one authoritative collection.

**Alternatives considered**:

- Keep projects directly in `ZitShell`: rejected because it makes metadata window-local rather than application-wide and complicates reopen behavior.
- Process-wide mutex/static: rejected because it bypasses GPUI notifications and adds unnecessary synchronization around UI-owned metadata.

## Repository identity and duplicate prevention

**Decision**: Discover a repository from the supplied worktree path with `gix`, canonicalize its resolved Git directory, and derive `ProjectId` from that normalized location. Persist the user-facing worktree path separately. On case-insensitive platforms, compare normalized identity keys case-insensitively.

**Rationale**: A repository's display name and input path are mutable or ambiguous. The resolved Git directory distinguishes repositories while coalescing path aliases and discovery from subdirectories. Keeping the worktree path preserves useful display and reopening behavior.

**Alternatives considered**:

- Raw input path: rejected because relative paths, subdirectories, symlinks, and case differences create duplicates.
- Repository name: rejected because unrelated repositories commonly share names.
- Current HEAD object ID: rejected because it changes and can be shared by clones.

## Lightweight startup restoration

**Decision**: Read a versioned JSON session containing ordered repository paths and the active project identity. Create metadata/page records synchronously from persisted values, then perform only repository discovery and lightweight availability inspection before or alongside tab construction. Do not walk revisions at startup.

**Rationale**: A small local file is sufficient for the bounded session model and supports human inspection, schema versioning, and isolated recovery. Separating discovery from history guarantees the lazy-load contract.

**Alternatives considered**:

- Persist complete history: rejected because it introduces invalidation and large storage for data already owned by Git.
- Rebuild the session from a fixed list in code: rejected because it cannot restore user state.
- Load all histories during startup: rejected because it violates the primary performance requirement.

## History loading and UI responsiveness

**Decision**: On the transition from `NotLoaded` or `Failed` to `Loading`, submit a blocking repository-history function to GPUI's background executor. The page immediately notifies the UI, awaits the result in an entity-bound task, and applies it only if its project ID and load generation still match.

**Rationale**: `gix` repository traversal is synchronous and may be expensive. The background executor keeps it off the UI thread, while the entity task safely returns the result to GPUI state. A generation token makes retry and late completion deterministic.

**Alternatives considered**:

- Run revision walking in render/activation handlers: rejected because large repositories could freeze interaction.
- Spawn an unmanaged OS thread per activation: rejected because cancellation, test control, and UI handoff become harder.
- Start a new load on every activation: rejected because it wastes work and creates races.

## Revision traversal boundary

**Decision**: Resolve HEAD when present and use `gix` revision walking to produce immutable commit summaries containing object ID, summary, author identity, and authored time needed by the initial page. An unborn branch or repository with no commits yields an empty ready result rather than an error.

**Rationale**: The approved Git backend exposes discovery, `head_id`, and revision walking in the pinned version. Returning view-ready summaries avoids retaining a repository handle across threads or tying the page to borrowed Git objects.

**Alternatives considered**:

- Invoke `git log`: rejected by the constitution and unnecessary for this capability.
- Retain lazy borrowed commit objects in UI state: rejected because their lifetimes and repository access leak into rendering.
- Treat an empty repository as failed: rejected because it is a valid Git project.

## Closing and obsolete work

**Decision**: Removing a project drops its retained page and load task handle. Every completion additionally validates a monotonically increasing generation token before state mutation. Session persistence occurs after open-order or active-project changes using an atomic replace strategy.

**Rationale**: Dropping a task may not stop already running blocking work, so stale-result rejection is required. Atomic replacement prevents a partial session file from breaking the next startup.

**Alternatives considered**:

- Depend only on task cancellation: rejected because blocking work may already be executing.
- Allow completions to update the global store after close: rejected because they could recreate or contaminate visible state.

## Testability

**Decision**: Define a narrow repository loader boundary used by the store/page. Production uses `gix`; tests use deterministic loaders that count requests, delay completion, and inject errors. Integration tests create temporary repositories without invoking the Git CLI.

**Rationale**: Lazy-loading, retry, and race behavior require deterministic scheduling and request counts. The real adapter still needs integration coverage for discovery and empty/non-empty history.

**Alternatives considered**:

- Test only with real large repositories: rejected because timing and contents are nondeterministic.
- Test only the `gix` adapter: rejected because it does not prove UI lifecycle behavior.
