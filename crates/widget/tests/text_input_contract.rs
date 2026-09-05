use gpui::{AppContext, TestAppContext};
use widget::TextInput;

#[gpui::test]
fn public_configuration_is_independent_per_instance(cx: &mut TestAppContext) {
    let first = cx.new(TextInput::new);
    let second = cx.new(|cx| TextInput::with_value("second", cx));
    first.update(cx, |input, cx| {
        input.set_placeholder("first hint", cx);
        input.set_value("first", cx);
    });
    assert_eq!(
        first.read_with(cx, |input, _| input.value().to_owned()),
        "first"
    );
    assert_eq!(
        second.read_with(cx, |input, _| input.value().to_owned()),
        "second"
    );
    assert_eq!(
        first.read_with(cx, |input, _| input.placeholder().to_owned()),
        "first hint"
    );
}
