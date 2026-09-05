use std::{borrow::Cow, fs, path::PathBuf};

use gpui::{
    App, AssetSource, Bounds, Context, Entity, Render, Result, SharedString, Window, WindowBounds,
    WindowOptions, div, prelude::*, px, rgb, size,
};
use gpui_platform::application;
use widget::{NF, TabBar, TabId, TabSpec};

struct ExampleAssets(PathBuf);

impl AssetSource for ExampleAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        let path = self.0.join(path);
        Ok(path
            .exists()
            .then(|| Cow::Owned(fs::read(path).unwrap_or_default())))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(fs::read_dir(self.0.join(path))?
            .filter_map(|entry| entry.ok()?.file_name().into_string().ok())
            .map(SharedString::from)
            .collect())
    }
}

struct ProjectPage {
    number: usize,
}

struct Example {
    tab_bar: Entity<TabBar>,
}

impl Render for Example {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(
                div()
                    .px_3()
                    .py_2()
                    .text_xs()
                    .bg(rgb(0xe8edf5))
                    .text_color(rgb(0x172033))
                    .child("Use the pointer to select, close, and scroll project tabs."),
            )
            .child(div().flex_1().overflow_hidden().child(self.tab_bar.clone()))
    }
}

impl Render for ProjectPage {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgb(0xf8fafc))
            .text_color(rgb(0x172033))
            .child(format!("Project page {}", self.number))
    }
}

fn main() {
    let assets = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
    application()
        .with_assets(ExampleAssets(assets))
        .run(|cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(760.), px(480.)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    window_min_size: Some(size(px(360.), px(240.))),
                    ..Default::default()
                },
                |_, cx| {
                    let tab_bar = cx.new(|cx| {
                        let tabs: Vec<_> = (1..=30)
                            .map(|number| {
                                let page = cx.new(|_| ProjectPage { number });
                                let name: SharedString = if number == 3 {
                                    "A project with a very long name".into()
                                } else {
                                    format!("Project {number}").into()
                                };
                                TabSpec::new(
                                    TabId::new(format!("project-{number}")),
                                    name,
                                    NF,
                                    page,
                                )
                            })
                            .collect();
                        TabBar::new(tabs, None, cx).expect("example tab IDs are unique")
                    });
                    cx.new(|_| Example { tab_bar })
                },
            )
            .expect("TabBar example window should open");
            cx.activate(true);
        });
}
