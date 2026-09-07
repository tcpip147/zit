//! Reusable GPUI widgets for desktop applications.
//!
//! [`DataGrid`] provides an asset-free, virtualized fixed-header table. Consumers supply
//! columns and an indexed row callback, so the widget does not need to own domain data.

#[path = "data-grid.rs"]
mod data_grid;
mod diff_editor;
mod tab_bar;
mod text_editor;
mod text_input;
mod scroll_container;
mod theme;

pub use data_grid::{
    DataGrid, DataGridCell, DataGridColumn, DataGridColumnWidth, DataGridError, DataGridRow,
};
pub use diff_editor::DiffEditor;
pub use tab_bar::{NF, TabBar, TabBarError, TabBarEvent, TabId, TabSpec};
pub use theme::{
    DataGridTheme, TabBarTheme, TextEditorTheme, TextFieldTheme, Theme, ThemeColor, ThemeLoadError,
};

pub use text_editor::{
    DocumentEnd as TextEditorDocumentEnd, DocumentStart as TextEditorDocumentStart,
    Down as TextEditorDown, InsertNewline as TextEditorInsertNewline,
    PageDown as TextEditorPageDown, PageUp as TextEditorPageUp,
    SelectDocumentEnd as TextEditorSelectDocumentEnd,
    SelectDocumentStart as TextEditorSelectDocumentStart, SelectDown as TextEditorSelectDown,
    SelectPageDown as TextEditorSelectPageDown, SelectPageUp as TextEditorSelectPageUp,
    SelectUp as TextEditorSelectUp, TextEditor, TextEditorChange, Up as TextEditorUp,
    init_text_editor,
};

pub use text_input::{
    Backspace, Copy, Cut, Delete, End, Left, Paste, Redo, Right, SelectAll, SelectEnd, SelectLeft,
    SelectRight, SelectStart, SelectWordLeft, SelectWordRight, Start, TextInput, TextInputChange,
    Undo, WordLeft, WordRight, init,
};
pub use scroll_container::ScrollContainer;
