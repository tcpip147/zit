use gpui::{
    AppContext, Focusable, Modifiers, MouseButton, TestAppContext, VisualTestContext, point, px,
};
use widget::TextInput;

#[gpui::test]
fn rendered_input_exposes_valid_initial_cursor_for_pointer_hit_testing(cx: &mut TestAppContext) {
    let window = cx.update(|cx| {
        cx.open_window(Default::default(), |_, cx| {
            cx.new(|cx| TextInput::with_value("pointer target", cx))
        })
        .unwrap()
    });
    let mut cx = VisualTestContext::from_window(window.into(), cx);
    let input = window.root(&mut cx).unwrap();
    let range = input.read_with(&cx, |input, _| input.selection());
    assert!(range.start <= "pointer target".len());
    assert_eq!(range.start, range.end);
}

#[gpui::test]
fn pointer_focus_shows_caret_immediately_at_clicked_boundary(cx: &mut TestAppContext) {
    let window = cx.update(|cx| {
        cx.open_window(Default::default(), |_, cx| {
            cx.new(|cx| TextInput::with_value("pointer target", cx))
        })
        .unwrap()
    });
    let mut cx = VisualTestContext::from_window(window.into(), cx);
    let input = window.root(&mut cx).unwrap();
    let focus = input.read_with(&cx, |input, cx| input.focus_handle(cx));

    input.update_in(&mut cx, |_, _, cx| cx.notify());
    cx.run_until_parked();
    cx.update(|window, _| window.activate_window());
    assert!(!cx.update(|window, _| focus.is_focused(window)));

    cx.simulate_mouse_down(
        point(px(12.), px(12.)),
        MouseButton::Left,
        Modifiers::default(),
    );

    assert!(cx.update(|window, _| focus.is_focused(window)));
    assert!(input.read_with(&cx, |input, _| input.is_caret_visible()));
    let selection = input.read_with(&cx, |input, _| input.selection());
    assert!(selection.is_empty());
    assert!(selection.start < "pointer target".len());
}

#[gpui::test]
fn disabled_pointer_press_does_not_focus_or_show_caret(cx: &mut TestAppContext) {
    let window = cx.update(|cx| {
        cx.open_window(Default::default(), |window, cx| {
            cx.new(|cx| {
                let mut input = TextInput::with_value("disabled", cx);
                input.set_enabled(false, window, cx);
                input
            })
        })
        .unwrap()
    });
    let mut cx = VisualTestContext::from_window(window.into(), cx);
    let input = window.root(&mut cx).unwrap();
    let focus = input.read_with(&cx, |input, cx| input.focus_handle(cx));

    input.update_in(&mut cx, |_, _, cx| cx.notify());
    cx.run_until_parked();
    cx.update(|window, _| window.activate_window());
    cx.simulate_mouse_down(
        point(px(12.), px(12.)),
        MouseButton::Left,
        Modifiers::default(),
    );

    assert!(!cx.update(|window, _| focus.is_focused(window)));
    assert!(!input.read_with(&cx, |input, _| input.is_caret_visible()));
    assert_eq!(input.read_with(&cx, |input, _| input.selection()), 8..8);
}
