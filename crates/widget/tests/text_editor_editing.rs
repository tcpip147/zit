use gpui::{AppContext, EntityInputHandler, TestAppContext, VisualTestContext};
use widget::TextEditor;

#[gpui::test]
fn inserts_and_replaces_across_lines(cx: &mut TestAppContext) {
    let window = cx.update(|cx| {
        cx.open_window(Default::default(), |_, cx| cx.new(TextEditor::new))
            .unwrap()
    });
    let mut cx = VisualTestContext::from_window(window.into(), cx);
    let editor = window.root(&mut cx).unwrap();
    editor.update_in(&mut cx, |editor, window, cx| {
        editor.replace_text_in_range(None, "a\nb\n", window, cx);
    });
    assert_eq!(
        editor.read_with(&cx, |editor, _| editor.value().to_owned()),
        "a\nb\n"
    );
    assert_eq!(editor.read_with(&cx, |editor, _| editor.selection()), 4..4);
}
