# Contract: Component Tab Bar API

## Public types

`TabId` is cloneable and comparable. `TabSpec` supplies id, project name, SVG asset path, and an
already-created GPUI page entity. `TabBar` is focusable and renderable. `TabBarEvent` reports
`SelectionChanged`, `CloseRequested`, and `Closed` using stable IDs.

## Host operations

| Operation | Result |
|---|---|
| construct(entries, active id) | Validate; select first when requested id is absent/invalid |
| insert(entry, activate) | Reject duplicate; append and optionally select |
| select(id) | False if unknown; idempotent if active; otherwise show and reveal page |
| request_close(id) | Emit request; remove only after host acceptance |
| close(id) | Apply neighbor selection and emit events |
| entries / active_id / page | Read-only inspection without mutable internal order |

No repository type or global singleton is exposed. The host may synchronously veto closure.

## Input contract

- Tab click selects; close click requests closure and consumes the click.
- Left/Right/Home/End move logical focus; Enter/Space selects; close action targets focused/active.
- Wheel input scrolls horizontally only when the offset can move in that direction.
- Every selection path reveals the active item.

## Presentation and accessibility contract

- Labels use `text_xs`; active/inactive/hover/focus and light/dark states are distinct.
- SVG, name, and X SVG render in order with fixed icon/close space.
- Container exposes TabList. Tabs expose Tab, complete name, position/count, and selected state.
  Close controls expose Button and `Close <project name>`.
- Accessible Click shares the select/close transition.
