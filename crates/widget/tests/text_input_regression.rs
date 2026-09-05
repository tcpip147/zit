use gpui::{AppContext, EntityInputHandler, TestAppContext, VisualTestContext};
use widget::TextInput;

#[gpui::test]
fn mixed_script_and_emoji_input_remains_intact(cx: &mut TestAppContext) {
    let window = cx.update(|cx| {
        cx.open_window(Default::default(), |_, cx| cx.new(TextInput::new))
            .unwrap()
    });
    let mut cx = VisualTestContext::from_window(window.into(), cx);
    let input = window.root(&mut cx).unwrap();
    let value = "English 한글 العربية 👨‍👩‍👧‍👦 e\u{301}";
    input.update_in(&mut cx, |input, window, cx| {
        input.replace_text_in_range(None, value, window, cx)
    });
    assert_eq!(
        input.read_with(&cx, |input, _| input.value().to_owned()),
        value
    );
}
