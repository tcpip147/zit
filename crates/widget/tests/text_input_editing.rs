use gpui::{AppContext, EntityInputHandler, TestAppContext, VisualTestContext};
use widget::TextInput;

#[gpui::test]
fn inserts_replaces_and_normalizes_single_line_text(cx: &mut TestAppContext) {
    let window = cx.update(|cx| {
        cx.open_window(Default::default(), |_, cx| {
            cx.new(|cx| TextInput::with_value("hello", cx))
        })
        .unwrap()
    });
    let mut cx = VisualTestContext::from_window(window.into(), cx);
    let input = window.root(&mut cx).unwrap();

    input.update_in(&mut cx, |input, window, cx| {
        input.replace_text_in_range(Some(0..5), "one\r\ntwo", window, cx);
    });

    assert_eq!(
        input.read_with(&cx, |input, _| input.value().to_owned()),
        "one two"
    );
    assert_eq!(input.read_with(&cx, |input, _| input.selection()), 7..7);
}

#[gpui::test]
fn disabled_input_rejects_platform_replacement(cx: &mut TestAppContext) {
    let window = cx.update(|cx| {
        cx.open_window(Default::default(), |_, cx| {
            cx.new(|cx| TextInput::with_value("stable", cx))
        })
        .unwrap()
    });
    let mut cx = VisualTestContext::from_window(window.into(), cx);
    let input = window.root(&mut cx).unwrap();

    input.update_in(&mut cx, |input, window, cx| {
        input.set_enabled(false, window, cx);
        input.replace_text_in_range(None, "changed", window, cx);
    });

    assert_eq!(
        input.read_with(&cx, |input, _| input.value().to_owned()),
        "stable"
    );
}
