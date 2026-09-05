use gpui::{AppContext, TestAppContext};
use widget::TextInput;

#[gpui::test]
fn construction_normalizes_value_and_starts_enabled(cx: &mut TestAppContext) {
    let input = cx.new(|cx| TextInput::with_value("first\nsecond", cx));
    input.read_with(cx, |input, _| {
        assert_eq!(input.value(), "first second");
        assert!(input.is_enabled());
        assert_eq!(input.selection(), 12..12);
    });
}

#[gpui::test]
fn programmatic_value_change_resets_selection(cx: &mut TestAppContext) {
    let input = cx.new(TextInput::new);
    input.update(cx, |input, cx| input.set_value("é😀", cx));
    input.read_with(cx, |input, _| {
        assert_eq!(input.value(), "é😀");
        assert_eq!(input.selection(), "é😀".len().."é😀".len());
    });
}
