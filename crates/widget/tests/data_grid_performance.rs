use std::time::{Duration, Instant};
use widget::{DataGrid, DataGridCell, DataGridColumn, DataGridRow};

#[test]
fn ten_thousand_rows_are_configured_without_eager_rendering() {
    let start = Instant::now();
    let grid = DataGrid::new(vec![DataGridColumn::new("value", "Value")], 10_000, |i| {
        DataGridRow::new([DataGridCell::text(i.to_string())])
    })
    .unwrap();
    assert_eq!(grid.row_count(), 10_000);
    assert!(start.elapsed() < Duration::from_millis(100));
}
