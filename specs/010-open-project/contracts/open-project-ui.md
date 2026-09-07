# Open Project UI Contract

- File > Open ... is enabled and selectable.
- The overlay is a modal dialog labelled “Open Project”.
- It contains project-name and folder text inputs.
- Browse opens the OS folder chooser; cancelling preserves the form.
- Open confirms; Cancel and Escape close without project-store changes.
- Invalid confirmation keeps the dialog visible and shows an actionable error.
- Valid confirmation closes the dialog and produces one active project tab.
