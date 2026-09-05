use gpui::{
    App, Bounds, Context, Entity, Focusable, Window, WindowBounds, WindowOptions, div, prelude::*,
    px, rgb, size,
};
use gpui_platform::application;
use widget::{TextInput, init};

struct Example {
    first: Entity<TextInput>,
    second: Entity<TextInput>,
    _subscriptions: Vec<gpui::Subscription>,
}

impl gpui::Render for Example {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let first_value = self.first.read(cx).value().to_owned();
        let second_value = self.second.read(cx).value().to_owned();
        let first_status = self.first.read(cx);
        let first_focused = first_status.focus_handle(cx).is_focused(window);
        let first_caret = first_status.is_caret_visible();
        let first_selection = first_status.selection();
        let second_enabled = self.second.read(cx).is_enabled();
        div()
            .flex()
            .flex_col()
            .gap_3()
            .p_4()
            .size_full()
            .bg(rgb(0xf8fafc))
            .text_color(rgb(0x0f172a))
            .child("Reusable ZPUI TextInput")
            .child(self.first.clone())
            .child(format!("First value: {first_value}"))
            .child(format!(
                "First state: focused={first_focused}, caret_visible={first_caret}, selection={first_selection:?}"
            ))
            .child(self.second.clone())
            .child(format!("Second value: {second_value}"))
            .child(format!("Second enabled: {second_enabled}"))
            .child(
                div()
                    .id("toggle-second")
                    .px_2()
                    .py_1()
                    .bg(rgb(0xe2e8f0))
                    .cursor_pointer()
                    .on_click(cx.listener(|this, _, window, cx| {
                        let enabled = this.second.read(cx).is_enabled();
                        this.second.update(cx, |input, cx| {
                            input.set_enabled(!enabled, window, cx);
                        });
                        cx.notify();
                    }))
                    .child("Toggle second input"),
            )
    }
}

fn main() {
    application().run(|cx: &mut App| {
        init(cx);
        let bounds = Bounds::centered(None, size(px(560.), px(360.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                let first = cx.new(|cx| {
                    let mut input = TextInput::new(cx);
                    input.set_placeholder("Type here…", cx);
                    input
                });
                let second = cx.new(|cx| {
                    let mut input = TextInput::with_value("Independent value", cx);
                    input.set_placeholder("Second input", cx);
                    input
                });
                first.focus_handle(cx).focus(window, cx);
                cx.new(|cx| {
                    let subscriptions = vec![
                        cx.subscribe(&first, |_, _, _, cx| cx.notify()),
                        cx.subscribe(&second, |_, _, _, cx| cx.notify()),
                        cx.observe(&first, |_, _, cx| cx.notify()),
                        cx.observe(&second, |_, _, cx| cx.notify()),
                    ];
                    Example {
                        first,
                        second,
                        _subscriptions: subscriptions,
                    }
                })
            },
        )
        .expect("TextInput example window should open");
        cx.activate(true);
    });
}
