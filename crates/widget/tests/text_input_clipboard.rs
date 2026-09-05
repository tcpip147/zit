use gpui::{AppContext, ClipboardItem, Focusable, TestAppContext, VisualTestContext};
use widget::{Paste, SelectAll, TextInput};

#[gpui::test]
fn paste_replaces_selection_and_normalizes_lines(cx: &mut TestAppContext) {
    let window = cx.update(|cx| {
        cx.open_window(Default::default(), |_, cx| {
            cx.new(|cx| TextInput::with_value("replace me", cx))
        })
        .unwrap()
    });
    let mut cx = VisualTestContext::from_window(window.into(), cx);
    let input = window.root(&mut cx).unwrap();
    let focus = input.read_with(&cx, |input, cx| input.focus_handle(cx));
    cx.update(|window, cx| {
        focus.focus(window, cx);
        focus.dispatch_action(&SelectAll, window, cx);
        cx.write_to_clipboard(ClipboardItem::new_string("one\ntwo".to_owned()));
        focus.dispatch_action(&Paste, window, cx);
    });
    assert_eq!(
        input.read_with(&cx, |input, _| input.value().to_owned()),
        "one two"
    );
}
