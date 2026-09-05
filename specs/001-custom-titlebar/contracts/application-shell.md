# UI Contract: Zit Application Shell

## Application Boundary

The zit executable calls `gpui_platform::application()` directly, supplies application
name/version and bundled assets, creates its root GPUI entity, opens one window, and handles
semantic commands and application exit inside `crates/zit`. There is no intermediate shell crate.
No public HTTP, IPC, or Git command-line interface is introduced.

## Menu Contract

| Heading | Item label | Initial state | Result |
|---|---|---|---|
| File | New Repository | Disabled | No dispatch |
| File | Open ... | Disabled | No dispatch |
| File | Clone ... | Disabled | No dispatch |
| File | Exit | Enabled | Quit application |
| Help | About | Enabled | Show Zit and package version |

Labels contain three ASCII periods and the specified preceding space. No extra menu items.
File precedes Help. macOS places both after the left-hand custom SVG controls; other platforms
place them before the right-hand controls. No application title text occupies the menu bar.
The native window title may remain Zit for task switching and accessibility.

Menu-bar and popup text use GPUI `text_xs`. Their surfaces are lighter than the surrounding
content while remaining coordinated with the active light or dark title-bar appearance. File,
Help, and every submenu row show a hand pointer and change background color on hover. Disabled
repository rows may show hover feedback but never dispatch their commands.

## Input and Focus

Tab navigation can reach the menu bar. Left/Right switches headings; Down or Enter opens the
focused heading. Up/Down traverses enabled items, wrapping within the popup. Enter activates.
Escape dismisses and restores heading focus. Pointer activation and assistive activation use
the same semantic dispatch path. Popup dismissal on outside click must consume that click so it
cannot accidentally move the window or activate an obscured control. Blur closes popups.

Expose menu bar, menu, menu item, button, and dialog roles through GPUI accessibility facilities,
with names, expanded state, disabled state, and current maximize/restore action. About traps
focus while visible and dismisses through Escape or its Close button.

## Window Contract

Windows/Linux: minimize, maximize/restore, close. macOS: close, minimize, enter/exit full screen.
SVGs are mandatory, including macOS traffic-light shapes and state symbols. Native controls
must not remain underneath or beside custom controls. The OS owns actual window state.

Window controls dispatch explicit GPUI click handlers. On Windows, maximize calls GPUI
`zoom_window()` while restore calls Win32 `ShowWindowAsync` with `SW_RESTORE` through the raw
window handle, because the pinned GPUI Windows `zoom()` implementation only maximizes. This
target-only adapter must not take ownership of the handle or expand into a replacement platform layer.

Close removes the current window. Exit quits the process on every platform. On macOS, closing
the last window may leave the application alive; reopening through platform activation must
produce a usable window. On Windows/Linux, closing the last application window quits Zit.

Drag only the empty region; preserve resize borders and configured title-bar double-click
behavior through GPUI. Unsupported operations are disabled. Initial failure to open a window
uses a platform-visible error path that does not rely on that failed window, then exits with
failure. A terminal log alone is insufficient for a desktop launch.

## Asset and Reference Contract

Every icon has a platform/state identity and an SVG viewBox. Bundle assets; do not download them
at runtime. Record provenance/license for upstream assets, or author equivalent geometry against
captured native references. Match native geometry and color; tolerate only antialiasing differences.
Never use screenshot bitmaps as the delivered control icons.

## Coverage Map

| Requirements | Validation |
|---|---|
| FR-001/002 | Launch count, one custom title bar, no duplicate native controls |
| FR-003/004 | SVG inventory and native reference comparisons |
| FR-005/006/007 | Actual native operations, external state change, drag/resize/double-click |
| FR-008/009 | Cancellation, disabled capabilities, scale/theme/minimum-size cases |
| FR-010/015 | Accessibility tree and assistive activation |
| FR-011 | Injected startup failure, visible error, failure exit |
| FR-012/013/014 | Exact menus, ordering, keyboard/pointer dismissal and navigation |
| FR-016/017 | About, process-level Exit, rejected repository dispatch |
| FR-018/019/020 | `text_xs`, appearance surfaces, pointer and hover-state inspection |


