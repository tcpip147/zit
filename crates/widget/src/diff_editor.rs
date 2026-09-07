use gpui::{
    AnyElement, Context, CursorStyle, InteractiveElement, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, Render, ScrollHandle, ScrollWheelEvent, SharedString,
    StatefulInteractiveElement, Window, canvas, div, prelude::*, px, rgb,
};

const ROW_HEIGHT: f32 = 20.;
const OVERSCAN_ROWS: usize = 12;
const BOTTOM_CONTENT_PADDING: f32 = 10.;

/// A read-only, line-oriented renderer for unified Git diffs.
pub struct DiffEditor {
    lines: Vec<ParsedLine>,
    rendered_diff: SharedString,
    scroll: ScrollHandle,
    horizontal_scroll: ScrollHandle,
    content_width: f32,
    gutter_width: f32,
    drag: Option<(bool, f32, f32)>,
    track_hovered: Option<bool>,
}

#[derive(Clone, Copy)]
enum DiffLineKind {
    Context,
    Added,
    Removed,
    HunkSeparator,
}

#[derive(Clone)]
struct ParsedLine {
    before: Option<usize>,
    after: Option<usize>,
    content: String,
    kind: DiffLineKind,
}

impl DiffEditor {
    pub fn new(_: &mut Context<Self>) -> Self {
        Self {
            lines: Vec::new(),
            rendered_diff: SharedString::default(),
            scroll: ScrollHandle::new(),
            horizontal_scroll: ScrollHandle::new(),
            content_width: 400.,
            gutter_width: 24.,
            drag: None,
            track_hovered: None,
        }
    }

    pub fn set_diff(&mut self, diff: impl Into<SharedString>, cx: &mut Context<Self>) {
        let diff = diff.into();
        if self.rendered_diff == diff {
            return;
        }
        self.rendered_diff = diff.clone();
        let lines = diff
            .lines()
            .scan((None, None, false), |cursor, line| {
                if let Some((before, after)) = parse_hunk(line) {
                    let separator = cursor.2;
                    cursor.0 = Some(before);
                    cursor.1 = Some(after);
                    cursor.2 = true;
                    return Some(separator.then_some(ParsedLine {
                        before: None,
                        after: None,
                        content: String::new(),
                        kind: DiffLineKind::HunkSeparator,
                    }));
                }
                let (before, after, _) = cursor;
                let parsed = match line.as_bytes().first().copied() {
                    Some(b' ') => {
                        let value = ParsedLine {
                            before: *before,
                            after: *after,
                            content: line[1..].to_owned(),
                            kind: DiffLineKind::Context,
                        };
                        *before = before.map(|line| line + 1);
                        *after = after.map(|line| line + 1);
                        value
                    }
                    Some(b'-') if !line.starts_with("---") => {
                        let value = ParsedLine {
                            before: *before,
                            after: None,
                            content: line[1..].to_owned(),
                            kind: DiffLineKind::Removed,
                        };
                        *before = before.map(|line| line + 1);
                        value
                    }
                    Some(b'+') if !line.starts_with("+++") => {
                        let value = ParsedLine {
                            before: None,
                            after: *after,
                            content: line[1..].to_owned(),
                            kind: DiffLineKind::Added,
                        };
                        *after = after.map(|line| line + 1);
                        value
                    }
                    _ => return Some(None),
                };
                Some(Some(parsed))
            })
            .flatten()
            .collect::<Vec<_>>();
        self.lines = lines;
        self.scroll.set_offset(gpui::point(px(0.), px(0.)));
        self.horizontal_scroll
            .set_offset(gpui::point(px(0.), px(0.)));
        self.content_width = self
            .lines
            .iter()
            .map(|line| line.content.chars().count() as f32 * 7.2 + 96.)
            .fold(400., f32::max);
        let max_line_number = self
            .lines
            .iter()
            .flat_map(|line| [line.before, line.after])
            .flatten()
            .max()
            .unwrap_or(0);
        self.gutter_width = (max_line_number.to_string().len() as f32 * 7. + 16.).max(24.);
        cx.notify();
    }
}

fn parse_hunk(line: &str) -> Option<(usize, usize)> {
    let mut parts = line.split_whitespace();
    if parts.next()? != "@@" {
        return None;
    }
    let before = parts
        .next()?
        .strip_prefix('-')?
        .split(',')
        .next()?
        .parse()
        .ok()?;
    let after = parts
        .next()?
        .strip_prefix('+')?
        .split(',')
        .next()?
        .parse()
        .ok()?;
    Some((before, after))
}

fn line_colors(kind: DiffLineKind) -> (gpui::Rgba, gpui::Rgba) {
    match kind {
        DiffLineKind::Added => (rgb(0xdcfce7), rgb(0x166534)),
        DiffLineKind::Removed => (rgb(0xfee2e2), rgb(0x991b1b)),
        DiffLineKind::Context => (rgb(0xffffff), rgb(0x334155)),
        DiffLineKind::HunkSeparator => (rgb(0xffffff), rgb(0x334155)),
    }
}

fn diff_gutter_line(line: ParsedLine, gutter_width: f32) -> gpui::Div {
    if matches!(line.kind, DiffLineKind::HunkSeparator) {
        return div().w_full().h(px(ROW_HEIGHT));
    }
    div()
        .w(px(gutter_width * 2.))
        .h(px(ROW_HEIGHT))
        .flex()
        .bg(line_colors(line.kind).0)
        .child(
            div()
                .w(px(gutter_width))
                .flex_shrink_0()
                .px_2()
                .text_size(px(10.))
                .text_color(rgb(0x94a3b8))
                .child(format_line_number(line.before)),
        )
        .child(
            div()
                .w(px((gutter_width - 6.).max(1.)))
                .flex_shrink_0()
                .pl_2()
                .pr_0()
                .border_r_1()
                .border_color(rgb(0xe2e8f0))
                .text_size(px(10.))
                .text_color(rgb(0x94a3b8))
                .child(format_line_number(line.after)),
        )
}

fn diff_code_line(line: ParsedLine) -> gpui::Div {
    let (background, foreground) = line_colors(line.kind);
    if matches!(line.kind, DiffLineKind::HunkSeparator) {
        return div()
            .w_full()
            .h(px(ROW_HEIGHT))
            .child(div().w_full().h(px(1.)).bg(rgb(0xcbd5e1)));
    }
    div()
        .w_full()
        .h(px(ROW_HEIGHT))
        .flex()
        .font_family("monospace")
        .text_size(px(12.))
        .text_color(foreground)
        .bg(background)
        .child(div().px_2().whitespace_nowrap().child(line.content))
}

fn format_line_number(number: Option<usize>) -> String {
    number.map_or_else(String::new, |number| number.to_string())
}

impl Render for DiffEditor {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let max_offset = self.scroll.max_offset();
        let offset = self.scroll.offset();
        let clamped = gpui::point(
            offset.x.clamp(-max_offset.x, px(0.)),
            offset.y.clamp(-max_offset.y, px(0.)),
        );
        if offset != clamped {
            self.scroll.set_offset(clamped);
        }
        let horizontal_max = self.horizontal_scroll.max_offset();
        let horizontal_offset = self.horizontal_scroll.offset();
        let horizontal_clamped = gpui::point(
            horizontal_offset.x.clamp(-horizontal_max.x, px(0.)),
            horizontal_offset.y,
        );
        if horizontal_offset != horizontal_clamped {
            self.horizontal_scroll.set_offset(horizontal_clamped);
        }
        let viewport_height = self.scroll.bounds().size.height.as_f32().max(ROW_HEIGHT);
        let offset = (-self.scroll.offset().y.as_f32()).max(0.);
        let first = (offset / ROW_HEIGHT).floor() as usize;
        let visible = (viewport_height / ROW_HEIGHT).ceil() as usize;
        let start = first.saturating_sub(OVERSCAN_ROWS).min(self.lines.len());
        let end = (first + visible + OVERSCAN_ROWS).min(self.lines.len());
        let gutter = self.lines[start..end]
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, line)| {
                div()
                    .absolute()
                    .top(px((start + index) as f32 * ROW_HEIGHT))
                    .left(px(0.))
                    .right(px(0.))
                    .h(px(ROW_HEIGHT))
                    .child(diff_gutter_line(line, self.gutter_width))
            })
            .collect::<Vec<_>>();
        let code = self.lines[start..end]
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, line)| {
                div()
                    .absolute()
                    .top(px((start + index) as f32 * ROW_HEIGHT))
                    .left(px(0.))
                    .right(px(0.))
                    .h(px(ROW_HEIGHT))
                    .child(diff_code_line(line))
            })
            .collect::<Vec<_>>();
        let total_height = self.lines.len() as f32 * ROW_HEIGHT + BOTTOM_CONTENT_PADDING;
        div()
            .id("diff-editor")
            .size_full()
            .relative()
            .flex()
            .child(
                div()
                    .id("diff-gutter-viewport")
                    .w(px(self.gutter_width * 2.))
                    .h_full()
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll)
                    .on_scroll_wheel(cx.listener(Self::wheel))
                    .child(
                        div()
                            .relative()
                            .w_full()
                            .h(px(total_height))
                            .children(gutter),
                    ),
            )
            .child(
                div()
                    .id("diff-code-viewport")
                    .flex_1()
                    .h_full()
                    .overflow_scroll()
                    .track_scroll(&self.horizontal_scroll)
                    .on_scroll_wheel(cx.listener(Self::wheel))
                    .child(
                        div()
                            .relative()
                            .min_w(px(self.content_width))
                            .w_full()
                            .h(px(total_height))
                            .min_h(px(total_height))
                            .children(code),
                    ),
            )
            .child(self.capture(cx))
            .child(self.scrollbars(cx))
    }
}

impl DiffEditor {
    fn wheel(&mut self, event: &ScrollWheelEvent, _: &mut Window, cx: &mut Context<Self>) {
        let delta = event.delta.pixel_delta(px(16.)).y;
        let current = self.scroll.offset();
        let max = self.scroll.max_offset().y.max(px(0.));
        let y = (current.y + delta).clamp(-max, px(0.));
        self.scroll.set_offset(gpui::point(current.x, y));
        let horizontal = self.horizontal_scroll.offset();
        self.horizontal_scroll
            .set_offset(gpui::point(horizontal.x, y));
        cx.stop_propagation();
        cx.notify();
    }

    fn scrollbars(&self, cx: &mut Context<Self>) -> gpui::Div {
        let vertical_bounds = self.scroll.bounds();
        let horizontal_bounds = self.horizontal_scroll.bounds();
        let viewport_width = horizontal_bounds.size.width.max(px(1.));
        let viewport_height = vertical_bounds.size.height.max(px(1.));
        let max = self.scroll.max_offset();
        let horizontal_max = self.horizontal_scroll.max_offset();
        let track_width = (viewport_width - px(12.)).max(px(1.));
        let track_height = (viewport_height - px(12.)).max(px(1.));
        let horizontal_thumb = (track_width
            * (viewport_width / (viewport_width + horizontal_max.x)))
            .max(px(24.))
            .min(track_width);
        let vertical_thumb = (track_height * (viewport_height / (viewport_height + max.y)))
            .max(px(24.))
            .min(track_height);
        let horizontal_top = if horizontal_max.x > px(0.) {
            (-self.horizontal_scroll.offset().x / horizontal_max.x
                * (track_width - horizontal_thumb))
                .max(px(0.))
        } else {
            px(0.)
        };
        let vertical_top = if max.y > px(0.) {
            (-self.scroll.offset().y / max.y * (track_height - vertical_thumb)).max(px(0.))
        } else {
            px(0.)
        };

        div()
            .absolute()
            .inset_0()
            .child(
                div()
                    .id("diff-vertical-scroll-track")
                    .absolute()
                    .right(px(2.))
                    .top(px(2.))
                    .bottom(px(10.))
                    .w(px(6.))
                    .rounded(px(3.))
                    .bg(rgb(0xe2e8f0))
                    .on_hover(cx.listener(Self::hover_vertical_track))
                    .on_mouse_down(MouseButton::Left, cx.listener(Self::jump_vertical))
                    .when(max.y > px(0.), |track| {
                        track.child(
                            div()
                                .id("diff-vertical-scroll-thumb")
                                .absolute()
                                .top(vertical_top)
                                .w_full()
                                .h(vertical_thumb)
                                .rounded(px(3.))
                                .bg(
                                    if self.track_hovered == Some(true)
                                        || self.drag.is_some_and(|(vertical, _, _)| vertical)
                                    {
                                        rgb(0x64748b)
                                    } else {
                                        rgb(0x94a3b8)
                                    },
                                )
                                .hover(|style| style.bg(rgb(0x64748b)))
                                .cursor(CursorStyle::PointingHand)
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(Self::start_vertical_drag),
                                )
                                .on_mouse_move(cx.listener(Self::drag))
                                .on_mouse_up(MouseButton::Left, cx.listener(Self::stop_drag))
                                .on_mouse_up_out(MouseButton::Left, cx.listener(Self::stop_drag)),
                        )
                    }),
            )
            .child(
                div()
                    .id("diff-horizontal-scroll-track")
                    .absolute()
                    .left(px(self.gutter_width * 2. + 2.))
                    .right(px(10.))
                    .bottom(px(2.))
                    .h(px(6.))
                    .rounded(px(3.))
                    .bg(rgb(0xe2e8f0))
                    .on_hover(cx.listener(Self::hover_horizontal_track))
                    .on_mouse_down(MouseButton::Left, cx.listener(Self::jump_horizontal))
                    .when(horizontal_max.x > px(0.), |track| {
                        track.child(
                            div()
                                .id("diff-horizontal-scroll-thumb")
                                .absolute()
                                .left(horizontal_top)
                                .h_full()
                                .w(horizontal_thumb)
                                .rounded(px(3.))
                                .bg(
                                    if self.track_hovered == Some(false)
                                        || self.drag.is_some_and(|(vertical, _, _)| !vertical)
                                    {
                                        rgb(0x64748b)
                                    } else {
                                        rgb(0x94a3b8)
                                    },
                                )
                                .hover(|style| style.bg(rgb(0x64748b)))
                                .cursor(CursorStyle::PointingHand)
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(Self::start_horizontal_drag),
                                )
                                .on_mouse_move(cx.listener(Self::drag))
                                .on_mouse_up(MouseButton::Left, cx.listener(Self::stop_drag))
                                .on_mouse_up_out(MouseButton::Left, cx.listener(Self::stop_drag)),
                        )
                    }),
            )
    }

    fn start_vertical_drag(
        &mut self,
        event: &MouseDownEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.drag = Some((
            true,
            event.position.y.as_f32(),
            self.scroll.offset().y.as_f32(),
        ));
        cx.stop_propagation();
    }

    fn start_horizontal_drag(
        &mut self,
        event: &MouseDownEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.drag = Some((
            false,
            event.position.x.as_f32(),
            self.horizontal_scroll.offset().x.as_f32(),
        ));
        cx.stop_propagation();
    }

    fn jump_vertical(&mut self, event: &MouseDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        let bounds = self.scroll.bounds();
        let max = self.scroll.max_offset().y.max(px(0.));
        if max <= px(0.) {
            return;
        }
        let track = (bounds.size.height - px(12.)).max(px(1.));
        let thumb = (track * (bounds.size.height / (bounds.size.height + max)))
            .max(px(24.))
            .min(track);
        let position = (event.position.y - bounds.origin.y - px(2.) - thumb / 2.)
            .clamp(px(0.), (track - thumb).max(px(0.)));
        let offset = -(position / (track - thumb).max(px(1.)) * max);
        self.scroll
            .set_offset(gpui::point(self.scroll.offset().x, offset));
        self.horizontal_scroll
            .set_offset(gpui::point(self.horizontal_scroll.offset().x, offset));
        cx.notify();
    }

    fn jump_horizontal(&mut self, event: &MouseDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        let bounds = self.horizontal_scroll.bounds();
        let max = self.horizontal_scroll.max_offset().x.max(px(0.));
        if max <= px(0.) {
            return;
        }
        let track = (bounds.size.width - px(12.)).max(px(1.));
        let thumb = (track * (bounds.size.width / (bounds.size.width + max)))
            .max(px(48.))
            .min(track);
        let position = (event.position.x - bounds.origin.x - px(2.) - thumb / 2.)
            .clamp(px(0.), (track - thumb).max(px(0.)));
        let offset = -(position / (track - thumb).max(px(1.)) * max);
        self.horizontal_scroll
            .set_offset(gpui::point(offset, self.horizontal_scroll.offset().y));
        cx.notify();
    }

    fn drag(&mut self, event: &MouseMoveEvent, _: &mut Window, cx: &mut Context<Self>) {
        let Some((vertical, start, offset)) = self.drag else {
            return;
        };
        let bounds = if vertical {
            self.scroll.bounds()
        } else {
            self.horizontal_scroll.bounds()
        };
        let max = if vertical {
            self.scroll.max_offset()
        } else {
            self.horizontal_scroll.max_offset()
        };
        let viewport = if vertical {
            bounds.size.height
        } else {
            bounds.size.width
        };
        let content = if vertical {
            bounds.size.height + max.y
        } else {
            bounds.size.width + max.x
        };
        let track = (viewport - px(12.)).max(px(1.));
        let minimum_thumb = if vertical { px(24.) } else { px(48.) };
        let thumb = (track * (viewport / content)).max(minimum_thumb).min(track);
        let travel = (track - thumb).max(px(1.)).as_f32();
        let delta = if vertical {
            event.position.y.as_f32() - start
        } else {
            event.position.x.as_f32() - start
        };
        let value = if vertical {
            (offset - delta * max.y.as_f32() / travel).clamp(-max.y.as_f32(), 0.)
        } else {
            (offset - delta * max.x.as_f32() / travel).clamp(-max.x.as_f32(), 0.)
        };
        if vertical {
            let current = self.scroll.offset();
            self.scroll.set_offset(gpui::point(current.x, px(value)));
            let horizontal = self.horizontal_scroll.offset();
            self.horizontal_scroll
                .set_offset(gpui::point(horizontal.x, px(value)));
        } else {
            let current = self.horizontal_scroll.offset();
            self.horizontal_scroll
                .set_offset(gpui::point(px(value), current.y));
        }
        cx.notify();
    }

    fn stop_drag(&mut self, _: &MouseUpEvent, _: &mut Window, _: &mut Context<Self>) {
        self.drag = None;
    }

    fn hover_vertical_track(&mut self, hovered: &bool, _: &mut Window, cx: &mut Context<Self>) {
        self.track_hovered = if *hovered { Some(true) } else { None };
        cx.notify();
    }

    fn hover_horizontal_track(&mut self, hovered: &bool, _: &mut Window, cx: &mut Context<Self>) {
        self.track_hovered = if *hovered { Some(false) } else { None };
        cx.notify();
    }

    fn capture(&self, cx: &mut Context<Self>) -> AnyElement {
        let editor = cx.entity().downgrade();
        canvas(
            |_, _, _| (),
            move |_, _, window, _| {
                let move_editor = editor.clone();
                window.on_mouse_event(move |event: &MouseMoveEvent, _, window, cx| {
                    let _ = move_editor.update(cx, |editor, cx| editor.drag(event, window, cx));
                });
                let up_editor = editor.clone();
                window.on_mouse_event(move |event: &MouseUpEvent, _, window, cx| {
                    if event.button == MouseButton::Left {
                        let _ =
                            up_editor.update(cx, |editor, cx| editor.stop_drag(event, window, cx));
                    }
                });
            },
        )
        .absolute()
        .inset_0()
        .into_any_element()
    }
}
