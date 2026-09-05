<!--
Sync Impact Report
- Version change: 1.1.0 -> 1.2.0 (new Rust workspace structure principle).
- Modified principles: none; Principles I and II remain unchanged.
- Added sections: III. Rust Workspace and Application Entry Point.
- Updated sections: Compliance Review (language, workspace, and entry-point checks).
- Removed sections: none.
- Dependent templates: unchanged; consume the constitution at runtime.
- Follow-up TODOs: none.
-->

# Zit Constitution

## Core Principles

### I. GPUI and Git Backend Foundation (NON-NEGOTIABLE)

The primary goal of this project is to build a high-performance Git GUI client application
using the GPUI framework that powers the Zed editor.

GPUI MUST remain the core UI foundation of the application and MUST be declared as a Git
dependency on the official Zed repository using:

```toml
gpui = { git = "https://github.com/zed-industries/zed" }
```

The project MUST also include the `gix` crate as its Git implementation layer. Git-related
functionality SHOULD use `gix` and its ecosystem rather than invoking the Git command-line
executable directly, unless a specific capability is not provided by `gix` and an exception
is explicitly justified.

The project MUST NOT replace GPUI with an unrelated or incompatible UI framework, fork,
reimplementation, or substitute. The project MUST NOT replace `gix` with an unrelated Git
implementation without an approved architectural change.

All UI functionality MUST be designed around GPUI's application, view, entity, event,
rendering, and platform facilities. Git operations MUST be designed around `gix` and its
Rust-native Git functionality.

This requirement ensures that the application remains a high-performance, GPUI-native Git
client while maintaining a Rust-native Git implementation layer.

### II. English-Language Specifications

All feature specifications, including every `spec.md`, MUST be written in English,
regardless of the language used to request the feature. This requirement covers titles,
summaries, user stories, acceptance scenarios, requirements, success criteria, assumptions,
and any other explanatory text within the specification.

Non-English feature requests MUST be translated faithfully into English when recorded in
specifications, including the Input section, without changing their meaning or scope.
New specifications and amendments to existing specifications MUST follow this rule.
Existing non-English specifications MUST be translated before further planning or implementation.

English provides a consistent language for reviewing requirements and maintaining specifications
across the project.

### III. Rust Workspace and Application Entry Point

The application MUST be implemented in Rust. The project MUST use a Cargo workspace defined
by the repository-root `Cargo.toml`. Project modules MUST be organized as member crates under
the repository-root `crates/` directory, each with its own `Cargo.toml`.

The primary application package and executable crate MUST be named `zit` and located at
`crates/zit/`. Its executable entry point MUST be `crates/zit/src/main.rs`; a library-only
crate MUST NOT replace this application entry point. Additional module crates MUST reside
under `crates/` and be included in the root workspace membership.

The required baseline layout is:

```text
Cargo.toml
crates/
    zit/
        Cargo.toml
        src/
            main.rs
```

This structure establishes a consistent Rust build foundation, explicit module boundaries,
and a predictable application entry point while preserving the GPUI and gix architecture.

## Architecture Constraints

Dependency declarations and architectural designs MUST preserve the foundations defined in
Principle I. Any exception permitting a Git command-line invocation MUST document the specific
capability unavailable through `gix` and its ecosystem, the reason the invocation is necessary,
and the scope of the fallback. Such an exception MUST NOT authorize replacement of `gix` as
the project's Git implementation layer.

A proposed replacement of `gix` MUST be documented as an architectural change and approved by
the project maintainer before implementation. GPUI substitutions are prohibited by this
constitution.

## Compliance Review

Specifications, implementation plans, and code reviews MUST verify compliance with all principles.
Reviews of dependency changes MUST check the official Zed Git dependency declaration and the
presence of `gix`. Reviews of UI and Git functionality MUST check use of the corresponding
framework facilities and any documented Git command-line exceptions.
Specification reviews MUST verify that all specification prose is in English, including
translated user input and newly amended sections.
Architecture and code reviews MUST verify Rust application code, a root Cargo workspace,
module crates under `crates/`, workspace membership, and the `zit` executable entry point at
`crates/zit/src/main.rs`.

Noncompliant changes MUST be corrected before acceptance, or, where permitted, receive the
architectural approval required by this constitution before implementation.

## Governance

This constitution takes precedence over conflicting project guidance. Amendments MUST document
the proposed change, rationale, compliance impact, and any required migration. Amendments MUST
receive project maintainer approval and update this document's version, amendment date, and
Sync Impact Report.

Versions follow semantic versioning: MAJOR for incompatible principle or governance changes,
MINOR for new principles or materially expanded guidance, and PATCH for clarifications that do
not change requirements. The original ratification date MUST be preserved in later amendments.
Every specification, plan, and change review MUST assess compliance with the current constitution.

**Version**: 1.2.0 | **Ratified**: 2026-09-05 | **Last Amended**: 2026-09-05


