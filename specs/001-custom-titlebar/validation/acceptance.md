# Acceptance Validation

**Date**: 2026-09-05  
**Current host**: Windows

| Criterion | Status | Evidence or gap |
|---|---|---|
| SC-001 | NOT RUN | The ten-launch matrix requires usable runners for every reference platform. |
| SC-002 | NOT RUN | Windows probe launched, but interactive automation could not attach; Linux and macOS were unavailable. |
| SC-003 | NOT RUN | Frozen native reference captures do not yet exist. Project-authored SVG prototypes are not accepted as references. |
| SC-004 | NOT RUN | Multi-monitor 100%, 150%, and 200% validation is pending. |
| SC-005 | PARTIAL | GPUI roles, names, focus entry, and accessible Click handlers are implemented; Narrator, VoiceOver, and Orca execution is pending. |
| SC-006 | PARTIAL | `ZIT_SIMULATE_STARTUP_FAILURE=1` selects a visible GPUI startup-error window; native failure and cancellation execution remain pending. |
| SC-007 | PARTIAL | Exact menu labels/order, disabled commands, and keyboard state transitions pass deterministic tests; native pointer, focus, About, and process Exit scenarios remain pending. |
| SC-008 | NOT RUN | Minimum-size and supported-scale visual checks are pending. |
| SC-009 | PARTIAL | `text_xs`, appearance-aware lightened surfaces, pointer cursors, and hover backgrounds are implemented; native light/dark pointer inspection remains pending. |

The implementation is ready for native evidence collection, but T006-T012 and the platform
acceptance tasks remain blockers. No missing platform result is treated as PASS.
