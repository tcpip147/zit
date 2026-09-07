# Research: Open Project

## Decision: Reuse the existing project store insertion boundary

**Rationale**: `ProjectStore` already owns metadata, pages, active-project state, session persistence, and store events.

**Alternatives considered**: Shell-only tabs would bypass persistence and page creation.

## Decision: Use the existing gix repository loader

**Rationale**: The constitution requires gix for Git functionality, and the loader already produces project metadata.

**Alternatives considered**: Invoking the Git executable was rejected as unnecessary.

## Decision: Use a native folder dialog

**Rationale**: Folder selection is an OS UI capability and must work across supported desktop platforms.

**Alternatives considered**: A text-only field and platform-specific dialog code were rejected.

## Decision: Deduplicate using canonical paths

**Rationale**: Relative paths and aliases can identify the same repository.

**Alternatives considered**: Raw string comparison was rejected.
