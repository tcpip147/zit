# Research: Component Tab Bar

## Reusable boundary

**Decision**: Implement `TabBar` as an entity in `widget`, storing stable `TabId` values and
host-supplied GPUI page entities and emitting typed events.
**Rationale**: Selection, focus, and scroll state must survive renders without repository coupling.
**Alternatives considered**: Render-once state is insufficient; Zed's higher-level `ui::TabBar`
adds a large internal dependency and does not own pages.

## Selection and closure

**Decision**: Switch preserved page entities by identity. Inactive closure preserves selection;
active closure selects the pre-removal right neighbor, otherwise left. A host close request may veto.
**Rationale**: This preserves page state and supports unsaved-work policy outside the component.
**Alternatives considered**: Index identity breaks on removal; recreating pages loses state.

## Horizontal overflow

**Decision**: Track an `overflow_x_scroll` row with GPUI `ScrollHandle`. Convert wheel delta using
a 16-pixel line height, prefer x delta, otherwise map y to x, clamp between `max_offset().x` and
zero, and call `scroll_to_item` after selection.
**Rationale**: GPUI already provides measured viewport, child bounds, maximum offset, and reveal.
**Alternatives considered**: Indefinite shrinking violates readability; an untracked offset breaks on resize.

## Input and accessibility

**Decision**: Use roving focus with previous/next, Home/End, Enter/Space selection, and close action.
Expose TabList, Tab, and Button roles, names, position/count, selected state, and accessible Click.
Stop close-button propagation before the tab body.
**Rationale**: Pointer, keyboard, and assistive paths must share transitions without nested click leakage.
**Alternatives considered**: Pointer-only controls violate the specification.

## Typography and assets

**Decision**: Apply `text_xs`; render project SVG paths and bundled authored close/fallback SVGs.
Set SVG text color explicitly because pinned GPUI paints path SVGs only with element text color.
**Rationale**: This matches workspace behavior and prevents invisible `currentColor` icons.
**Alternatives considered**: Raster icons violate the requirement; inherited-only color is unreliable.
