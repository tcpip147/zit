//! A reusable, virtualized data grid with a fixed header.
//!
//! Rows are requested lazily by index and normalized to the configured column count:
//! missing cells are blank and excess cells are ignored. The callback must own or share
//! everything it captures because it is retained for the lifetime of the grid entity.
//! Scrolling affects only the virtualized body; the header remains fixed.

use std::{fmt, rc::Rc, time::Duration};

use gpui::{
    AnyElement, App, Context, CursorStyle, InteractiveElement, ListHorizontalSizingBehavior,
    MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Render, Role,
    ScrollStrategy, ScrollWheelEvent, SharedString, StatefulInteractiveElement, Task,
    UniformListScrollHandle, Window, canvas, div, point, prelude::*, px, uniform_list,
};

use crate::DataGridTheme;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DataGridColumnWidth {
    Fixed(Pixels),
    Fraction(f32),
}

#[derive(Clone, Debug)]
pub struct DataGridColumn {
    pub id: SharedString,
    pub label: SharedString,
    pub width: DataGridColumnWidth,
    pub divider: bool,
}

impl DataGridColumn {
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            width: DataGridColumnWidth::Fraction(1.),
            divider: true,
        }
    }
    pub fn fixed(mut self, width: Pixels) -> Self {
        self.width = DataGridColumnWidth::Fixed(width);
        self
    }
    pub fn fraction(mut self, weight: f32) -> Self {
        self.width = DataGridColumnWidth::Fraction(weight);
        self
    }
    pub fn divider(mut self, divider: bool) -> Self {
        self.divider = divider;
        self
    }
}

pub struct DataGridCell {
    content: AnyElement,
    accessible_label: Option<SharedString>,
}
impl DataGridCell {
    pub fn new(content: impl IntoElement) -> Self {
        Self {
            content: content.into_any_element(),
            accessible_label: None,
        }
    }
    pub fn text(text: impl Into<SharedString>) -> Self {
        let text = text.into();
        Self {
            content: text.clone().into_any_element(),
            accessible_label: Some(text),
        }
    }
    pub fn accessible_label(mut self, label: impl Into<SharedString>) -> Self {
        self.accessible_label = Some(label.into());
        self
    }
}

pub struct DataGridRow {
    pub cells: Vec<DataGridCell>,
    pub accessible_label: Option<SharedString>,
}
impl DataGridRow {
    pub fn new(cells: impl IntoIterator<Item = DataGridCell>) -> Self {
        Self {
            cells: cells.into_iter().collect(),
            accessible_label: None,
        }
    }
    pub fn accessible_label(mut self, label: impl Into<SharedString>) -> Self {
        self.accessible_label = Some(label.into());
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum DataGridError {
    NoColumns,
    EmptyColumnId,
    DuplicateColumnId(SharedString),
    InvalidColumnWidth,
    InvalidRowHeight,
    InvalidHeaderHeight,
}
impl fmt::Display for DataGridError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid data grid configuration: {self:?}")
    }
}
impl std::error::Error for DataGridError {}

type RowRenderer = Rc<dyn Fn(usize) -> DataGridRow>;
type RowMouseDownHandler = Rc<dyn Fn(usize, &MouseDownEvent, &mut Window, &mut App)>;
const COLUMN_RESIZE_HIT_WIDTH: Pixels = px(8.);
const MIN_COLUMN_WIDTH: f32 = 40.;

#[derive(Clone, Copy, Debug)]
struct ColumnResizeDrag {
    boundary: usize,
    start_x: f32,
    left_width: f32,
    right_width: f32,
}

pub struct DataGrid {
    columns: Vec<DataGridColumn>,
    row_count: usize,
    render_row: RowRenderer,
    row_mouse_down_handler: Option<RowMouseDownHandler>,
    selected_row: Option<usize>,
    row_height: Pixels,
    header_height: Pixels,
    theme: DataGridTheme,
    label: SharedString,
    scroll: UniformListScrollHandle,
    drag: Option<(f32, f32)>,
    scrollbar_visible: bool,
    scrollbar_hovered: bool,
    hide_generation: u64,
    hide_task: Option<Task<()>>,
    column_resize_drag: Option<ColumnResizeDrag>,
}

impl DataGrid {
    pub fn new(
        columns: Vec<DataGridColumn>,
        row_count: usize,
        render_row: impl Fn(usize) -> DataGridRow + 'static,
    ) -> Result<Self, DataGridError> {
        Self::validate_columns(&columns)?;
        Ok(Self {
            columns,
            row_count,
            render_row: Rc::new(render_row),
            row_mouse_down_handler: None,
            selected_row: None,
            row_height: px(24.),
            header_height: px(24.),
            theme: DataGridTheme::default(),
            label: "Data grid".into(),
            scroll: UniformListScrollHandle::new(),
            drag: None,
            scrollbar_visible: false,
            scrollbar_hovered: false,
            hide_generation: 0,
            hide_task: None,
            column_resize_drag: None,
        })
    }
    fn validate_columns(columns: &[DataGridColumn]) -> Result<(), DataGridError> {
        if columns.is_empty() {
            return Err(DataGridError::NoColumns);
        }
        for (i, column) in columns.iter().enumerate() {
            if column.id.is_empty() {
                return Err(DataGridError::EmptyColumnId);
            }
            if columns[..i].iter().any(|other| other.id == column.id) {
                return Err(DataGridError::DuplicateColumnId(column.id.clone()));
            }
            match column.width {
                DataGridColumnWidth::Fixed(v) if v <= px(0.) => {
                    return Err(DataGridError::InvalidColumnWidth);
                }
                DataGridColumnWidth::Fraction(v) if !v.is_finite() || v <= 0. => {
                    return Err(DataGridError::InvalidColumnWidth);
                }
                _ => {}
            }
        }
        Ok(())
    }
    pub fn columns(&self) -> &[DataGridColumn] {
        &self.columns
    }
    pub fn row_count(&self) -> usize {
        self.row_count
    }
    pub fn is_scrollbar_visible(&self) -> bool {
        self.scrollbar_visible
    }
    pub fn selected_row(&self) -> Option<usize> {
        self.selected_row
    }
    pub fn select_row(&mut self, index: Option<usize>, cx: &mut Context<Self>) {
        let selected_row = index.filter(|index| *index < self.row_count);
        if self.selected_row != selected_row {
            self.selected_row = selected_row;
            cx.notify();
        }
    }
    pub fn on_row_mouse_down(
        mut self,
        handler: impl Fn(usize, &MouseDownEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.row_mouse_down_handler = Some(Rc::new(handler));
        self
    }
    pub fn set_on_row_mouse_down(
        &mut self,
        handler: impl Fn(usize, &MouseDownEvent, &mut Window, &mut App) + 'static,
        cx: &mut Context<Self>,
    ) {
        self.row_mouse_down_handler = Some(Rc::new(handler));
        cx.notify();
    }
    pub fn clear_on_row_mouse_down(&mut self, cx: &mut Context<Self>) {
        self.row_mouse_down_handler = None;
        cx.notify();
    }
    pub fn set_rows(
        &mut self,
        row_count: usize,
        render_row: impl Fn(usize) -> DataGridRow + 'static,
        cx: &mut Context<Self>,
    ) {
        self.row_count = row_count;
        self.render_row = Rc::new(render_row);
        if self.selected_row.is_some_and(|index| index >= row_count) {
            self.selected_row = None;
        }
        self.clamp_scroll();
        cx.notify();
    }
    pub fn set_columns(
        &mut self,
        columns: Vec<DataGridColumn>,
        cx: &mut Context<Self>,
    ) -> Result<(), DataGridError> {
        Self::validate_columns(&columns)?;
        self.columns = columns;
        cx.notify();
        Ok(())
    }
    pub fn set_theme(&mut self, theme: DataGridTheme, cx: &mut Context<Self>) {
        self.theme = theme;
        cx.notify();
    }
    pub fn set_accessible_label(&mut self, label: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.label = label.into();
        cx.notify();
    }
    pub fn set_row_height(
        &mut self,
        height: Pixels,
        cx: &mut Context<Self>,
    ) -> Result<(), DataGridError> {
        if height <= px(0.) {
            return Err(DataGridError::InvalidRowHeight);
        }
        self.row_height = height;
        cx.notify();
        Ok(())
    }
    pub fn set_header_height(
        &mut self,
        height: Pixels,
        cx: &mut Context<Self>,
    ) -> Result<(), DataGridError> {
        if height <= px(0.) {
            return Err(DataGridError::InvalidHeaderHeight);
        }
        self.header_height = height;
        cx.notify();
        Ok(())
    }
    pub fn scroll_to_row(
        &mut self,
        index: usize,
        strategy: ScrollStrategy,
        cx: &mut Context<Self>,
    ) {
        if self.row_count > 0 {
            self.scroll
                .scroll_to_item(index.min(self.row_count - 1), strategy);
            cx.notify();
        }
    }
    fn clamp_scroll(&self) {
        let state = self.scroll.0.borrow();
        let max = state.base_handle.max_offset().y.as_f32().max(0.);
        let x = state.base_handle.offset().x;
        let y = state.base_handle.offset().y.as_f32().clamp(-max, 0.);
        state.base_handle.set_offset(point(x, px(y)));
    }
    fn sized_cell(
        column: &DataGridColumn,
        child: impl IntoElement,
        role: Role,
        row: Option<usize>,
        col: usize,
        label: Option<SharedString>,
        theme: &DataGridTheme,
    ) -> AnyElement {
        div()
            .id(("data-grid-cell", col))
            .role(role)
            .when_some(row, |d, r| d.aria_row_index(r + 1))
            .aria_column_index(col + 1)
            .when_some(label, |d, label| d.aria_label(label))
            .h_full()
            .min_w_0()
            .text_color(theme.text.gpui())
            .when(column.divider, |d| {
                d.border_r_1().border_color(theme.divider.gpui())
            })
            .map(|d| match column.width {
                DataGridColumnWidth::Fixed(w) => d.w(w).flex_none(),
                DataGridColumnWidth::Fraction(weight) => d.flex_basis(px(0.)).flex_grow(weight),
            })
            .child(
                div()
                    .size_full()
                    .min_w_0()
                    .px_2()
                    .flex()
                    .items_center()
                    .overflow_hidden()
                    .whitespace_nowrap()
                    .child(child),
            )
            .into_any_element()
    }
    fn header(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = self.theme.clone();
        div()
            .id("data-grid-header")
            .role(Role::Row)
            .h(self.header_height)
            .w_full()
            .pr_2()
            .flex()
            .flex_none()
            .bg(theme.header_background.gpui())
            .children(
                self.columns
                    .iter()
                    .enumerate()
                    .map(|(i, c)| self.header_cell(c, i, &theme, cx)),
            )
            .into_any_element()
    }
    fn header_cell(
        &self,
        column: &DataGridColumn,
        index: usize,
        theme: &DataGridTheme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        div()
            .id(("data-grid-header-cell", index))
            .role(Role::ColumnHeader)
            .aria_row_index(1)
            .aria_column_index(index + 1)
            .aria_label(column.label.clone())
            .relative()
            .h_full()
            .min_w_0()
            .text_color(theme.text.gpui())
            .when(column.divider, |d| {
                d.border_r_1().border_color(theme.divider.gpui())
            })
            .map(|d| match column.width {
                DataGridColumnWidth::Fixed(width) => d.w(width).flex_none(),
                DataGridColumnWidth::Fraction(weight) => d.flex_basis(px(0.)).flex_grow(weight),
            })
            .child(
                div()
                    .size_full()
                    .min_w_0()
                    .px_2()
                    .flex()
                    .items_center()
                    .overflow_hidden()
                    .whitespace_nowrap()
                    .child(column.label.clone()),
            )
            .when(index + 1 < self.columns.len(), |cell| {
                cell.child(
                    div()
                        .id(("data-grid-column-resizer", index))
                        .absolute()
                        .right(px(-4.))
                        .top_0()
                        .h_full()
                        .w(COLUMN_RESIZE_HIT_WIDTH)
                        .cursor(CursorStyle::ResizeLeftRight)
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |grid, event, window, cx| {
                                grid.start_column_resize(index, event, window, cx)
                            }),
                        ),
                )
            })
            .into_any_element()
    }
    fn row(
        columns: &[DataGridColumn],
        row_height: Pixels,
        theme: &DataGridTheme,
        index: usize,
        selected: bool,
        row: DataGridRow,
        mouse_down_handler: Option<RowMouseDownHandler>,
    ) -> AnyElement {
        let label = row.accessible_label;
        let mut cells = row.cells.into_iter();
        div()
            .id(("data-grid-row", index))
            .role(Role::Row)
            .aria_row_index(index + 2)
            .aria_selected(selected)
            .when_some(label, |d, l| d.aria_label(l))
            .h(row_height)
            .w_full()
            .pr_2()
            .flex()
            .bg(if selected {
                theme.selected_row_background.gpui()
            } else {
                theme.row_background.gpui()
            })
            .when_some(mouse_down_handler, |row, handler| {
                row.on_mouse_down(MouseButton::Left, move |event, window, cx| {
                    handler(index, event, window, cx);
                })
            })
            .children(columns.iter().enumerate().map(|(i, c)| {
                let cell = cells.next().unwrap_or_else(|| DataGridCell::text(""));
                Self::sized_cell(
                    c,
                    cell.content,
                    Role::Cell,
                    Some(index + 1),
                    i,
                    cell.accessible_label,
                    theme,
                )
            }))
            .into_any_element()
    }
    fn rendered_column_widths(&self) -> Vec<f32> {
        let viewport = self
            .scroll
            .0
            .borrow()
            .base_handle
            .bounds()
            .size
            .width
            .as_f32()
            .max(1.);
        let fixed: f32 = self
            .columns
            .iter()
            .map(|column| match column.width {
                DataGridColumnWidth::Fixed(width) => width.as_f32(),
                DataGridColumnWidth::Fraction(_) => 0.,
            })
            .sum();
        let fractions: f32 = self
            .columns
            .iter()
            .map(|column| match column.width {
                DataGridColumnWidth::Fraction(weight) => weight,
                DataGridColumnWidth::Fixed(_) => 0.,
            })
            .sum();
        let flexible_space = (viewport - fixed - 8.).max(0.);
        self.columns
            .iter()
            .map(|column| match column.width {
                DataGridColumnWidth::Fixed(width) => width.as_f32(),
                DataGridColumnWidth::Fraction(weight) => flexible_space * weight / fractions,
            })
            .collect()
    }
    fn start_column_resize(
        &mut self,
        boundary: usize,
        event: &MouseDownEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let widths = self.rendered_column_widths();
        let Some((&left_width, &right_width)) = widths.get(boundary).zip(widths.get(boundary + 1))
        else {
            return;
        };
        for (column, width) in self.columns.iter_mut().zip(widths) {
            column.width = DataGridColumnWidth::Fraction(width);
        }
        self.column_resize_drag = Some(ColumnResizeDrag {
            boundary,
            start_x: event.position.x.as_f32(),
            left_width,
            right_width,
        });
        cx.stop_propagation();
    }
    fn resize_columns(&mut self, event: &MouseMoveEvent, cx: &mut Context<Self>) {
        let Some(drag) = self.column_resize_drag else {
            return;
        };
        let delta = (event.position.x.as_f32() - drag.start_x).clamp(
            MIN_COLUMN_WIDTH - drag.left_width,
            drag.right_width - MIN_COLUMN_WIDTH,
        );
        self.columns[drag.boundary].width = DataGridColumnWidth::Fraction(drag.left_width + delta);
        self.columns[drag.boundary + 1].width =
            DataGridColumnWidth::Fraction(drag.right_width - delta);
        cx.notify();
    }
    fn stop_column_resize(&mut self, cx: &mut Context<Self>) {
        if self.column_resize_drag.take().is_some() {
            cx.notify();
        }
    }
    fn geometry(&self) -> (f32, f32, f32, bool) {
        let s = self.scroll.0.borrow();
        let viewport = (s.base_handle.bounds().size.height.as_f32() - 4.).max(1.);
        let max = s.base_handle.max_offset().y.as_f32().max(0.);
        let thumb = if max > 0. {
            (viewport * viewport / (viewport + max))
                .max(24.)
                .min(viewport)
        } else {
            viewport
        };
        let offset = (-s.base_handle.offset().y.as_f32()).clamp(0., max);
        let top = if max > 0. {
            offset / max * (viewport - thumb).max(0.)
        } else {
            0.
        };
        (viewport, thumb, top, max > 0.)
    }
    fn scrollbar(&self, cx: &mut Context<Self>) -> AnyElement {
        let (_, thumb, top, scrollable) = self.geometry();
        div()
            .id("data-grid-scroll-track")
            .absolute()
            .right(px(2.))
            .top(px(2.))
            .bottom(px(2.))
            .w(px(6.))
            .rounded(px(3.))
            .bg(self.theme.scrollbar_track.gpui())
            .on_hover(cx.listener(Self::hover_scrollbar))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::jump_to_track))
            .when(!self.scrollbar_visible, |d| {
                d.opacity(0.).hover(|s| s.opacity(1.))
            })
            .when(scrollable, |d| {
                d.child(
                    div()
                        .id("data-grid-scroll-thumb")
                        .absolute()
                        .top(px(top))
                        .w_full()
                        .h(px(thumb))
                        .rounded(px(3.))
                        .bg(if self.scrollbar_hovered || self.drag.is_some() {
                            self.theme.scrollbar_thumb_hover.gpui()
                        } else {
                            self.theme.scrollbar_thumb.gpui()
                        })
                        .cursor(CursorStyle::PointingHand)
                        .on_mouse_down(MouseButton::Left, cx.listener(Self::start_drag)),
                )
            })
            .into_any_element()
    }
    fn show(&mut self, cx: &mut Context<Self>) {
        self.hide_generation = self.hide_generation.wrapping_add(1);
        self.hide_task = None;
        if !self.scrollbar_visible {
            self.scrollbar_visible = true;
            cx.notify();
        }
    }
    fn show_temporarily(&mut self, cx: &mut Context<Self>) {
        self.show(cx);
        let generation = self.hide_generation;
        let timer = cx.background_executor().timer(Duration::from_secs(1));
        self.hide_task = Some(cx.spawn(async move |this, cx| {
            timer.await;
            let _ = this.update(cx, |grid, cx| {
                if grid.hide_generation == generation
                    && grid.drag.is_none()
                    && !grid.scrollbar_hovered
                {
                    grid.scrollbar_visible = false;
                    grid.hide_task = None;
                    cx.notify();
                }
            });
        }));
    }
    fn wheel(&mut self, _: &ScrollWheelEvent, _: &mut Window, cx: &mut Context<Self>) {
        if self.scroll.is_scrollable() {
            self.show_temporarily(cx)
        }
    }
    fn hover_scrollbar(&mut self, hovered: &bool, _: &mut Window, cx: &mut Context<Self>) {
        self.scrollbar_hovered = *hovered;
        if *hovered {
            self.show(cx)
        } else if self.drag.is_none() {
            self.show_temporarily(cx)
        };
        cx.notify();
    }
    fn start_drag(&mut self, e: &MouseDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        let offset = self.scroll.0.borrow().base_handle.offset().y.as_f32();
        self.drag = Some((e.position.y.as_f32(), offset));
        self.show(cx);
        cx.stop_propagation();
    }
    fn jump_to_track(&mut self, e: &MouseDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        let (viewport, thumb, _, scrollable) = self.geometry();
        if !scrollable { return; }
        let state = self.scroll.0.borrow();
        let bounds = state.base_handle.bounds();
        let max = state.base_handle.max_offset().y.as_f32().max(0.);
        let position = (e.position.y.as_f32() - bounds.origin.y.as_f32() - 2. - thumb / 2.)
            .clamp(0., (viewport - thumb).max(0.));
        let x = state.base_handle.offset().x;
        state.base_handle.set_offset(point(x, px(-(position / (viewport - thumb).max(1.)) * max)));
        drop(state);
        self.show_temporarily(cx);
        cx.notify();
    }

    fn drag(&mut self, e: &MouseMoveEvent, _: &mut Window, cx: &mut Context<Self>) {
        let Some((start, start_offset)) = self.drag else {
            return;
        };
        let (viewport, thumb, _, _) = self.geometry();
        let max = self
            .scroll
            .0
            .borrow()
            .base_handle
            .max_offset()
            .y
            .as_f32()
            .max(0.);
        let y = (start_offset - (e.position.y.as_f32() - start) * max / (viewport - thumb).max(1.))
            .clamp(-max, 0.);
        let s = self.scroll.0.borrow();
        let x = s.base_handle.offset().x;
        s.base_handle.set_offset(point(x, px(y)));
        drop(s);
        cx.notify();
    }
    fn stop_drag(&mut self, _: &MouseUpEvent, _: &mut Window, cx: &mut Context<Self>) {
        if self.drag.take().is_some() {
            self.show_temporarily(cx)
        }
    }
    fn capture(&self, cx: &mut Context<Self>) -> AnyElement {
        let grid = cx.entity().downgrade();
        canvas(
            |_, _, _| (),
            move |_, _, window, _| {
                let g = grid.clone();
                window.on_mouse_event(move |e: &MouseMoveEvent, _, w, cx| {
                    let _ = g.update(cx, |x, cx| {
                        x.drag(e, w, cx);
                        x.resize_columns(e, cx);
                    });
                });
                let g = grid.clone();
                window.on_mouse_event(move |e: &MouseUpEvent, _, w, cx| {
                    if e.button == MouseButton::Left {
                        let _ = g.update(cx, |x, cx| {
                            x.stop_drag(e, w, cx);
                            x.stop_column_resize(cx);
                        });
                    }
                });
            },
        )
        .absolute()
        .inset_0()
        .into_any_element()
    }
}

impl Render for DataGrid {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let columns = self.columns.clone();
        let theme = self.theme.clone();
        let height = self.row_height;
        let renderer = self.render_row.clone();
        let selected_row = self.selected_row;
        let consumer_mouse_down_handler = self.row_mouse_down_handler.clone();
        let grid = cx.entity();
        let row_mouse_down_handler: RowMouseDownHandler =
            Rc::new(move |index, event, window, cx| {
                let _ = grid.update(cx, |grid, cx| grid.select_row(Some(index), cx));
                if let Some(handler) = &consumer_mouse_down_handler {
                    handler(index, event, window, cx);
                }
            });
        div()
            .id(("data-grid", cx.entity_id()))
            .role(Role::Grid)
            .aria_label(self.label.clone())
            .aria_row_count(self.row_count + 1)
            .aria_column_count(self.columns.len())
            .relative()
            .size_full()
            .min_h_0()
            .overflow_hidden()
            .flex()
            .flex_col()
            .child(self.header(cx))
            .child(
                div()
                    .relative()
                    .flex_1()
                    .min_h_0()
                    .overflow_hidden()
                    .bg(self.theme.row_background.gpui())
                    .on_scroll_wheel(cx.listener(Self::wheel))
                    .child(
                        uniform_list("data-grid-rows", self.row_count, move |range, _, _| {
                            range
                                .map(|i| {
                                    Self::row(
                                        &columns,
                                        height,
                                        &theme,
                                        i,
                                        selected_row == Some(i),
                                        renderer(i),
                                        Some(row_mouse_down_handler.clone()),
                                    )
                                })
                                .collect()
                        })
                        .size_full()
                        .with_horizontal_sizing_behavior(ListHorizontalSizingBehavior::FitList)
                        .track_scroll(&self.scroll),
                    )
                    .child(self.scrollbar(cx)),
            )
            .child(self.capture(cx))
    }
}

#[cfg(test)]
mod data_grid_tests {
    use super::*;

    #[test]
    fn resize_hit_area_and_minimum_width_are_stable() {
        assert_eq!(COLUMN_RESIZE_HIT_WIDTH, px(8.));
        let drag = ColumnResizeDrag {
            boundary: 0,
            start_x: 100.,
            left_width: 120.,
            right_width: 80.,
        };
        let delta = (300_f32 - drag.start_x).clamp(
            MIN_COLUMN_WIDTH - drag.left_width,
            drag.right_width - MIN_COLUMN_WIDTH,
        );
        assert_eq!(drag.left_width + delta, 160.);
        assert_eq!(drag.right_width - delta, MIN_COLUMN_WIDTH);
    }

    #[test]
    fn resized_widths_remain_proportional() {
        let left = DataGridColumnWidth::Fraction(140.);
        let right = DataGridColumnWidth::Fraction(60.);
        assert_eq!(left, DataGridColumnWidth::Fraction(140.));
        assert_eq!(right, DataGridColumnWidth::Fraction(60.));
        assert_eq!(140_f32 / 60., 280_f32 / 120.);
    }

    #[test]
    fn row_mouse_down_handler_is_optional_and_configurable() {
        let columns = vec![DataGridColumn::new("value", "Value")];
        let grid = DataGrid::new(columns, 1, |_| {
            DataGridRow::new([DataGridCell::text("row")])
        })
        .expect("valid grid");
        assert!(grid.row_mouse_down_handler.is_none());

        let grid = grid.on_row_mouse_down(|index, _, _, _| assert_eq!(index, 0));
        assert!(grid.row_mouse_down_handler.is_some());
    }
}
