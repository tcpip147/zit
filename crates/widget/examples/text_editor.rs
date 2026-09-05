use gpui::{
    App, Bounds, Context, Entity, Focusable, Window, WindowBounds, WindowOptions, div, prelude::*,
    px, rgb, size,
};
use gpui_platform::application;
use widget::{TextEditor, init_text_editor};

struct Example {
    editable: Entity<TextEditor>,
    read_only: Entity<TextEditor>,
}

impl gpui::Render for Example {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .p_4()
            .size_full()
            .bg(rgb(0xf8fafc))
            .text_color(rgb(0x0f172a))
            .child("Reusable ZPUI TextEditor")
            .child(div().w_full().h(px(180.)).child(self.editable.clone()))
            .child("Read-only")
            .child(div().w_full().h(px(96.)).child(self.read_only.clone()))
    }
}

fn main() {
    application().run(|cx: &mut App| {
        init_text_editor(cx);
        let bounds = Bounds::centered(None, size(px(720.), px(480.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                let editable = cx.new(|cx| {
                    let mut editor = TextEditor::with_value(
                        "A fixed-height editor\n\nAdd more lines here.\nThe scrollbar appears when needed.\nLine five\nLine six",
                        cx,
                    );
                    editor.set_placeholder("Write multiple lines…", cx);
                    editor
                });
                let read_only = cx.new(|cx| {
                    let mut editor = TextEditor::with_value("Selectable read-only text", cx);
                    editor.set_read_only(true, cx);
                    editor
                });
                editable.focus_handle(cx).focus(window, cx);
                cx.new(|_| Example {
                    editable,
                    read_only,
                })
            },
        )
        .expect("TextEditor example window should open");
        cx.activate(true);
    });
}
