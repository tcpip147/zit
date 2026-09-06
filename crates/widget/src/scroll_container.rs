use std::time::Duration;

use gpui::{AnyElement, Context, CursorStyle, InteractiveElement, Render, Role, ScrollHandle,
    ScrollWheelEvent, StatefulInteractiveElement, Task, Window, div, prelude::*, px};

/// A vertically scrollable region with an auto-hiding overlay scrollbar.
pub struct ScrollContainer {
    content: Option<AnyElement>,
    scroll: ScrollHandle,
    visible: bool,
    hovered: bool,
    generation: u64,
    hide_task: Option<Task<()>>,
}

impl ScrollContainer {
    pub fn new(content: impl IntoElement) -> Self {
        Self { content: Some(content.into_any_element()), scroll: ScrollHandle::new(), visible: false,
            hovered: false, generation: 0, hide_task: None }
    }

    pub fn set_content(&mut self, content: impl IntoElement, cx: &mut Context<Self>) {
        self.content = Some(content.into_any_element());
        // The owner updates this entity while it is already rendering. Notifying here
        // would schedule another layout pass on every frame and make the overlay track
        // visibly oscillate while the scroll offset changes.
        let _ = cx;
    }

    fn show_temporarily(&mut self, cx: &mut Context<Self>) {
        self.generation = self.generation.wrapping_add(1);
        let generation = self.generation;
        self.visible = true;
        let timer = cx.background_executor().timer(Duration::from_secs(1));
        self.hide_task = Some(cx.spawn(async move |this, cx| {
            timer.await;
            let _ = this.update(cx, |view, cx| {
                if view.generation == generation && !view.hovered {
                    view.visible = false;
                    view.hide_task = None;
                    cx.notify();
                }
            });
        }));
    }

    fn wheel(&mut self, event: &ScrollWheelEvent, _: &mut Window, cx: &mut Context<Self>) {
        let delta = event.delta.pixel_delta(px(16.)).y;
        let state = self.scroll.offset();
        let max = self.scroll.max_offset().y.max(px(0.));
        self.scroll.set_offset(gpui::point(state.x, (state.y + delta).clamp(-max, px(0.))));
        self.show_temporarily(cx);
        cx.notify();
    }

    fn hover(&mut self, hovered: &bool, _: &mut Window, cx: &mut Context<Self>) {
        self.hovered = *hovered;
        if *hovered { self.visible = true; } else { self.show_temporarily(cx); }
        cx.notify();
    }

    fn scrollbar(&self, cx: &mut Context<Self>) -> AnyElement {
        let bounds = self.scroll.bounds();
        let max = self.scroll.max_offset().y;
        // The track has 2px top/bottom insets, so use its actual height for both
        // thumb size and position. Using the full viewport makes the thumb overshoot
        // and causes a visible jump at the ends of the scroll range.
        let viewport = bounds.size.height.max(px(1.));
        let track_height = (viewport - px(4.)).max(px(1.));
        let thumb = if max > px(0.) {
            (track_height * (viewport / (viewport + max))).max(px(18.)).min(track_height)
        } else {
            track_height
        };
        let offset = (-self.scroll.offset().y).clamp(px(0.), max);
        let top = if max > px(0.) {
            (offset / max * (track_height - thumb).max(px(0.)))
                .clamp(px(0.), (track_height - thumb).max(px(0.)))
        } else {
            px(0.)
        };
        div().id("scroll-container-track").absolute().right(px(2.)).top(px(2.)).bottom(px(2.)).w(px(6.)).rounded(px(3.))
            .bg(gpui::rgb(0xe2e8f0)).on_hover(cx.listener(Self::hover))
            .when(!self.visible, |d| d.opacity(0.).hover(|s| s.opacity(1.)))
            .when(max > px(0.), |d| d.child(div().absolute().top(top).w_full().h(thumb).rounded(px(3.)).bg(gpui::rgb(0x94a3b8)).cursor(CursorStyle::PointingHand)))
            .into_any_element()
    }
}

impl Render for ScrollContainer {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = self.content.take().unwrap_or_else(|| div().into_any_element());
        div().id("scroll-container").role(Role::ScrollBar).size_full().relative()
            .overflow_hidden()
            .on_scroll_wheel(cx.listener(Self::wheel))
            .child(
                div()
                    .id("scroll-container-content")
                    .size_full()
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll)
                    .child(content),
            )
            .child(self.scrollbar(cx))
    }
}
