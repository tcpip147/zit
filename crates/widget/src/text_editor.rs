use std::{ops::Range, sync::Once, time::Duration};

use crate::Theme;
use gpui::{
    App, Bounds, ClipboardItem, Context, CursorStyle, Element, ElementId, ElementInputHandler,
    Entity, EntityInputHandler, EventEmitter, FocusHandle, Focusable, GlobalElementId, KeyBinding,
    LayoutId, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, PaintQuad, Pixels, Point,
    ScrollHandle, ScrollWheelEvent, ShapedLine, SharedString, Style, Subscription, Task, TextRun,
    UTF16Selection, Window, actions, div, fill, point, prelude::*, px, relative,
};
use unicode_segmentation::UnicodeSegmentation;

const CARET_BLINK_INTERVAL: Duration = Duration::from_millis(500);
const VERTICAL_CHROME: Pixels = px(10.);

actions!(
    text_editor,
    [
        Backspace,
        Delete,
        Left,
        Right,
        WordLeft,
        WordRight,
        Start,
        End,
        SelectLeft,
        SelectRight,
        SelectWordLeft,
        SelectWordRight,
        SelectStart,
        SelectEnd,
        SelectAll,
        Copy,
        Cut,
        Paste,
        Undo,
        Redo,
        InsertNewline,
        Up,
        Down,
        PageUp,
        PageDown,
        DocumentStart,
        DocumentEnd,
        SelectUp,
        SelectDown,
        SelectPageUp,
        SelectPageDown,
        SelectDocumentStart,
        SelectDocumentEnd,
    ]
);

/// Registers TextEditor's default desktop key bindings once per process.
pub fn init_text_editor(cx: &mut App) {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        cx.bind_keys([
            KeyBinding::new("backspace", Backspace, Some("TextEditor")),
            KeyBinding::new("delete", Delete, Some("TextEditor")),
            KeyBinding::new("left", Left, Some("TextEditor")),
            KeyBinding::new("right", Right, Some("TextEditor")),
            KeyBinding::new("alt-left", WordLeft, Some("TextEditor")),
            KeyBinding::new("alt-right", WordRight, Some("TextEditor")),
            KeyBinding::new("ctrl-left", WordLeft, Some("TextEditor")),
            KeyBinding::new("ctrl-right", WordRight, Some("TextEditor")),
            KeyBinding::new("home", Start, Some("TextEditor")),
            KeyBinding::new("end", End, Some("TextEditor")),
            KeyBinding::new("secondary-left", Start, Some("TextEditor")),
            KeyBinding::new("secondary-right", End, Some("TextEditor")),
            KeyBinding::new("shift-left", SelectLeft, Some("TextEditor")),
            KeyBinding::new("shift-right", SelectRight, Some("TextEditor")),
            KeyBinding::new("shift-alt-left", SelectWordLeft, Some("TextEditor")),
            KeyBinding::new("shift-alt-right", SelectWordRight, Some("TextEditor")),
            KeyBinding::new("shift-ctrl-left", SelectWordLeft, Some("TextEditor")),
            KeyBinding::new("shift-ctrl-right", SelectWordRight, Some("TextEditor")),
            KeyBinding::new("shift-home", SelectStart, Some("TextEditor")),
            KeyBinding::new("shift-end", SelectEnd, Some("TextEditor")),
            KeyBinding::new("secondary-a", SelectAll, Some("TextEditor")),
            KeyBinding::new("secondary-c", Copy, Some("TextEditor")),
            KeyBinding::new("secondary-x", Cut, Some("TextEditor")),
            KeyBinding::new("secondary-v", Paste, Some("TextEditor")),
            KeyBinding::new("secondary-z", Undo, Some("TextEditor")),
            KeyBinding::new("secondary-shift-z", Redo, Some("TextEditor")),
            KeyBinding::new("ctrl-y", Redo, Some("TextEditor")),
            KeyBinding::new("enter", InsertNewline, Some("TextEditor")),
            KeyBinding::new("up", Up, Some("TextEditor")),
            KeyBinding::new("down", Down, Some("TextEditor")),
            KeyBinding::new("pageup", PageUp, Some("TextEditor")),
            KeyBinding::new("pagedown", PageDown, Some("TextEditor")),
            KeyBinding::new("secondary-up", DocumentStart, Some("TextEditor")),
            KeyBinding::new("secondary-down", DocumentEnd, Some("TextEditor")),
            KeyBinding::new("shift-up", SelectUp, Some("TextEditor")),
            KeyBinding::new("shift-down", SelectDown, Some("TextEditor")),
            KeyBinding::new("shift-pageup", SelectPageUp, Some("TextEditor")),
            KeyBinding::new("shift-pagedown", SelectPageDown, Some("TextEditor")),
            KeyBinding::new(
                "shift-secondary-up",
                SelectDocumentStart,
                Some("TextEditor"),
            ),
            KeyBinding::new(
                "shift-secondary-down",
                SelectDocumentEnd,
                Some("TextEditor"),
            ),
        ]);
    });
}

/// Emitted after a user edit, undo, or redo changes the committed value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextEditorChange {
    pub value: SharedString,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Snapshot {
    text: SharedString,
    selection: Range<usize>,
    reversed: bool,
}

#[derive(Clone, Debug)]
struct Composition {
    original: Snapshot,
    marked_range: Range<usize>,
}

/// A reusable GPUI multi-line plain-text editor.
///
/// While focused and enabled with an empty selection, the caret alternates between visible and
/// hidden every 500 ms. Editing, navigation, selection, pointer, clipboard, history, IME, focus,
/// and enabled-state transitions reset it to a visible phase when it remains eligible. Blurring,
/// disabling, or creating a non-empty selection stops the cycle. Blink-only visual transitions do
/// not emit [`TextEditorChange`].
///
/// A pointer press that focuses an enabled input makes the caret visible in the same interaction
/// update as caret placement, before the first blink interval. The rendered control also keeps a
/// minimum height of one current text line plus its vertical padding and borders, so smaller host
/// constraints cannot vertically clip the text, selection, composition, or caret.
pub struct TextEditor {
    focus_handle: FocusHandle,
    content: SharedString,
    committed: SharedString,
    placeholder: SharedString,
    enabled: bool,
    read_only: bool,
    wrap: bool,
    scroll_handle: ScrollHandle,
    line_height: Pixels,
    scrollbar_drag: Option<(Pixels, Pixels)>,
    scrollbar_drag_scale: f32,
    scrollbar_max_offset: Pixels,
    selected_range: Range<usize>,
    selection_reversed: bool,
    composition: Option<Composition>,
    undo_stack: Vec<Snapshot>,
    redo_stack: Vec<Snapshot>,
    last_layouts: Vec<ShapedLine>,
    last_bounds: Option<Bounds<Pixels>>,
    horizontal_offset: Pixels,
    is_selecting: bool,
    has_focus: bool,
    caret_visible: bool,
    blink_generation: u64,
    blink_task: Task<()>,
    focus_subscriptions: Option<Vec<Subscription>>,
    theme: Theme,
}

impl TextEditor {
    /// Creates an empty, enabled input.
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self::with_value("", cx)
    }

    /// Creates an enabled input with a normalized single-line value.
    pub fn with_value(value: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        let value = normalize_multi_line(&value.into());
        let cursor = value.len();
        Self {
            focus_handle: cx.focus_handle(),
            content: value.clone().into(),
            committed: value.into(),
            placeholder: SharedString::default(),
            enabled: true,
            read_only: false,
            wrap: true,
            scroll_handle: ScrollHandle::new(),
            line_height: px(0.),
            scrollbar_drag: None,
            scrollbar_drag_scale: 0.,
            scrollbar_max_offset: px(0.),
            selected_range: cursor..cursor,
            selection_reversed: false,
            composition: None,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            last_layouts: Vec::new(),
            last_bounds: None,
            horizontal_offset: px(0.),
            is_selecting: false,
            has_focus: false,
            caret_visible: false,
            blink_generation: 0,
            blink_task: Task::ready(()),
            focus_subscriptions: None,
            theme: Theme::default(),
        }
    }

    pub fn set_theme(&mut self, theme: Theme, cx: &mut Context<Self>) {
        self.theme = theme;
        cx.notify();
    }
    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    /// Returns the committed value. Provisional IME text is excluded.
    pub fn value(&self) -> &str {
        &self.committed
    }

    /// Replaces the value programmatically without emitting a user-change event.
    pub fn set_value(&mut self, value: impl Into<SharedString>, cx: &mut Context<Self>) {
        let value = normalize_multi_line(&value.into());
        let cursor = value.len();
        self.content = value.clone().into();
        self.committed = value.into();
        self.selected_range = cursor..cursor;
        self.selection_reversed = false;
        self.composition = None;
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.reset_blink(cx);
        cx.notify();
    }

    /// Sets non-value hint text displayed only while the value is empty.
    pub fn set_placeholder(
        &mut self,
        placeholder: impl Into<SharedString>,
        cx: &mut Context<Self>,
    ) {
        self.placeholder = placeholder.into();
        cx.notify();
    }

    /// Returns the configured placeholder.
    pub fn placeholder(&self) -> &str {
        &self.placeholder
    }

    /// Enables or disables user interaction, blurring when disabled.
    pub fn set_enabled(&mut self, enabled: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.enabled == enabled {
            return;
        }
        self.enabled = enabled;
        self.is_selecting = false;
        if !enabled {
            self.cancel_composition();
            self.stop_blink(cx);
            if self.focus_handle.is_focused(window) {
                window.blur(cx);
            }
        } else {
            self.reset_blink(cx);
        }
        cx.notify();
    }

    /// Returns whether user interaction is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Makes the editor selectable and scrollable without allowing mutations.
    pub fn set_read_only(&mut self, read_only: bool, cx: &mut Context<Self>) {
        if self.read_only != read_only {
            self.read_only = read_only;
            if read_only {
                self.cancel_composition();
            }
            self.reset_blink(cx);
            cx.notify();
        }
    }

    pub fn is_read_only(&self) -> bool {
        self.read_only
    }

    /// Enables or disables soft wrapping. Multi-line layout consumes this flag.
    pub fn set_wrap(&mut self, wrap: bool, cx: &mut Context<Self>) {
        if self.wrap != wrap {
            self.wrap = wrap;
            if wrap {
                self.horizontal_offset = px(0.);
            }
            cx.notify();
        }
    }

    pub fn is_wrapping(&self) -> bool {
        self.wrap
    }

    /// Returns the current internal scroll offset. Vertical offsets are zero
    /// at the top and negative while scrolled toward the end of the document.
    pub fn scroll_offset(&self) -> Point<Pixels> {
        self.scroll_handle.offset()
    }

    /// Returns the normalized UTF-8 byte range of the current selection or caret.
    pub fn selection(&self) -> Range<usize> {
        self.selected_range.clone()
    }

    /// Returns whether the caret is currently in its visible blink phase.
    ///
    /// Blinking is eligible only while the input is focused, enabled, and has
    /// an empty selection. Blink-only transitions never emit a value change.
    pub fn is_caret_visible(&self) -> bool {
        self.caret_visible && self.blink_eligible()
    }

    fn blink_eligible(&self) -> bool {
        self.has_focus && self.enabled && self.selected_range.is_empty()
    }

    fn start_blink(&mut self, cx: &mut Context<Self>) {
        self.has_focus = true;
        self.reset_blink(cx);
    }

    fn stop_blink(&mut self, cx: &mut Context<Self>) {
        self.has_focus = false;
        self.blink_generation = self.blink_generation.wrapping_add(1);
        self.caret_visible = false;
        self.blink_task = Task::ready(());
        cx.notify();
    }

    fn reset_blink(&mut self, cx: &mut Context<Self>) {
        self.blink_generation = self.blink_generation.wrapping_add(1);
        let generation = self.blink_generation;
        self.caret_visible = self.blink_eligible();
        self.blink_task = Task::ready(());

        if !self.blink_eligible() {
            cx.notify();
            return;
        }

        self.blink_task = cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(CARET_BLINK_INTERVAL).await;
                let keep_blinking = this
                    .update(cx, |input, cx| {
                        if input.blink_generation != generation || !input.blink_eligible() {
                            return false;
                        }
                        input.caret_visible = !input.caret_visible;
                        cx.notify();
                        true
                    })
                    .unwrap_or(false);
                if !keep_blinking {
                    break;
                }
            }
        });
        cx.notify();
    }

    fn snapshot(&self) -> Snapshot {
        Snapshot {
            text: self.committed.clone(),
            selection: self.selected_range.clone(),
            reversed: self.selection_reversed,
        }
    }

    fn restore(&mut self, snapshot: Snapshot) {
        self.content = snapshot.text.clone();
        self.committed = snapshot.text;
        self.selected_range = snapshot.selection;
        self.selection_reversed = snapshot.reversed;
        self.composition = None;
    }

    fn emit_change(&self, cx: &mut Context<Self>) {
        cx.emit(TextEditorChange {
            value: self.committed.clone(),
        });
    }

    fn commit_edit(&mut self, before: Snapshot, cx: &mut Context<Self>) {
        if before.text == self.content {
            return;
        }
        self.undo_stack.push(before);
        self.redo_stack.clear();
        self.committed = self.content.clone();
        self.reveal_cursor();
        self.emit_change(cx);
        self.reset_blink(cx);
    }

    fn replace_range(&mut self, range: Range<usize>, text: &str) {
        let range = self.clamp_range(range);
        let text = normalize_multi_line(text);
        let mut result = String::with_capacity(self.content.len() + text.len());
        result.push_str(&self.content[..range.start]);
        result.push_str(&text);
        result.push_str(&self.content[range.end..]);
        let cursor = range.start + text.len();
        self.content = result.into();
        self.selected_range = cursor..cursor;
        self.selection_reversed = false;
    }

    fn replace_selection_committed(&mut self, text: &str, cx: &mut Context<Self>) {
        if !self.enabled || self.read_only {
            return;
        }
        let before = self.snapshot();
        let range = self
            .composition
            .take()
            .map(|composition| composition.marked_range)
            .unwrap_or_else(|| self.selected_range.clone());
        self.replace_range(range, text);
        self.commit_edit(before, cx);
        cx.notify();
    }

    fn cursor_offset(&self) -> usize {
        if self.selection_reversed {
            self.selected_range.start
        } else {
            self.selected_range.end
        }
    }

    fn move_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        let offset = self.clamp_offset(offset);
        self.selected_range = offset..offset;
        self.selection_reversed = false;
        self.reveal_cursor();
        self.reset_blink(cx);
        cx.notify();
    }

    fn select_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        let offset = self.clamp_offset(offset);
        if self.selection_reversed {
            self.selected_range.start = offset;
        } else {
            self.selected_range.end = offset;
        }
        if self.selected_range.end < self.selected_range.start {
            self.selection_reversed = !self.selection_reversed;
            self.selected_range = self.selected_range.end..self.selected_range.start;
        }
        self.reveal_cursor();
        self.reset_blink(cx);
        cx.notify();
    }

    fn clamp_offset(&self, offset: usize) -> usize {
        let offset = offset.min(self.content.len());
        if self.content.is_char_boundary(offset) {
            offset
        } else {
            (0..offset)
                .rev()
                .find(|candidate| self.content.is_char_boundary(*candidate))
                .unwrap_or(0)
        }
    }

    fn clamp_range(&self, range: Range<usize>) -> Range<usize> {
        let start = self.clamp_offset(range.start);
        let end = self.clamp_offset(range.end.max(start));
        start..end
    }

    fn reveal_cursor(&mut self) {
        let line_height = self.line_height;
        if line_height <= px(0.) {
            return;
        }
        let measured_height = self.scroll_handle.bounds().size.height;
        if measured_height <= px(0.) {
            return;
        }
        let viewport_height = measured_height;
        let row = self.content[..self.cursor_offset()]
            .bytes()
            .filter(|byte| *byte == b'\n')
            .count();
        let caret_top = line_height * row;
        let caret_bottom = caret_top + line_height;
        let current = self.scroll_handle.offset();
        let visible_top = -current.y;
        let mut target_y = current.y;
        if caret_top < visible_top {
            target_y = -caret_top;
        } else if caret_bottom > visible_top + viewport_height {
            target_y = -(caret_bottom - viewport_height);
        }
        let estimated_max =
            (line_height * (self.content.matches('\n').count() + 1) - viewport_height).max(px(0.));
        let max_offset = self.scroll_handle.max_offset().y.max(estimated_max);
        self.scroll_handle
            .set_offset(point(current.x, target_y.clamp(-max_offset, px(0.))));
    }

    fn offset_from_utf16(&self, offset: usize) -> usize {
        let mut utf8 = 0;
        let mut utf16 = 0;
        for ch in self.content.chars() {
            if utf16 >= offset {
                break;
            }
            let next = utf16 + ch.len_utf16();
            if next > offset {
                break;
            }
            utf16 = next;
            utf8 += ch.len_utf8();
        }
        utf8
    }

    fn offset_to_utf16(&self, offset: usize) -> usize {
        self.content[..self.clamp_offset(offset)]
            .chars()
            .map(char::len_utf16)
            .sum()
    }

    fn range_from_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_from_utf16(range.start)..self.offset_from_utf16(range.end)
    }

    fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
    }

    fn previous_boundary(&self, offset: usize) -> usize {
        self.content
            .grapheme_indices(true)
            .rev()
            .find_map(|(index, _)| (index < offset).then_some(index))
            .unwrap_or(0)
    }

    fn next_boundary(&self, offset: usize) -> usize {
        self.content
            .grapheme_indices(true)
            .find_map(|(index, _)| (index > offset).then_some(index))
            .unwrap_or(self.content.len())
    }

    fn previous_word_boundary(&self, offset: usize) -> usize {
        self.content[..self.clamp_offset(offset)]
            .split_word_bound_indices()
            .filter_map(|(index, segment)| (!segment.trim().is_empty()).then_some(index))
            .next_back()
            .unwrap_or(0)
    }

    fn next_word_boundary(&self, offset: usize) -> usize {
        let offset = self.clamp_offset(offset);
        let tail = &self.content[offset..];
        let mut saw_word = false;
        for (index, segment) in tail.split_word_bound_indices() {
            if segment.trim().is_empty() {
                if saw_word {
                    return offset + index;
                }
            } else {
                saw_word = true;
            }
        }
        self.content.len()
    }

    fn line_bounds(&self, offset: usize) -> Range<usize> {
        let offset = self.clamp_offset(offset);
        let start = self.content[..offset].rfind('\n').map_or(0, |ix| ix + 1);
        let end = self.content[offset..]
            .find('\n')
            .map_or(self.content.len(), |ix| offset + ix);
        start..end
    }

    fn vertical_offset(&self, offset: usize, delta: isize) -> usize {
        let current = self.line_bounds(offset);
        let column = offset.saturating_sub(current.start);
        let target = if delta < 0 {
            if current.start == 0 {
                return 0;
            }
            self.line_bounds(current.start - 1)
        } else {
            if current.end == self.content.len() {
                return self.content.len();
            }
            self.line_bounds(current.end + 1)
        };
        self.clamp_offset((target.start + column).min(target.end))
    }

    fn page_offset(&self, mut offset: usize, delta: isize) -> usize {
        for _ in 0..10 {
            let next = self.vertical_offset(offset, delta);
            if next == offset {
                break;
            }
            offset = next;
        }
        offset
    }

    fn delete_range(&mut self, range: Range<usize>, cx: &mut Context<Self>) {
        let before = self.snapshot();
        self.replace_range(range, "");
        self.commit_edit(before, cx);
        cx.notify();
    }

    fn on_backspace(&mut self, _: &Backspace, _: &mut Window, cx: &mut Context<Self>) {
        if !self.enabled || self.read_only {
            return;
        }
        let range = if self.selected_range.is_empty() {
            self.previous_boundary(self.cursor_offset())..self.cursor_offset()
        } else {
            self.selected_range.clone()
        };
        self.delete_range(range, cx);
    }

    fn on_delete(&mut self, _: &Delete, _: &mut Window, cx: &mut Context<Self>) {
        if !self.enabled || self.read_only {
            return;
        }
        let range = if self.selected_range.is_empty() {
            self.cursor_offset()..self.next_boundary(self.cursor_offset())
        } else {
            self.selected_range.clone()
        };
        self.delete_range(range, cx);
    }

    fn on_left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) {
        if !self.enabled {
            return;
        }
        if self.selected_range.is_empty() {
            self.move_to(self.previous_boundary(self.cursor_offset()), cx);
        } else {
            self.move_to(self.selected_range.start, cx);
        }
    }

    fn on_right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
        if !self.enabled {
            return;
        }
        if self.selected_range.is_empty() {
            self.move_to(self.next_boundary(self.cursor_offset()), cx);
        } else {
            self.move_to(self.selected_range.end, cx);
        }
    }

    fn on_word_left(&mut self, _: &WordLeft, _: &mut Window, cx: &mut Context<Self>) {
        if !self.enabled {
            return;
        }
        self.move_to(self.previous_word_boundary(self.cursor_offset()), cx);
    }

    fn on_word_right(&mut self, _: &WordRight, _: &mut Window, cx: &mut Context<Self>) {
        if !self.enabled {
            return;
        }
        self.move_to(self.next_word_boundary(self.cursor_offset()), cx);
    }

    fn on_start(&mut self, _: &Start, _: &mut Window, cx: &mut Context<Self>) {
        if !self.enabled {
            return;
        }
        self.move_to(self.line_bounds(self.cursor_offset()).start, cx);
    }

    fn on_end(&mut self, _: &End, _: &mut Window, cx: &mut Context<Self>) {
        if !self.enabled {
            return;
        }
        self.move_to(self.line_bounds(self.cursor_offset()).end, cx);
    }

    fn on_select_left(&mut self, _: &SelectLeft, _: &mut Window, cx: &mut Context<Self>) {
        if !self.enabled {
            return;
        }
        self.select_to(self.previous_boundary(self.cursor_offset()), cx);
    }

    fn on_select_right(&mut self, _: &SelectRight, _: &mut Window, cx: &mut Context<Self>) {
        if !self.enabled {
            return;
        }
        self.select_to(self.next_boundary(self.cursor_offset()), cx);
    }

    fn on_select_word_left(&mut self, _: &SelectWordLeft, _: &mut Window, cx: &mut Context<Self>) {
        if !self.enabled {
            return;
        }
        self.select_to(self.previous_word_boundary(self.cursor_offset()), cx);
    }

    fn on_select_word_right(
        &mut self,
        _: &SelectWordRight,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.enabled {
            return;
        }
        self.select_to(self.next_word_boundary(self.cursor_offset()), cx);
    }

    fn on_select_start(&mut self, _: &SelectStart, _: &mut Window, cx: &mut Context<Self>) {
        if !self.enabled {
            return;
        }
        self.select_to(self.line_bounds(self.cursor_offset()).start, cx);
    }

    fn on_select_end(&mut self, _: &SelectEnd, _: &mut Window, cx: &mut Context<Self>) {
        if !self.enabled {
            return;
        }
        self.select_to(self.line_bounds(self.cursor_offset()).end, cx);
    }

    fn on_select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        if !self.enabled {
            return;
        }
        self.selected_range = 0..self.content.len();
        self.selection_reversed = false;
        cx.notify();
    }

    fn on_insert_newline(&mut self, _: &InsertNewline, _: &mut Window, cx: &mut Context<Self>) {
        self.replace_selection_committed("\n", cx);
    }

    fn on_up(&mut self, _: &Up, _: &mut Window, cx: &mut Context<Self>) {
        if self.enabled {
            self.move_to(self.vertical_offset(self.cursor_offset(), -1), cx);
        }
    }

    fn on_down(&mut self, _: &Down, _: &mut Window, cx: &mut Context<Self>) {
        if self.enabled {
            self.move_to(self.vertical_offset(self.cursor_offset(), 1), cx);
        }
    }

    fn on_page_up(&mut self, _: &PageUp, _: &mut Window, cx: &mut Context<Self>) {
        if self.enabled {
            self.move_to(self.page_offset(self.cursor_offset(), -1), cx);
        }
    }

    fn on_page_down(&mut self, _: &PageDown, _: &mut Window, cx: &mut Context<Self>) {
        if self.enabled {
            self.move_to(self.page_offset(self.cursor_offset(), 1), cx);
        }
    }

    fn on_document_start(&mut self, _: &DocumentStart, _: &mut Window, cx: &mut Context<Self>) {
        if self.enabled {
            self.move_to(0, cx);
        }
    }

    fn on_document_end(&mut self, _: &DocumentEnd, _: &mut Window, cx: &mut Context<Self>) {
        if self.enabled {
            self.move_to(self.content.len(), cx);
        }
    }

    fn on_select_up(&mut self, _: &SelectUp, _: &mut Window, cx: &mut Context<Self>) {
        if self.enabled {
            self.select_to(self.vertical_offset(self.cursor_offset(), -1), cx);
        }
    }

    fn on_select_down(&mut self, _: &SelectDown, _: &mut Window, cx: &mut Context<Self>) {
        if self.enabled {
            self.select_to(self.vertical_offset(self.cursor_offset(), 1), cx);
        }
    }

    fn on_select_page_up(&mut self, _: &SelectPageUp, _: &mut Window, cx: &mut Context<Self>) {
        if self.enabled {
            self.select_to(self.page_offset(self.cursor_offset(), -1), cx);
        }
    }

    fn on_select_page_down(&mut self, _: &SelectPageDown, _: &mut Window, cx: &mut Context<Self>) {
        if self.enabled {
            self.select_to(self.page_offset(self.cursor_offset(), 1), cx);
        }
    }

    fn on_select_document_start(
        &mut self,
        _: &SelectDocumentStart,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.enabled {
            self.select_to(0, cx);
        }
    }

    fn on_select_document_end(
        &mut self,
        _: &SelectDocumentEnd,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.enabled {
            self.select_to(self.content.len(), cx);
        }
    }

    fn on_copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        if self.enabled && !self.selected_range.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content[self.selected_range.clone()].to_owned(),
            ));
        }
        self.reset_blink(cx);
    }

    fn on_cut(&mut self, _: &Cut, _: &mut Window, cx: &mut Context<Self>) {
        if !self.enabled || self.read_only || self.selected_range.is_empty() {
            return;
        }
        cx.write_to_clipboard(ClipboardItem::new_string(
            self.content[self.selected_range.clone()].to_owned(),
        ));
        self.delete_range(self.selected_range.clone(), cx);
    }

    fn on_paste(&mut self, _: &Paste, _: &mut Window, cx: &mut Context<Self>) {
        if !self.enabled || self.read_only {
            return;
        }
        if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
            self.replace_selection_committed(&text, cx);
        }
    }

    fn on_undo(&mut self, _: &Undo, _: &mut Window, cx: &mut Context<Self>) {
        if !self.enabled || self.read_only {
            return;
        }
        if let Some(previous) = self.undo_stack.pop() {
            self.redo_stack.push(self.snapshot());
            self.restore(previous);
            self.reveal_cursor();
            self.emit_change(cx);
            self.reset_blink(cx);
            cx.notify();
        }
    }

    fn on_redo(&mut self, _: &Redo, _: &mut Window, cx: &mut Context<Self>) {
        if !self.enabled || self.read_only {
            return;
        }
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(self.snapshot());
            self.restore(next);
            self.reveal_cursor();
            self.emit_change(cx);
            self.reset_blink(cx);
            cx.notify();
        }
    }

    fn index_for_mouse_position(&self, position: Point<Pixels>) -> usize {
        if self.content.is_empty() {
            return 0;
        }
        let Some(bounds) = &self.last_bounds else {
            return 0;
        };
        if self.last_layouts.is_empty() {
            return 0;
        }
        let line_height = bounds.size.height / self.last_layouts.len() as f32;
        let row =
            (((position.y - bounds.top()) / line_height) as usize).min(self.last_layouts.len() - 1);
        let line = &self.last_layouts[row];
        let start = self
            .content
            .split_inclusive('\n')
            .take(row)
            .map(str::len)
            .sum::<usize>();
        start + line.closest_index_for_x(position.x - bounds.left() + self.horizontal_offset)
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.enabled {
            return;
        }
        window.focus(&self.focus_handle, cx);
        // Pointer focus and caret placement are one visible interaction. Do not wait for the
        // asynchronous focus observer before making the caret eligible for painting.
        self.has_focus = true;
        self.is_selecting = true;
        let offset = self.index_for_mouse_position(event.position);
        if event.modifiers.shift {
            self.select_to(offset, cx);
        } else {
            self.move_to(offset, cx);
        }
    }

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, _: &mut Window, cx: &mut Context<Self>) {
        if let Some((start_y, start_offset)) = self.scrollbar_drag {
            let dragged = (event.position.y - start_y) * self.scrollbar_drag_scale;
            let offset = (start_offset - dragged).clamp(-self.scrollbar_max_offset, px(0.));
            let current = self.scroll_handle.offset();
            self.scroll_handle.set_offset(point(current.x, offset));
            cx.notify();
            return;
        }
        if self.enabled && self.is_selecting {
            let offset = self.index_for_mouse_position(event.position);
            self.select_to(offset, cx);
        }
    }

    fn on_mouse_up(&mut self, _: &MouseUpEvent, _: &mut Window, _: &mut Context<Self>) {
        self.is_selecting = false;
        self.scrollbar_drag = None;
    }

    fn on_scrollbar_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.is_selecting = false;
        self.scrollbar_drag = Some((event.position.y, self.scroll_handle.offset().y));
        cx.stop_propagation();
    }

    fn cancel_composition(&mut self) {
        if let Some(composition) = self.composition.take() {
            self.restore(composition.original);
        }
    }
}

impl EventEmitter<TextEditorChange> for TextEditor {}

impl Focusable for TextEditor {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EntityInputHandler for TextEditor {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        actual_range: &mut Option<Range<usize>>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<String> {
        let range = self.range_from_utf16(&range_utf16);
        actual_range.replace(self.range_to_utf16(&range));
        Some(self.content[range].to_owned())
    }

    fn selected_text_range(
        &mut self,
        ignore_disabled_input: bool,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        if !self.enabled && !ignore_disabled_input {
            return None;
        }
        Some(UTF16Selection {
            range: self.range_to_utf16(&self.selected_range),
            reversed: self.selection_reversed,
        })
    }

    fn marked_text_range(&self, _: &mut Window, _: &mut Context<Self>) -> Option<Range<usize>> {
        self.composition
            .as_ref()
            .map(|composition| self.range_to_utf16(&composition.marked_range))
    }

    fn unmark_text(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        if !self.enabled || self.read_only {
            self.cancel_composition();
            cx.notify();
            return;
        }
        if let Some(composition) = self.composition.take() {
            self.committed = self.content.clone();
            if composition.original.text != self.committed {
                self.undo_stack.push(composition.original);
                self.redo_stack.clear();
                self.emit_change(cx);
            }
            self.reveal_cursor();
            self.reset_blink(cx);
            cx.notify();
        }
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.enabled || self.read_only {
            return;
        }
        let before = self
            .composition
            .as_ref()
            .map(|composition| composition.original.clone())
            .unwrap_or_else(|| self.snapshot());
        let range = range_utf16
            .as_ref()
            .map(|range| self.range_from_utf16(range))
            .or_else(|| {
                self.composition
                    .as_ref()
                    .map(|composition| composition.marked_range.clone())
            })
            .unwrap_or_else(|| self.selected_range.clone());
        self.composition = None;
        self.replace_range(range, new_text);
        self.commit_edit(before, cx);
        self.reset_blink(cx);
        self.reveal_cursor();
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range_utf16: Option<Range<usize>>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.enabled || self.read_only {
            return;
        }
        let original = self
            .composition
            .as_ref()
            .map(|composition| composition.original.clone())
            .unwrap_or_else(|| self.snapshot());
        let range = range_utf16
            .as_ref()
            .map(|range| self.range_from_utf16(range))
            .or_else(|| {
                self.composition
                    .as_ref()
                    .map(|composition| composition.marked_range.clone())
            })
            .unwrap_or_else(|| self.selected_range.clone());
        let text = normalize_multi_line(new_text);
        let start = range.start;
        self.replace_range(range, &text);
        let marked_range = start..start + text.len();
        if text.is_empty() {
            self.restore(original);
        } else {
            if let Some(relative) = new_selected_range_utf16 {
                let selected = utf16_range_in(&text, relative);
                self.selected_range = start + selected.start..start + selected.end;
            }
            self.composition = Some(Composition {
                original,
                marked_range,
            });
        }
        self.reset_blink(cx);
        self.reveal_cursor();
        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        bounds: Bounds<Pixels>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let line_height = bounds.size.height / self.last_layouts.len().max(1) as f32;
        let range = self.range_from_utf16(&range_utf16);
        let before = &self.content[..range.start];
        let row = before.bytes().filter(|byte| *byte == b'\n').count();
        let line_start = before.rfind('\n').map_or(0, |ix| ix + 1);
        let line = self.last_layouts.get(row)?;
        Some(Bounds::from_corners(
            point(
                bounds.left() + line.x_for_index(range.start - line_start) - self.horizontal_offset,
                bounds.top() + line_height * row as f32,
            ),
            point(
                bounds.left()
                    + line
                        .x_for_index(range.end.min(self.line_bounds(range.start).end) - line_start)
                    - self.horizontal_offset,
                bounds.top() + line_height * (row + 1) as f32,
            ),
        ))
    }

    fn character_index_for_point(
        &mut self,
        position: Point<Pixels>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<usize> {
        Some(self.offset_to_utf16(self.index_for_mouse_position(position)))
    }
}

struct TextElement {
    input: Entity<TextEditor>,
}

struct PrepaintState {
    lines: Option<Vec<ShapedLine>>,
    caret: Option<PaintQuad>,
    selections: Vec<PaintQuad>,
    offset: Pixels,
}

impl IntoElement for TextElement {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for TextElement {
    type RequestLayoutState = ();
    type PrepaintState = PrepaintState;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = relative(1.).into();
        let line_count = self.input.read(cx).content.matches('\n').count() + 1;
        style.size.height = (window.line_height() * line_count as f32).into();
        style.min_size.height = window.line_height().into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let input = self.input.read(cx);
        let is_placeholder = input.content.is_empty();
        let display: SharedString = if is_placeholder {
            input.placeholder.clone()
        } else {
            input.content.clone()
        };
        let style = window.text_style();
        let caret_color = style.color;
        let color = if is_placeholder {
            input.theme.text_editor.field.placeholder.gpui().into()
        } else {
            style.color
        };
        let line_texts: Vec<&str> = if is_placeholder {
            vec![display.as_ref()]
        } else {
            display.split('\n').collect()
        };
        let lines: Vec<ShapedLine> = line_texts
            .iter()
            .map(|text| {
                window.text_system().shape_line(
                    SharedString::from((*text).to_owned()),
                    style.font_size.to_pixels(window.rem_size()),
                    &[TextRun {
                        len: text.len(),
                        font: style.font(),
                        color,
                        background_color: None,
                        underline: None,
                        strikethrough: None,
                    }],
                    None,
                )
            })
            .collect();
        let cursor_offset = input.cursor_offset();
        let before_cursor = &input.content[..cursor_offset];
        let cursor_row = before_cursor.bytes().filter(|byte| *byte == b'\n').count();
        let cursor_line_start = before_cursor.rfind('\n').map_or(0, |ix| ix + 1);
        let cursor_x = if is_placeholder {
            px(0.)
        } else {
            lines[cursor_row].x_for_index(cursor_offset - cursor_line_start)
        };
        let available = bounds.size.width;
        let mut offset = input.horizontal_offset;
        if cursor_x < offset {
            offset = cursor_x;
        } else if cursor_x > offset + available - px(2.) {
            offset = (cursor_x - available + px(2.)).max(px(0.));
        }
        let origin_x = bounds.left() - offset;
        let line_height = window.line_height();
        let mut selections = Vec::new();
        if !is_placeholder && !input.selected_range.is_empty() {
            let mut start = 0;
            for (row, (line, text)) in lines.iter().zip(line_texts.iter()).enumerate() {
                let end = start + text.len();
                let selected_start = input.selected_range.start.clamp(start, end) - start;
                let selected_end = input.selected_range.end.clamp(start, end) - start;
                if selected_start < selected_end
                    || (input.selected_range.start <= end && input.selected_range.end > end)
                {
                    selections.push(fill(
                        Bounds::from_corners(
                            point(
                                origin_x + line.x_for_index(selected_start),
                                bounds.top() + line_height * row as f32,
                            ),
                            point(
                                origin_x + line.x_for_index(selected_end),
                                bounds.top() + line_height * (row + 1) as f32,
                            ),
                        ),
                        input.theme.text_editor.field.selection_background.gpui(),
                    ));
                }
                start = end + 1;
            }
        }
        let caret = input.is_caret_visible().then(|| {
            let caret_x = window.pixel_snap(origin_x + cursor_x);
            fill(
                Bounds::new(
                    point(caret_x, bounds.top() + line_height * cursor_row as f32),
                    gpui::size(px(1.), line_height),
                ),
                caret_color,
            )
        });
        PrepaintState {
            lines: Some(lines),
            caret,
            selections,
            offset,
        }
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        state: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let (focus_handle, enabled) = {
            let input = self.input.read(cx);
            (input.focus_handle.clone(), input.enabled)
        };
        if enabled {
            window.handle_input(
                &focus_handle,
                ElementInputHandler::new(bounds, self.input.clone()),
                cx,
            );
        }
        for selection in state.selections.drain(..) {
            window.paint_quad(selection);
        }
        let lines = state
            .lines
            .take()
            .expect("text lines must be shaped before paint");
        for (row, line) in lines.iter().enumerate() {
            line.paint(
                point(
                    bounds.left() - state.offset,
                    bounds.top() + window.line_height() * row as f32,
                ),
                window.line_height(),
                gpui::TextAlign::Left,
                None,
                window,
                cx,
            )
            .expect("painting a shaped text line must succeed");
        }
        if focus_handle.is_focused(window)
            && let Some(caret) = state.caret.take()
        {
            window.paint_quad(caret);
        }
        let offset = state.offset;
        self.input.update(cx, |input, _| {
            input.last_layouts = lines;
            input.last_bounds = Some(bounds);
            input.horizontal_offset = offset;
        });
    }
}

impl Render for TextEditor {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Keep enough room for one full text line plus the 4 px vertical padding and 1 px border
        // on each side, even when an ancestor requests a smaller height.
        let minimum_height = window.line_height() + VERTICAL_CHROME;
        if self.focus_subscriptions.is_none() {
            let focus_handle = self.focus_handle.clone();
            let focus_subscription = cx.on_focus(&focus_handle, window, |input, _, cx| {
                input.start_blink(cx);
            });
            let blur_subscription = cx.on_blur(&focus_handle, window, |input, _, cx| {
                input.stop_blink(cx);
            });
            self.focus_subscriptions = Some(vec![focus_subscription, blur_subscription]);
            if focus_handle.is_focused(window) {
                self.start_blink(cx);
            }
        }
        let border = if self.focus_handle.is_focused(window) && self.enabled {
            self.theme.text_editor.field.focused_border.gpui()
        } else if self.enabled {
            self.theme.text_editor.field.border.gpui()
        } else {
            self.theme.text_editor.field.disabled_border.gpui()
        };
        let background = if self.enabled {
            self.theme.text_editor.field.background.gpui()
        } else {
            self.theme.text_editor.field.disabled_background.gpui()
        };
        self.line_height = window.line_height();
        self.scrollbar_drag_scale = 0.;
        self.scrollbar_max_offset = px(0.);
        let scroll_handle = self.scroll_handle.clone();
        let viewport_height = scroll_handle.bounds().size.height;
        let scrollbar = (viewport_height > px(0.))
            .then(|| {
                let content_height =
                    window.line_height() * (self.content.matches('\n').count() + 1);
                if content_height <= viewport_height {
                    return None;
                }
                let max_offset = content_height - viewport_height;
                let thumb_height = (viewport_height * (viewport_height / content_height))
                    .max(px(24.))
                    .min(viewport_height);
                let thumb_travel = viewport_height - thumb_height;
                self.scrollbar_max_offset = max_offset;
                self.scrollbar_drag_scale = if thumb_travel > px(0.) {
                    max_offset / thumb_travel
                } else {
                    0.
                };
                let progress = (-scroll_handle.offset().y / max_offset).clamp(0., 1.);
                let thumb_top = (viewport_height - thumb_height) * progress;
                Some(
                    div()
                        .absolute()
                        .right(px(2.))
                        .top(px(4.))
                        .w(px(6.))
                        .h(viewport_height)
                        .rounded(px(3.))
                        .bg(self.theme.text_editor.scrollbar_track.gpui())
                        .child(
                            div()
                                .absolute()
                                .top(thumb_top)
                                .w_full()
                                .h(thumb_height)
                                .rounded(px(3.))
                                .bg(self.theme.text_editor.scrollbar_thumb.gpui())
                                .hover({
                                    let color = self.theme.text_editor.scrollbar_thumb_hover.gpui();
                                    move |style| style.bg(color)
                                })
                                .cursor(CursorStyle::PointingHand)
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(Self::on_scrollbar_mouse_down),
                                ),
                        ),
                )
            })
            .flatten();
        let content = div()
            .id("text-editor-scroll-area")
            .w_full()
            .flex_1()
            .overflow_y_scroll()
            .track_scroll(&scroll_handle)
            .on_scroll_wheel(cx.listener(|_, _: &ScrollWheelEvent, _, cx| cx.notify()))
            .child(TextElement { input: cx.entity() });

        div()
            .debug_selector(|| "TEXT_INPUT".into())
            .key_context("TextEditor")
            .relative()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .when(self.enabled, |element| {
                element.track_focus(&self.focus_handle(cx))
            })
            .cursor(if self.enabled {
                CursorStyle::IBeam
            } else {
                CursorStyle::Arrow
            })
            .border_1()
            .border_color(border)
            .rounded(px(4.))
            .bg(background)
            .text_color(self.theme.text_editor.field.text.gpui())
            .when(self.enabled, |element| {
                let color = self.theme.text_editor.field.hover_background.gpui();
                element.hover(move |style| style.bg(color))
            })
            .min_h(minimum_height)
            .px(px(6.))
            .py(px(4.))
            .overflow_hidden()
            .on_action(cx.listener(Self::on_backspace))
            .on_action(cx.listener(Self::on_delete))
            .on_action(cx.listener(Self::on_left))
            .on_action(cx.listener(Self::on_right))
            .on_action(cx.listener(Self::on_word_left))
            .on_action(cx.listener(Self::on_word_right))
            .on_action(cx.listener(Self::on_start))
            .on_action(cx.listener(Self::on_end))
            .on_action(cx.listener(Self::on_select_left))
            .on_action(cx.listener(Self::on_select_right))
            .on_action(cx.listener(Self::on_select_word_left))
            .on_action(cx.listener(Self::on_select_word_right))
            .on_action(cx.listener(Self::on_select_start))
            .on_action(cx.listener(Self::on_select_end))
            .on_action(cx.listener(Self::on_select_all))
            .on_action(cx.listener(Self::on_copy))
            .on_action(cx.listener(Self::on_cut))
            .on_action(cx.listener(Self::on_paste))
            .on_action(cx.listener(Self::on_undo))
            .on_action(cx.listener(Self::on_redo))
            .on_action(cx.listener(Self::on_insert_newline))
            .on_action(cx.listener(Self::on_up))
            .on_action(cx.listener(Self::on_down))
            .on_action(cx.listener(Self::on_page_up))
            .on_action(cx.listener(Self::on_page_down))
            .on_action(cx.listener(Self::on_document_start))
            .on_action(cx.listener(Self::on_document_end))
            .on_action(cx.listener(Self::on_select_up))
            .on_action(cx.listener(Self::on_select_down))
            .on_action(cx.listener(Self::on_select_page_up))
            .on_action(cx.listener(Self::on_select_page_down))
            .on_action(cx.listener(Self::on_select_document_start))
            .on_action(cx.listener(Self::on_select_document_end))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
            .on_mouse_move(cx.listener(Self::on_mouse_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .child(content)
            .children(scrollbar)
    }
}

fn normalize_multi_line(value: &str) -> String {
    value.replace("\r\n", "\n").replace('\r', "\n")
}

fn utf16_range_in(value: &str, range: Range<usize>) -> Range<usize> {
    fn convert(value: &str, offset: usize) -> usize {
        let mut utf8 = 0;
        let mut utf16 = 0;
        for ch in value.chars() {
            if utf16 >= offset || utf16 + ch.len_utf16() > offset {
                break;
            }
            utf16 += ch.len_utf16();
            utf8 += ch.len_utf8();
        }
        utf8
    }
    convert(value, range.start)..convert(value, range.end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_all_common_line_endings() {
        assert_eq!(normalize_multi_line("a\r\nb\nc\rd"), "a\nb\nc\nd");
    }

    #[test]
    fn converts_utf16_ranges_without_splitting_surrogates() {
        assert_eq!(utf16_range_in("a\u{1f600}b", 1..3), 1..5);
        assert_eq!(utf16_range_in("a\u{1f600}b", 2..2), 1..1);
    }

    #[test]
    fn grapheme_boundaries_keep_combining_sequences_together() {
        let value = "a\u{301}b";
        let boundaries: Vec<_> = value
            .grapheme_indices(true)
            .map(|(index, _)| index)
            .collect();
        assert_eq!(boundaries, vec![0, 3]);
    }
}
