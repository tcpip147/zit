# New Repository Validation

1. Run `cargo check -p zit`.
2. Run `cargo test -p zit --test menu_contract --test window_contract --test project_store_contract`.
3. Select File > New Repository.
4. Enter a project name and choose a parent folder.
5. Select Create and verify `.git` is created inside the selected folder and the active project tab uses the entered name.
6. Repeat with the same folder and verify no overwrite occurs.
7. Try blank, invalid, and cancelled input and verify recoverable behavior.
