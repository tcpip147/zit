//! Zit application entry point.

mod about;
mod assets;
mod menu;
mod platform;
mod shell;
mod startup_error;
mod titlebar;

use assets::ZitAssets;
use gpui::{App, AppContext, Bounds, KeyBinding, WindowBounds, WindowOptions, px, size};
use gpui_platform::application;
use shell::{MenuDown, MenuEnter, MenuEscape, MenuLeft, MenuRight, MenuUp, ZitShell};
use startup_error::StartupError;
use widget::Theme;
use zit::project::{GlobalProjectStore, ProjectStore, default_session_path};

fn open_main_window(cx: &mut App, theme: Theme) -> gpui::Result<()> {
    if std::env::var_os("ZIT_SIMULATE_STARTUP_FAILURE").is_some() {
        return Err(std::io::Error::other("simulated main-window initialization failure").into());
    }
    let bounds = Bounds::centered(None, size(px(1024.0), px(720.0)), cx);
    cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: None,
            window_min_size: Some(size(px(480.0), px(320.0))),
            ..Default::default()
        },
        |window, cx| cx.new(|cx| ZitShell::new(window, theme, cx)),
    )?;
    Ok(())
}

fn open_startup_error(cx: &mut App, message: String) -> gpui::Result<()> {
    let bounds = Bounds::centered(None, size(px(480.0), px(220.0)), cx);
    cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            window_min_size: Some(size(px(360.0), px(180.0))),
            ..Default::default()
        },
        |_, cx| cx.new(|_| StartupError::new(message)),
    )?;
    Ok(())
}

fn main() {
    let theme = match std::env::var_os("ZIT_THEME") {
        Some(path) => match Theme::load(&path) {
            Ok(theme) => theme,
            Err(error) => {
                eprintln!(
                    "Zit could not load theme {}: {error}",
                    path.to_string_lossy()
                );
                Theme::default()
            }
        },
        None => Theme::default(),
    };
    let app = application().with_assets(ZitAssets::bundled());
    let reopen_theme = theme.clone();
    app.on_reopen(move |cx| {
        if cx.windows().is_empty() && open_main_window(cx, reopen_theme.clone()).is_ok() {
            cx.activate(true);
        }
    });
    app.run(|cx: &mut App| {
        let project_store = cx.new(|_| ProjectStore::new(default_session_path()));
        cx.set_global(GlobalProjectStore(project_store.clone()));
        project_store.update(cx, |store, cx| {
            store.restore(cx);
            store.seed_test_projects(3, cx);
            store.set_first_test_project(r"D:\Project\git-graph-sample".into(), cx);
            store.set_test_project_repository(1, r"D:\Project\zed".into(), cx);
        });
        cx.bind_keys([
            KeyBinding::new("left", MenuLeft, None),
            KeyBinding::new("right", MenuRight, None),
            KeyBinding::new("up", MenuUp, None),
            KeyBinding::new("down", MenuDown, None),
            KeyBinding::new("enter", MenuEnter, None),
            KeyBinding::new("escape", MenuEscape, None),
        ]);
        if let Err(error) = open_main_window(cx, theme) {
            eprintln!("Zit could not create its main window: {error:#}");
            let message = format!("Zit could not start its main window.\n\n{error:#}");
            if open_startup_error(cx, message).is_err() {
                cx.quit();
            } else {
                cx.activate(true);
            }
            return;
        }
        cx.activate(true);
    });
}
