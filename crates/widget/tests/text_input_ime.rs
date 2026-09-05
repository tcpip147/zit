use gpui::{AppContext, EntityInputHandler, TestAppContext, VisualTestContext};
use widget::TextInput;

#[gpui::test]
fn composition_is_marked_then_committed_once(cx: &mut TestAppContext) {
    let window = cx.update(|cx| {
        cx.open_window(Default::default(), |_, cx| cx.new(TextInput::new))
            .unwrap()
    });
    let mut cx = VisualTestContext::from_window(window.into(), cx);
    let input = window.root(&mut cx).unwrap();
    input.update_in(&mut cx, |input, window, cx| {
        input.replace_and_mark_text_in_range(None, "한", Some(1..1), window, cx);
    });
    assert_eq!(
        input.read_with(&cx, |input, _| input.value().to_owned()),
        ""
    );
    let marked = input.update_in(&mut cx, |input, window, cx| {
        input.marked_text_range(window, cx)
    });
    assert_eq!(marked, Some(0..1));
    input.update_in(&mut cx, |input, window, cx| {
        input.replace_text_in_range(None, "한", window, cx);
    });
    assert_eq!(
        input.read_with(&cx, |input, _| input.value().to_owned()),
        "한"
    );
}

#[gpui::test]
fn empty_marked_update_cancels_composition(cx: &mut TestAppContext) {
    let window = cx.update(|cx| {
        cx.open_window(Default::default(), |_, cx| {
            cx.new(|cx| TextInput::with_value("base", cx))
        })
        .unwrap()
    });
    let mut cx = VisualTestContext::from_window(window.into(), cx);
    let input = window.root(&mut cx).unwrap();
    input.update_in(&mut cx, |input, window, cx| {
        input.replace_and_mark_text_in_range(None, "候補", None, window, cx);
        input.replace_and_mark_text_in_range(None, "", None, window, cx);
    });
    assert_eq!(
        input.read_with(&cx, |input, _| input.value().to_owned()),
        "base"
    );
}
