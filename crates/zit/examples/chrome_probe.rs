use gpui::{
    App, AppContext, Bounds, Context, MouseButton, Window, WindowBounds, WindowControlArea,
    WindowOptions, div, prelude::*, px, rgb, size,
};
use gpui_platform::application;

struct ChromeProbe;

impl ChromeProbe {
    fn control(label: &'static str, area: WindowControlArea) -> impl IntoElement {
        div()
            .id(label)
            .w(px(46.0))
            .h_full()
            .flex()
            .items_center()
            .justify_center()
            .hover(|style| style.bg(rgb(0x3c4043)))
            .active(|style| style.bg(rgb(0x5f6368)))
            .window_control_area(area)
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(label)
    }
}

impl Render for ChromeProbe {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let state = format!(
            "maximized={} fullscreen={} scale={}",
            window.is_maximized(),
            window.is_fullscreen(),
            window.scale_factor()
        );

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(0x202124))
            .text_color(rgb(0xf1f3f4))
            .child(
                div()
                    .h(px(36.0))
                    .w_full()
                    .flex()
                    .items_center()
                    .window_control_area(WindowControlArea::Drag)
                    .child(div().px_3().child("File"))
                    .child(div().px_3().child("Help"))
                    .child(div().flex_1())
                    .child(Self::control("—", WindowControlArea::Min))
                    .child(Self::control("□", WindowControlArea::Max))
                    .child(Self::control("×", WindowControlArea::Close)),
            )
            .child(div().p_4().child("Native capability probe").child(state))
    }
}

fn main() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.0), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: None,
                window_min_size: Some(size(px(480.0), px(320.0))),
                ..Default::default()
            },
            |_, cx| cx.new(|_| ChromeProbe),
        )
        .expect("failed to open the chrome capability probe");
        cx.activate(true);
    });
}
