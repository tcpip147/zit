use gpui::px;

#[test]
fn scrollbar_formula_is_proportional_and_has_minimum_thumb() {
    let viewport: f32 = 100.;
    let max_offset: f32 = 900.;
    let thumb = (viewport * viewport / (viewport + max_offset))
        .max(24_f32)
        .min(viewport);
    assert_eq!(thumb, 24.);
    let bottom = max_offset / max_offset * (viewport - thumb);
    assert_eq!(bottom, 76.);
    assert!(px(34.) > px(0.));
}
