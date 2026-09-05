# Quickstart: Component Tab Bar Validation

## Prerequisites

- Workspace Rust toolchain and locked official-Zed GPUI dependencies.
- Graphical runners for pointer, wheel/trackpad, appearance, and accessibility checks.

## Automated checks

```powershell
cargo fmt --all -- --check
cargo check --workspace --all-targets --locked
cargo test -p widget --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
```

Cover duplicate IDs, initial/idempotent selection, inactive/active/final closure, veto, event order,
close propagation, keyboard actions, fallback SVG, both wheel directions/bounds, fit-without-scroll,
resize clamping, and active reveal.

## Example acceptance

Run the `tab_bar` example with 30 distinct pages.

1. Select every tab; confirm one matching page and active tab (SC-001).
2. Close inactive/active tabs at start, middle, and end; verify neighbor rules (SC-002).
3. Resize to overflow, use vertical wheel and horizontal trackpad, and verify bounds (SC-003/004).
4. Test empty, duplicate, and long names; confirm SVG/name/X order and no overlap (SC-005).
5. Inspect `text_xs`, hover, focus, active state, and cursor in light/dark appearance (SC-006).
6. Select and close using keyboard and assistive technology; inspect roles and state (SC-007).
7. Verify visible feedback remains below 100 ms (SC-008).

Record platform, appearance, scale, input device, timing, expected/actual result, and evidence path.
