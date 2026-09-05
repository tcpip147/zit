use gpui::{AppContext, EntityInputHandler, Focusable, TestAppContext, VisualTestContext};
use widget::{Redo, TextInput, Undo};

#[gpui::test]
fn undo_redo_restores_text_and_new_edit_invalidates_redo(cx: &mut TestAppContext) {
    let window = cx.update(|cx| {
        cx.open_window(Default::default(), |_, cx| cx.new(TextInput::new))
            .unwrap()
    });
    let mut cx = VisualTestContext::from_window(window.into(), cx);
    let input = window.root(&mut cx).unwrap();
    let focus = input.read_with(&cx, |input, cx| input.focus_handle(cx));
    input.update_in(&mut cx, |input, window, cx| {
        input.replace_text_in_range(None, "one", window, cx)
    });
    cx.update(|window, cx| focus.dispatch_action(&Undo, window, cx));
    assert_eq!(
        input.read_with(&cx, |input, _| input.value().to_owned()),
        ""
    );
    cx.update(|window, cx| focus.dispatch_action(&Redo, window, cx));
    assert_eq!(
        input.read_with(&cx, |input, _| input.value().to_owned()),
        "one"
    );
    cx.update(|window, cx| focus.dispatch_action(&Undo, window, cx));
    input.update_in(&mut cx, |input, window, cx| {
        input.replace_text_in_range(None, "new", window, cx)
    });
    cx.update(|window, cx| focus.dispatch_action(&Redo, window, cx));
    assert_eq!(
        input.read_with(&cx, |input, _| input.value().to_owned()),
        "new"
    );
}
