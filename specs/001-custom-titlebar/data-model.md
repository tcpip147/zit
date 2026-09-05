# Data Model: Zit Application Shell

All state is process-local. No database, repository model, or persistence is introduced.

| Entity | Fields | Ownership and validation |
|---|---|---|
| ShellState | platform profile, window snapshot, menu state, About state | One per GPUI window entity |
| WindowSnapshot | active, maximized, fullscreen, scale, appearance, capabilities | Read from GPUI; never infer success solely from a button click |
| PlatformProfile | platform, control order, icon assets, style metrics, reference identity | Exactly one profile; all assets resolve at startup |
| MenuDefinition | id, label, ordered item IDs | File and Help only; immutable definitions |
| MenuItem | command, label, enabled | Repository entries always disabled in this scope |
| MenuState | open heading or none, highlighted item, previous focus | At most one open popup; highlighted keyboard target must be enabled |
| AboutState | visible, return focus | At most one dialog; name Zit and host-provided package version |
| PressState | target, pointer identity, inside target | Release executes only the same enabled target while inside |
| MenuPresentation | typography, appearance surface, foreground, border, hover surface, pointer cursor | Uses `text_xs`; derived from light/dark appearance without changing command state |

## Commands

NewRepository, OpenRepository, CloneRepository, ExitApplication, ShowAbout, CloseWindow,
MinimizeWindow, ToggleMaximize, ToggleFullscreen. Repository commands are rejected before
host dispatch. Maximize and fullscreen are separate commands with platform capability checks.

## State Transitions

- Closed menu -> open File/Help: remember focus and select the first enabled item.
- Open File -> open Help: atomically replace the open heading; never leave both popups visible.
- Open menu -> Escape: dismiss and focus its heading.
- Open menu -> outside click or focus loss: dismiss without dispatch; do not steal focus on blur.
- Disabled item -> activation attempt: no command and no state-changing side effect.
- About activation -> dismiss menu, open modal overlay, move focus into it.
- About dismissal -> close overlay and restore a valid main-window focus target.
- Window action -> request GPUI operation -> observe actual window state -> refresh snapshot/icon.
- Pointer press -> release outside/cancellation/focus loss: clear pressed state without action.
- Scale/appearance change -> select updated style and rerender without changing menu definitions.
- Pointer entry over File, Help, or any submenu row -> use the hand cursor and hover surface;
  pointer exit restores the appearance surface without changing enabled state.

## Layout Invariants

Menu headings, popup hit areas, resize edges, draggable space, and control hit areas cannot
ambiguously handle the same event. Controls and menus stop propagation before the drag region.
Popup bounds clamp to the available client area; minimum window size preserves all labels and
64 logical pixels of draggable space. About overlays suppress underlying menu/control activation.
Focus handles must not outlive their entity; fallback focus is the main shell.


