#[test]
fn grid_declares_complete_table_semantics() {
    let source = include_str!("../src/data-grid.rs");
    for semantic in [
        "Role::Grid",
        "Role::Row",
        "Role::ColumnHeader",
        "Role::Cell",
        "aria_row_count",
        "aria_column_count",
        "aria_row_index",
        "aria_column_index",
    ] {
        assert!(source.contains(semantic), "missing {semantic}");
    }
}

#[test]
fn accessibility_element_ids_are_unique_by_grid_row_and_column() {
    let source = include_str!("../src/data-grid.rs");
    assert!(source.contains(".id((\"data-grid\", cx.entity_id()))"));
    assert!(source.contains(".id((\"data-grid-row\", index))"));
    assert!(source.contains(".id((\"data-grid-cell\", col))"));
    assert!(!source.contains(".id(\"data-grid-cell\")"));
}
