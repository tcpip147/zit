# Research: Custom Title Bar and Menu Bar

Research date: 2026-09-05. The workspace resolves official Zed commit `5a9b9558` with Rust
1.97.1 and gix 0.87.1. Dependency resolution and `cargo check --workspace --locked` succeeded
on Windows; native interaction and cross-platform behavior remain unverified.

## Dependency and Application Bootstrap

**Decision**: Use Rust edition 2024 with the official Zed Git dependency declaration required
by the constitution. The resolved Zed commit uses Rust 1.97.1. Use the same Git source
for gpui_platform, with font-kit, wayland, and x11 desktop features as appropriate. Resolve gix
0.87.1 and retain the exact resolution in Cargo.lock. The `zit` binary calls
`gpui_platform::application()` directly; no intermediate window-shell crate is used.

**Rationale**: The current official window example starts through gpui_platform::application.
Explicit platform feature selection avoids assuming that gpui_platform enables desktop backends
by default. gix is required by the constitution but has no runtime operation in this scope.

**Alternatives considered**: Published GPUI crates violate the required dependency declaration;
copying imported component lock assumptions is not evidence of compatibility; Git CLI is unnecessary.

Sources: [toolchain](https://raw.githubusercontent.com/zed-industries/zed/main/rust-toolchain.toml),
[platform manifest](https://raw.githubusercontent.com/zed-industries/zed/main/crates/gpui_platform/Cargo.toml),
[GPUI manifest](https://raw.githubusercontent.com/zed-industries/zed/main/crates/gpui/Cargo.toml),
[gix documentation](https://docs.rs/gix/0.87.1/gix/).

## Custom Chrome and Native Capabilities

**Decision**: Implement platform policy behind zit::platform. Use GPUI window state
and operations, keeping drawing and menu dispatch in GPUI. On macOS, start with a transparent
native titlebar preserving window capabilities and a narrow AppKit adapter through GPUI's raw
window handle to hide native traffic-light buttons. The adapter must not create another window,
replace the renderer, or fork GPUI. Keep unsafe code isolated, main-thread-only, and tied to the
GPUI window lifetime. Use target-specific objc2/AppKit dependencies compatible with the resolved
backend if the GPUI revision still lacks a public hide-controls facility.

**Rationale**: Current TitlebarOptions exposes title, appears_transparent, and traffic_light_position.
The macOS backend's titlebar: None path omits resizable/minimizable style flags. Transparent
chrome preserves capabilities but does not itself hide native controls. A prototype must prove
hiding survives activation and fullscreen transitions without duplicate controls.

**Alternatives considered**: titlebar: None is an example, not proof of full native capability.
Leaving native controls visible violates the custom-SVG requirement. GPUI forks are prohibited.
If the bounded adapter cannot satisfy the contract, record a blocker and pursue an official
upstream capability instead of claiming completion.

Sources: [platform options](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/platform.rs),
[macOS backend](https://github.com/zed-industries/zed/blob/main/crates/gpui_macos/src/window.rs#L862),
[official example](https://github.com/zed-industries/zed/blob/main/crates/gpui/examples/window.rs).

## Operations, Hit Testing, and Accessibility

**Decision**: Map semantic actions to Window minimize_window, zoom_window, toggle_fullscreen,
remove_window and state queries. Use platform-supported move/resize and native hit-testing paths;
do not assume start_window_move is the Windows solution. On macOS use app_owns_titlebar_drag
when supported. Linux requests client decorations, with actual compositor behavior verified.
Observe bounds, focus, appearance, and actual state; subscriptions have window-entity lifetimes.

**Windows implementation update (2026-09-05)**: The pinned GPUI Windows backend implements
`zoom()` with `SW_MAXIMIZE` and does not restore an already-maximized window. Zit therefore keeps
GPUI `zoom_window()` for maximize and uses a target-only `ShowWindowAsync(SW_RESTORE)` call for
restore, retrieving but not owning the HWND through GPUI's `HasWindowHandle` implementation.
The adapter is isolated in `crates/zit/src/platform.rs` and uses `raw-window-handle` 0.6 plus
`windows` 0.62. This exception closes a missing GPUI capability without replacing GPUI.

**Rationale**: Platform requests can be ignored or have different semantics. Windows caption hit
regions must exclude menu headings and controls. GPUI exposes accessibility semantics through
AccessKit; labels alone are insufficient without roles, state, focus, and action handling.

**Alternatives considered**: Synthetic cached maximize state drifts after operating-system actions;
whole-titlebar dragging conflicts with menus; relying on tooltips fails assistive interaction.

Sources: [window API](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/window.rs),
[accessibility guide](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/_accessibility.rs).

## Menus, Dialogs, and Startup Errors

**Decision**: Render the menu bar and popups as GPUI entities/overlays in the application window,
including macOS. Store immutable command definitions separately from open/focus state. About is
a modal GPUI overlay using the package version supplied by zit. Exit uses application quit;
Close uses window removal. Repository items remain disabled.

**Rationale**: A native global menu would not replace the visible title text as requested.
No text editor component is needed. Initial window failure needs a visible error independent
of the failed window: first use an available GPUI platform prompt path; if it requires an open
window, use a narrow target-native error dialog adapter. Prove this path during the native spike.

**Alternatives considered**: Silent failure, a terminal-only message, or a second unrelated UI
framework does not meet the desktop error contract. Log diagnostic details in addition to the dialog.

## Reference Profiles and Asset Provenance

**Decision**: Freeze the following target profiles for capture. They are selected test targets,
not assertions that their screenshots have already been collected.

| Platform | Reference | Appearance | Scale matrix |
|---|---|---|---|
| Windows | Windows 11 24H2, default caption controls | Light/dark, default accent | 100%, 150%, 200% |
| macOS | macOS 15.6, default traffic lights | Light/dark, default accent | Available 1x/2x backing scales; record 150% as unsupported if unavailable |
| Linux | Ubuntu 24.04, GNOME 46, Adwaita; minimize/maximize/close right | Light/dark | X11 and Wayland separately; record actual support for 100%, 150%, 200% |

Capture normal/maximized or fullscreen, focused/unfocused, default/hover/pressed/disabled states.
Before final SVG creation, store reference PNGs and a manifest under this feature's future
references/ directory, recording exact OS build and package versions, session, scale, theme,
control metrics, capture method, filename, and SHA-256. Do not mix different reference builds.
Use native reference applications with matching caption-control profiles, not Zit screenshots.

**Rationale**: Native appearance has no single cross-version pixel definition. SVG geometry and
state colors must be compared with fixed evidence. Bundle authored SVGs; document licensing
and provenance if adapting upstream assets. No image generation is needed for exact vector controls.

**Alternatives considered**: Guessing native pixels or treating GNOME as all Linux themes makes
SC-003 unverifiable. Shipping raster screenshots as icons violates FR-003.

**Evidence gap**: No native reference screenshots were captured in this session. Exact build
metadata and image hashes cannot be truthfully supplied yet. The spec explicitly requires these
during planning, so the reference-capture planning gate remains open. All architectural decisions
are recorded, but this evidence gap prevents an unconditional ready-for-implementation claim.

