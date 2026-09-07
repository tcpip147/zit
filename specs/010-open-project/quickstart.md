# Open Project Validation

1. Run `cargo check -p zit`.
2. Run `cargo test -p zit --test menu_contract --test project_store_contract`.
3. Launch with `cargo run -p zit`.
4. Select File > Open ... and verify the modal controls.
5. Browse to a Git repository, enter a name, and select Open; verify one active tab.
6. Repeat with the same repository; verify no duplicate tab.
7. Try empty name, missing path, and non-Git folder; verify recoverable errors.
8. Cancel and press Escape; verify no project is added.

## Validation Record

- `cargo test -p zit --test project_store_contract`: the new invalid-path, non-Git-folder,
  duplicate-repository, and activation test passes.
- The existing `seeds_three_distinct_test_repositories_only_for_empty_store` test fails in this
  environment because it expects `D:\Project\zed` for seeded project 2; this is an unrelated
  pre-existing external-path assumption.
- Interactive `cargo run -p zit` validation was not executed in the automated environment.
