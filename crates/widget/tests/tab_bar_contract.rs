use gpui::{AppContext, Context, Render, TestAppContext, Window, div, prelude::*};
use widget::{TabBar, TabBarError, TabId, TabSpec};

struct Page(&'static str);

impl Render for Page {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().child(self.0)
    }
}

fn tabs(cx: &mut TestAppContext) -> Vec<TabSpec> {
    ["first", "middle", "last"]
        .into_iter()
        .map(|id| {
            let page = cx.new(move |_| Page(id));
            TabSpec::new(id, id, "tab-bar/fallback-project.svg", page)
        })
        .collect()
}

#[gpui::test]
fn construction_selects_first_and_preserves_page_identity(cx: &mut TestAppContext) {
    let specs = tabs(cx);
    let first_page = specs[0].page.entity_id();
    let bar = cx.new(|cx| TabBar::new(specs, Some(TabId::new("unknown")), cx).unwrap());

    assert_eq!(
        bar.read_with(cx, |bar, _| bar.active_id().cloned()),
        Some(TabId::new("first"))
    );
    assert_eq!(
        bar.read_with(cx, |bar, _| bar.active_page_entity_id()),
        Some(first_page)
    );

    bar.update(cx, |bar, cx| assert!(bar.select(&TabId::new("middle"), cx)));
    let middle_page = bar.read_with(cx, |bar, _| bar.active_page_entity_id());
    bar.update(cx, |bar, cx| assert!(bar.select(&TabId::new("middle"), cx)));
    assert_eq!(
        bar.read_with(cx, |bar, _| bar.active_page_entity_id()),
        middle_page
    );
    bar.update(cx, |bar, cx| {
        assert!(!bar.select(&TabId::new("missing"), cx))
    });
}

#[gpui::test]
fn duplicate_ids_are_rejected(cx: &mut TestAppContext) {
    let page_a = cx.new(|_| Page("a"));
    let page_b = cx.new(|_| Page("b"));
    let specs = [
        TabSpec::new("same", "A", "", page_a),
        TabSpec::new("same", "B", "", page_b),
    ];
    assert!(matches!(
        TabBar::validate(&specs),
        Err(TabBarError::DuplicateId(id)) if id == TabId::new("same")
    ));
}

#[gpui::test]
fn close_uses_right_then_left_and_finishes_empty(cx: &mut TestAppContext) {
    let specs = tabs(cx);
    let bar = cx.new(|cx| TabBar::new(specs, Some(TabId::new("middle")), cx).unwrap());

    bar.update(cx, |bar, cx| assert!(bar.close(&TabId::new("middle"), cx)));
    assert_eq!(
        bar.read_with(cx, |bar, _| bar.active_id().cloned()),
        Some(TabId::new("last"))
    );
    bar.update(cx, |bar, cx| assert!(bar.close(&TabId::new("last"), cx)));
    assert_eq!(
        bar.read_with(cx, |bar, _| bar.active_id().cloned()),
        Some(TabId::new("first"))
    );
    bar.update(cx, |bar, cx| assert!(bar.close(&TabId::new("first"), cx)));
    assert!(bar.read_with(cx, |bar, _| bar.is_empty()));
    assert_eq!(
        bar.read_with(cx, |bar, _| bar.active_page_entity_id()),
        None
    );
}

#[gpui::test]
fn close_filter_vetoes_without_mutating_state(cx: &mut TestAppContext) {
    let specs = tabs(cx);
    let bar = cx.new(|cx| TabBar::new(specs, None, cx).unwrap());
    bar.update(cx, |bar, _| {
        bar.set_close_filter(|id| id.as_str() != "first")
    });

    bar.update(cx, |bar, cx| {
        assert!(!bar.request_close(&TabId::new("first"), cx))
    });
    assert_eq!(bar.read_with(cx, |bar, _| bar.len()), 3);
    assert_eq!(
        bar.read_with(cx, |bar, _| bar.active_id().cloned()),
        Some(TabId::new("first"))
    );
}

#[gpui::test]
fn metadata_update_preserves_retained_page(cx: &mut TestAppContext) {
    let specs = tabs(cx);
    let page_id = specs[0].page.entity_id();
    let bar = cx.new(|cx| TabBar::new(specs, None, cx).unwrap());
    bar.update(cx, |bar, cx| {
        assert!(bar.update_metadata(&TabId::new("first"), "Renamed", "", cx));
    });
    assert_eq!(
        bar.read_with(cx, |bar, _| bar
            .project_name(&TabId::new("first"))
            .map(str::to_owned)),
        Some("Renamed".into())
    );
    assert_eq!(
        bar.read_with(cx, |bar, _| bar.active_page_entity_id()),
        Some(page_id)
    );
}
