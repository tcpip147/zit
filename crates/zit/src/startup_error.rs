use gpui::{Context, Render, Window, div, prelude::*, px, rgb};

pub struct StartupError {
    message: String,
}

impl StartupError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Render for StartupError {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .text_xs()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgb(0xf5f5f5))
            .text_color(rgb(0x202124))
            .child(
                div()
                    .w(px(420.0))
                    .p_6()
                    .rounded_md()
                    .bg(rgb(0xffffff))
                    .child(self.message.clone()),
            )
    }
}
