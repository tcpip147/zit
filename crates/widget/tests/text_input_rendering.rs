use gpui::{
    AppContext, Context, Entity, Focusable, Render, TestAppContext, VisualTestContext, Window, div,
    prelude::*, px,
};
use widget::{SelectAll, TextInput};

struct ConstrainedHeightHost {
    input: Entity<TextInput>,
    text_size: gpui::Pixels,
}

impl Render for ConstrainedHeightHost {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .h(px(1.))
            .text_size(self.text_size)
            .child(self.input.clone())
    }
}

#[gpui::test]
fn empty_placeholder_and_long_value_render_without_changing_value(cx: &mut TestAppContext) {
    let window = cx.update(|cx| {
        cx.open_window(Default::default(), |_, cx| {
            cx.new(|cx| {
                let mut input = TextInput::new(cx);
                input.set_placeholder("A helpful hint", cx);
                input
            })
        })
        .unwrap()
    });
    let mut cx = VisualTestContext::from_window(window.into(), cx);
    let input = window.root(&mut cx).unwrap();
    assert_eq!(
        input.read_with(&cx, |input, _| input.value().to_owned()),
        ""
    );
    assert_eq!(
        input.read_with(&cx, |input, _| input.placeholder().to_owned()),
        "A helpful hint"
    );
    input.update_in(&mut cx, |input, _, cx| input.set_value("x".repeat(100), cx));
    assert_eq!(input.read_with(&cx, |input, _| input.value().len()), 100);
}

fn assert_constrained_input_keeps_text_height(cx: &mut TestAppContext, text_size: gpui::Pixels) {
    let window = cx.update(|cx| {
        cx.open_window(Default::default(), |_, cx| {
            let input = cx.new(|cx| TextInput::with_value("minimum height", cx));
            cx.new(|_| ConstrainedHeightHost { input, text_size })
        })
        .unwrap()
    });
    let mut cx = VisualTestContext::from_window(window.into(), cx);
    let host = window.root(&mut cx).unwrap();
    let input = host.read_with(&cx, |host, _| host.input.clone());
    let focus = input.read_with(&cx, |input, cx| input.focus_handle(cx));
    cx.run_until_parked();

    let initial_bounds = cx
        .debug_bounds("TEXT_INPUT")
        .expect("TextInput must publish test bounds after rendering");
    let minimum_with_vertical_chrome = text_size + px(10.);
    assert!(
        initial_bounds.size.height >= minimum_with_vertical_chrome,
        "TextInput height {:?} must retain text size {:?} and vertical chrome",
        initial_bounds.size.height,
        text_size
    );

    cx.update(|window, cx| {
        window.activate_window();
        focus.focus(window, cx);
    });
    cx.run_until_parked();
    let caret_bounds = cx.debug_bounds("TEXT_INPUT").unwrap();
    assert_eq!(caret_bounds.size.height, initial_bounds.size.height);

    cx.update(|window, cx| focus.dispatch_action(&SelectAll, window, cx));
    cx.run_until_parked();
    let selection_bounds = cx.debug_bounds("TEXT_INPUT").unwrap();
    assert_eq!(selection_bounds.size.height, initial_bounds.size.height);
}

#[gpui::test]
fn constrained_height_preserves_default_sized_text(cx: &mut TestAppContext) {
    assert_constrained_input_keeps_text_height(cx, px(14.));
}

#[gpui::test]
fn constrained_height_preserves_large_text(cx: &mut TestAppContext) {
    assert_constrained_input_keeps_text_height(cx, px(32.));
}
