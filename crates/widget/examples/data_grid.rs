use std::rc::Rc;

use gpui::{
    App, Bounds, Context, Entity, Render, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    size,
};
use gpui_platform::application;
use widget::{DataGrid, DataGridCell, DataGridColumn, DataGridRow};

struct Example {
    grids: Vec<Entity<DataGrid>>,
}
impl Render for Example {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .p_4()
            .gap_4()
            .flex()
            .children(self.grids.clone())
    }
}

fn columns() -> Vec<DataGridColumn> {
    vec![
        DataGridColumn::new("index", "Index").fixed(px(80.)),
        DataGridColumn::new("value", "Value").divider(false),
    ]
}

fn main() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.), px(500.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                let values: Rc<Vec<String>> = Rc::new(
                    (0..10_000)
                        .map(|i| format!("Virtual row {i}: clipped content remains on one line"))
                        .collect(),
                );
                let large = cx.new({
                    let values = values.clone();
                    move |_| {
                        DataGrid::new(columns(), values.len(), move |i| {
                            DataGridRow::new([
                                DataGridCell::text(i.to_string()),
                                DataGridCell::text(values[i].clone()),
                            ])
                        })
                        .unwrap()
                    }
                });
                let empty =
                    cx.new(|_| DataGrid::new(columns(), 0, |_| DataGridRow::new([])).unwrap());
                cx.new(|_| Example {
                    grids: vec![large, empty],
                })
            },
        )
        .unwrap();
    });
}
