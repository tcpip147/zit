use gpui::{AppContext, EntityInputHandler, Focusable, TestAppContext, VisualTestContext, px};
use widget::{TextEditor, TextEditorDocumentStart};

#[gpui::test]
fn preserves_multiline_value_and_configuration(cx: &mut TestAppContext) {
    let editor = cx.new(|cx| TextEditor::with_value("first\r\nsecond\rthird", cx));
    assert_eq!(
        editor.read_with(cx, |editor, _| editor.value().to_owned()),
        "first\nsecond\nthird"
    );
    assert!(editor.read_with(cx, |editor, _| editor.is_enabled()));
    assert!(editor.read_with(cx, |editor, _| editor.is_wrapping()));
    editor.update(cx, |editor, cx| editor.set_read_only(true, cx));
    assert!(editor.read_with(cx, |editor, _| editor.is_read_only()));
}

#[gpui::test]
fn read_only_rejects_platform_replacement(cx: &mut TestAppContext) {
    let window = cx.update(|cx| {
        cx.open_window(Default::default(), |_, cx| {
            cx.new(|cx| {
                let mut editor = TextEditor::with_value("stable", cx);
                editor.set_read_only(true, cx);
                editor
            })
        })
        .unwrap()
    });
    let mut cx = VisualTestContext::from_window(window.into(), cx);
    let editor = window.root(&mut cx).unwrap();
    editor.update_in(&mut cx, |editor, window, cx| {
        editor.replace_text_in_range(None, "changed", window, cx);
    });
    assert_eq!(
        editor.read_with(&cx, |editor, _| editor.value().to_owned()),
        "stable"
    );
}

#[gpui::test]
fn keyboard_input_reveals_caret_at_bottom_of_fixed_viewport(cx: &mut TestAppContext) {
    let window = cx.update(|cx| {
        cx.open_window(Default::default(), |_, cx| {
            cx.new(|cx| {
                let value = (1..=200)
                    .map(|line| line.to_string())
                    .collect::<Vec<_>>()
                    .join("\n");
                TextEditor::with_value(value, cx)
            })
        })
        .unwrap()
    });
    let mut cx = VisualTestContext::from_window(window.into(), cx);
    let editor = window.root(&mut cx).unwrap();
    editor.update_in(&mut cx, |editor, window, cx| {
        editor.replace_text_in_range(None, "x", window, cx);
    });
    assert!(editor.read_with(&cx, |editor, _| editor.scroll_offset().y) < px(0.));

    let focus = editor.read_with(&cx, |editor, cx| editor.focus_handle(cx));
    cx.update(|window, cx| focus.dispatch_action(&TextEditorDocumentStart, window, cx));
    assert_eq!(
        editor.read_with(&cx, |editor, _| editor.scroll_offset().y),
        px(0.)
    );
}
