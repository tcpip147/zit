use gpui::px;
use widget::{DataGrid, DataGridCell, DataGridColumn, DataGridError, DataGridRow};

fn row(_: usize) -> DataGridRow {
    DataGridRow::new([DataGridCell::text("value")])
}

#[test]
fn validates_columns_and_accepts_empty_rows() {
    assert!(DataGrid::new(vec![DataGridColumn::new("name", "Name")], 0, row).is_ok());
    assert!(matches!(
        DataGrid::new(vec![], 0, row),
        Err(DataGridError::NoColumns)
    ));
    assert!(matches!(
        DataGrid::new(vec![DataGridColumn::new("", "Name")], 0, row),
        Err(DataGridError::EmptyColumnId)
    ));
    assert!(matches!(
        DataGrid::new(
            vec![
                DataGridColumn::new("same", "A"),
                DataGridColumn::new("same", "B")
            ],
            0,
            row
        ),
        Err(DataGridError::DuplicateColumnId(_))
    ));
    assert!(matches!(
        DataGrid::new(
            vec![DataGridColumn::new("bad", "Bad").fixed(px(0.))],
            0,
            row
        ),
        Err(DataGridError::InvalidColumnWidth)
    ));
}

#[test]
fn keeps_order_and_independent_state() {
    let a = DataGrid::new(
        vec![DataGridColumn::new("a", "A"), DataGridColumn::new("b", "B")],
        10_000,
        row,
    )
    .unwrap();
    let b = DataGrid::new(vec![DataGridColumn::new("b", "B")], 1, row).unwrap();
    assert_eq!(
        a.columns()
            .iter()
            .map(|c| c.id.as_ref())
            .collect::<Vec<_>>(),
        ["a", "b"]
    );
    assert_eq!(a.row_count(), 10_000);
    assert_eq!(b.row_count(), 1);
    assert!(!a.is_scrollbar_visible() && !b.is_scrollbar_visible());
}

#[test]
fn generic_source_has_no_project_or_git_domain_dependency() {
    let source = include_str!("../src/data-grid.rs");
    for forbidden in ["ProjectPage", "CommitSummary", "RepositoryLoader", "gix::"] {
        assert!(
            !source.contains(forbidden),
            "found domain reference {forbidden}"
        );
    }
}
