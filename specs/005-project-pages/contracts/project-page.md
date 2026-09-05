# UI Contract: Project Tabs and Pages

## Purpose

This contract defines observable behavior between the application-wide project collection, project tabs, and retained project pages. It is testable without depending on a particular visual layout.

## Startup contract

1. Each restored unique project produces one tab and one retained page.
2. Tab order matches the persisted open-project order.
3. Tab labels are available from lightweight metadata before history is loaded.
4. The persisted active project is selected when valid; otherwise the first available project is selected.
5. No inactive page requests heavyweight data.
6. With no projects, the content area exposes an empty-project state.

## Selection contract

When a tab is selected:

1. Its existing page becomes the only visible project page.
2. The global active-project identity changes to that tab's project.
3. `NotLoaded` changes immediately to `Loading` and exactly one request begins.
4. `Loading` and `Ready` do not begin another request.
5. The active session identity is scheduled for persistence.

## Page state contract

| State | Required presentation | Available actions |
|-------|-----------------------|-------------------|
| NotLoaded | Lightweight project summary; normally transient once active | None |
| Loading | Project identity and explicit loading feedback | Tab switching and close remain available |
| Ready | Project identity and commit history; an empty repository is a valid empty result | Tab switching and close |
| Failed | Project identity, understandable project-specific error | Retry, tab switching, and close |

## Retry contract

1. Retry is offered only for `Failed`.
2. Retry reuses the same page and tab identity.
3. Retry changes the page to `Loading` immediately.
4. Repeated retry/activation while loading causes no duplicate request.
5. Only the newest load generation may transition the page to `Ready` or `Failed`.

## Close contract

1. A close request continues to respect the existing tab close filter/event sequence.
2. Successful close removes the project from the global open collection, its retained page, and its tab.
3. A completion for the closed page is ignored and cannot recreate it.
4. The next active project follows the tab bar's existing right-neighbor-then-left-neighbor rule.
5. The updated open order and active identity are persisted.

## Metadata update contract

1. A metadata update changes the associated tab label and page summary together.
2. It does not create a new tab or page.
3. Availability failure is scoped to the associated project.
4. Metadata access never implicitly walks repository history.

## Observable project-store events

| Event | Required payload | Consumer behavior |
|-------|------------------|-------------------|
| Restored | ordered project IDs, active ID | Shell creates/reconciles retained tabs and pages |
| MetadataChanged | project ID | Associated tab/page refreshes without identity change |
| ActiveChanged | optional project ID | Shell selection and session state reconcile |
| ProjectRemoved | project ID, next active ID | Shell removes matching retained tab/page |

Events carry identities rather than heavyweight data. The page reads only its own state and metadata.

## Accessibility contract

- Each tab exposes the project display name, selected state, position, and set size.
- Loading, ready-empty, and failed states expose distinct descriptive text.
- Retry is keyboard focusable, named for the failed project, and responds to accessible click activation.
- An unavailable repository message identifies the affected project without exposing unrelated filesystem details.
