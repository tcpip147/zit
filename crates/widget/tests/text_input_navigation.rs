use gpui::{AppContext, Focusable, TestAppContext, VisualTestContext};
use widget::{Left, SelectLeft, Start, TextInput};

#[gpui::test]
fn actions_move_and_extend_selection_on_grapheme_boundaries(cx: &mut TestAppContext) {
    let window = cx.update(|cx| {
        cx.open_window(Default::default(), |_, cx| {
            cx.new(|cx| TextInput::with_value("a\u{301}b", cx))
        })
        .unwrap()
    });
    let mut cx = VisualTestContext::from_window(window.into(), cx);
    let input = window.root(&mut cx).unwrap();
    let focus = input.read_with(&cx, |input, cx| input.focus_handle(cx));
    cx.update(|window, cx| {
        focus.focus(window, cx);
        focus.dispatch_action(&Left, window, cx);
    });
    assert_eq!(input.read_with(&cx, |input, _| input.selection()), 3..3);
    cx.update(|window, cx| focus.dispatch_action(&SelectLeft, window, cx));
    assert_eq!(input.read_with(&cx, |input, _| input.selection()), 0..3);
    cx.update(|window, cx| focus.dispatch_action(&Start, window, cx));
    assert_eq!(input.read_with(&cx, |input, _| input.selection()), 0..0);
}
