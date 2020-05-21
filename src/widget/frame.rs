// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE-APACHE file or at:
//     https://www.apache.org/licenses/LICENSE-2.0

//! A simple frame

use kas::class::*;
use kas::prelude::*;

/// A frame around content
///
/// This widget provides a simple abstraction: drawing a frame around its
/// contents.
#[handler(msg = <W as Handler>::Msg)]
#[derive(Clone, Debug, Default, Widget)]
pub struct Frame<W: Widget> {
    #[widget_core]
    core: CoreData,
    /// The inner widget
    #[widget]
    pub inner: W,
    m0: Size,
    m1: Size,
}

impl<W: Widget> Frame<W> {
    /// Construct a frame
    #[inline]
    pub fn new(inner: W) -> Self {
        Frame {
            core: Default::default(),
            inner,
            m0: Size::ZERO,
            m1: Size::ZERO,
        }
    }
}

impl<W: Widget> Layout for Frame<W> {
    fn size_rules(&mut self, size_handle: &mut dyn SizeHandle, axis: AxisInfo) -> SizeRules {
        let size = size_handle.frame();
        let margins = Margins::ZERO;
        let frame_rules = SizeRules::extract_fixed(axis.is_vertical(), size + size, margins);

        let inner_rules = self.inner.size_rules(size_handle, axis);
        let m = inner_rules.margins();

        if axis.is_horizontal() {
            self.m0.0 = size.0 + m.0 as u32;
            self.m1.0 = size.0 + m.1 as u32;
        } else {
            self.m0.1 = size.1 + m.0 as u32;
            self.m1.1 = size.1 + m.1 as u32;
        }

        inner_rules.surrounded_by(frame_rules, true)
    }

    fn set_rect(&mut self, mut rect: Rect, align: AlignHints) {
        self.core.rect = rect;
        rect.pos += self.m0;
        rect.size = rect.size.saturating_sub(self.m0 + self.m1);
        self.inner.set_rect(rect, align);
    }

    #[inline]
    fn find_id(&self, coord: Coord) -> Option<WidgetId> {
        if !self.rect().contains(coord) {
            return None;
        }
        self.inner.find_id(coord).or(Some(self.id()))
    }

    fn draw(&self, draw_handle: &mut dyn DrawHandle, mgr: &event::ManagerState, disabled: bool) {
        draw_handle.outer_frame(self.core_data().rect);
        let disabled = disabled || self.is_disabled();
        self.inner.draw(draw_handle, mgr, disabled);
    }
}

impl<W: HasBool + Widget> HasBool for Frame<W> {
    fn get_bool(&self) -> bool {
        self.inner.get_bool()
    }

    fn set_bool(&mut self, state: bool) -> TkAction {
        self.inner.set_bool(state)
    }
}

impl<W: HasText + Widget> HasText for Frame<W> {
    fn get_text(&self) -> &str {
        self.inner.get_text()
    }

    fn set_cow_string(&mut self, text: CowString) -> TkAction {
        self.inner.set_cow_string(text)
    }
}

impl<W: Editable + Widget> Editable for Frame<W> {
    fn is_editable(&self) -> bool {
        self.inner.is_editable()
    }

    fn set_editable(&mut self, editable: bool) {
        self.inner.set_editable(editable);
    }
}
