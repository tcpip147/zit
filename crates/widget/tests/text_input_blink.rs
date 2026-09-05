use std::time::Duration;

use gpui::{
    AppContext, EntityInputHandler, Focusable, Modifiers, MouseButton, TestAppContext,
    VisualTestContext, point, px,
};
use widget::{Left, TextInput};

#[gpui::test]
fn caret_blinks_and_interaction_restarts_visible_phase(cx: &mut TestAppContext) {
    let window = cx.update(|cx| {
        cx.open_window(Default::default(), |_, cx| {
            cx.new(|cx| TextInput::with_value("blink", cx))
        })
        .unwrap()
    });
    let mut cx = VisualTestContext::from_window(window.into(), cx);
    let input = window.root(&mut cx).unwrap();
    let focus = input.read_with(&cx, |input, cx| input.focus_handle(cx));

    input.update_in(&mut cx, |_, _, cx| cx.notify());
    cx.run_until_parked();
    cx.update(|window, cx| {
        window.activate_window();
        focus.focus(window, cx);
    });
    cx.executor().advance_clock(Duration::ZERO);
    assert!(input.read_with(&cx, |input, _| input.is_caret_visible()));

    cx.executor().advance_clock(Duration::from_millis(500));
    assert!(!input.read_with(&cx, |input, _| input.is_caret_visible()));

    cx.update(|window, cx| focus.dispatch_action(&Left, window, cx));
    assert!(input.read_with(&cx, |input, _| input.is_caret_visible()));

    cx.executor().advance_clock(Duration::from_millis(500));
    assert!(!input.read_with(&cx, |input, _| input.is_caret_visible()));
}

#[gpui::test]
fn selection_blur_and_disabled_state_stop_blinking(cx: &mut TestAppContext) {
    let window = cx.update(|cx| {
        cx.open_window(Default::default(), |_, cx| {
            cx.new(|cx| TextInput::with_value("state", cx))
        })
        .unwrap()
    });
    let mut cx = VisualTestContext::from_window(window.into(), cx);
    let input = window.root(&mut cx).unwrap();
    let focus = input.read_with(&cx, |input, cx| input.focus_handle(cx));
    input.update_in(&mut cx, |_, _, cx| cx.notify());
    cx.run_until_parked();
    cx.update(|window, cx| {
        window.activate_window();
        focus.focus(window, cx);
    });
    cx.executor().advance_clock(Duration::ZERO);

    input.update_in(&mut cx, |input, window, cx| {
        input.replace_and_mark_text_in_range(None, "선택", Some(0..1), window, cx);
    });
    assert!(!input.read_with(&cx, |input, _| input.is_caret_visible()));

    input.update_in(&mut cx, |input, window, cx| {
        input.replace_text_in_range(None, "선택", window, cx);
        input.set_enabled(false, window, cx);
    });
    assert!(!input.read_with(&cx, |input, _| input.is_caret_visible()));
    cx.executor().advance_clock(Duration::from_secs(2));
    assert!(!input.read_with(&cx, |input, _| input.is_caret_visible()));
}

#[gpui::test]
fn inputs_keep_independent_blink_state(cx: &mut TestAppContext) {
    let first = cx.new(TextInput::new);
    let second = cx.new(TextInput::new);
    assert!(!first.read_with(cx, |input, _| input.is_caret_visible()));
    assert!(!second.read_with(cx, |input, _| input.is_caret_visible()));
}

#[gpui::test]
fn pointer_focus_starts_a_fresh_blink_cycle_without_stale_toggle(cx: &mut TestAppContext) {
    let window = cx.update(|cx| {
        cx.open_window(Default::default(), |_, cx| {
            cx.new(|cx| TextInput::with_value("blink", cx))
        })
        .unwrap()
    });
    let mut cx = VisualTestContext::from_window(window.into(), cx);
    let input = window.root(&mut cx).unwrap();

    input.update_in(&mut cx, |_, _, cx| cx.notify());
    cx.run_until_parked();
    cx.update(|window, _| window.activate_window());
    cx.simulate_mouse_down(
        point(px(12.), px(12.)),
        MouseButton::Left,
        Modifiers::default(),
    );

    assert!(input.read_with(&cx, |input, _| input.is_caret_visible()));
    cx.executor().advance_clock(Duration::from_millis(499));
    assert!(input.read_with(&cx, |input, _| input.is_caret_visible()));
    cx.executor().advance_clock(Duration::from_millis(1));
    assert!(!input.read_with(&cx, |input, _| input.is_caret_visible()));
}
