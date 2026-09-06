use widget::{Theme, ThemeColor};

#[test]
fn partial_json_inherits_defaults() {
    let theme = Theme::from_json(r##"{"tab_bar":{"text":"#112233"}}"##).unwrap();
    assert_eq!(theme.tab_bar.text, ThemeColor::rgb(0x112233));
    assert_eq!(
        theme.text_input.background,
        Theme::default().text_input.background
    );
}

#[test]
fn accepts_alpha_and_rejects_invalid_colors() {
    let theme =
        Theme::from_json(r##"{"text_input":{"selection_background":"#11223344"}}"##).unwrap();
    assert_eq!(
        theme.text_input.selection_background,
        ThemeColor::rgba(0x11223344)
    );
    assert!(Theme::from_json(r##"{"tab_bar":{"text":"red"}}"##).is_err());
}

#[test]
fn data_grid_selection_background_is_configurable() {
    let theme =
        Theme::from_json(r##"{"data_grid":{"selected_row_background":"#123456"}}"##).unwrap();
    assert_eq!(
        theme.data_grid.selected_row_background,
        ThemeColor::rgb(0x123456)
    );
}
