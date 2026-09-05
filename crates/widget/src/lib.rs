//! Reusable GPUI widgets for desktop applications.

mod tab_bar;
mod text_editor;
mod text_input;
mod theme;

pub use tab_bar::{NF, TabBar, TabBarError, TabBarEvent, TabId, TabSpec};
pub use theme::{TabBarTheme, TextEditorTheme, TextFieldTheme, Theme, ThemeColor, ThemeLoadError};

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
