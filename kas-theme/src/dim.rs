// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE-APACHE file or at:
//     https://www.apache.org/licenses/LICENSE-2.0

//! Widget styling
//!
//! Widget size and appearance can be modified through themes.

use std::any::Any;
use std::f32;

use kas::draw::{self, TextClass};
use kas::geom::{Size, Vec2};
use kas::layout::{AxisInfo, Margins, SizeRules, StretchPolicy};
use kas::text::{FontId, PreparedText};

/// Parameterisation of [`Dimensions`]
///
/// All dimensions are multiplied by the DPI factor, then rounded to the
/// nearest integer. Example: `(2.0 * 1.25).round() = 3.0`.
#[derive(Clone, Debug)]
pub struct DimensionsParams {
    /// Inner margin
    pub margin: f32,
    /// Frame size
    pub frame_size: f32,
    /// Button frame size (non-flat outer region)
    pub button_frame: f32,
    /// Scrollbar minimum handle size
    pub scrollbar_size: Vec2,
    /// Slider minimum handle size
    pub slider_size: Vec2,
}

/// Dimensions available within [`DimensionsWindow`]
#[derive(Clone, Debug)]
pub struct Dimensions {
    pub font_id: FontId,
    pub font_scale: f32,
    pub font_marker_width: f32,
    pub scale_factor: f32,
    pub line_height: u32,
    pub min_line_length: u32,
    pub ideal_line_length: u32,
    pub margin: u32,
    pub frame: u32,
    pub button_frame: u32,
    pub checkbox: u32,
    pub scrollbar: Size,
    pub slider: Size,
}

impl Dimensions {
    pub fn new(
        params: DimensionsParams,
        font_id: FontId,
        font_size: f32,
        scale_factor: f32,
    ) -> Self {
        let font_scale = font_size * scale_factor;
        let line_height = font_scale.round() as u32;
        let margin = (params.margin * scale_factor).round() as u32;
        let frame = (params.frame_size * scale_factor).round() as u32;
        Dimensions {
            font_id,
            font_scale,
            font_marker_width: (font_size * (1.0 / 9.0)).round().max(1.0),
            scale_factor,
            line_height,
            // We appear to average about 2 characters per line_height
            // TODO: better to specify in terms of font's 'n' size?
            min_line_length: line_height * 6,
            ideal_line_length: line_height * 15,
            margin,
            frame,
            button_frame: (params.button_frame * scale_factor).round() as u32,
            checkbox: (font_scale * 0.7).round() as u32 + 2 * (margin + frame),
            scrollbar: Size::from(params.scrollbar_size * scale_factor),
            slider: Size::from(params.slider_size * scale_factor),
        }
    }

    pub fn edit_marker_size(&self) -> Vec2 {
        Vec2(self.font_marker_width, self.font_scale)
    }
}

/// A convenient implementation of [`crate::Window`]
pub struct DimensionsWindow {
    pub dims: Dimensions,
}

impl DimensionsWindow {
    pub fn new(dims: DimensionsParams, font_id: FontId, font_size: f32, scale_factor: f32) -> Self {
        DimensionsWindow {
            dims: Dimensions::new(dims, font_id, font_size, scale_factor),
        }
    }
}

impl crate::Window for DimensionsWindow {
    #[cfg(not(feature = "gat"))]
    type SizeHandle = SizeHandle<'static>;
    #[cfg(feature = "gat")]
    type SizeHandle<'a> = SizeHandle<'a>;

    #[cfg(not(feature = "gat"))]
    unsafe fn size_handle<'a>(&'a mut self) -> Self::SizeHandle {
        // We extend lifetimes (unsafe) due to the lack of associated type generics.
        let h: SizeHandle<'a> = SizeHandle::new(&self.dims);
        std::mem::transmute(h)
    }
    #[cfg(feature = "gat")]
    fn size_handle<'a>(&'a mut self) -> Self::SizeHandle<'a> {
        SizeHandle::new(&self.dims)
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct SizeHandle<'a> {
    dims: &'a Dimensions,
}

impl<'a> SizeHandle<'a> {
    pub fn new(dims: &'a Dimensions) -> Self {
        SizeHandle { dims }
    }
}

impl<'a> draw::SizeHandle for SizeHandle<'a> {
    fn scale_factor(&self) -> f32 {
        self.dims.scale_factor
    }

    fn frame(&self) -> Size {
        let f = self.dims.frame as u32;
        Size::uniform(f)
    }
    fn menu_frame(&self) -> Size {
        let f = self.dims.frame as u32;
        Size(f, f / 2)
    }

    fn inner_margin(&self) -> Size {
        Size::uniform(self.dims.margin as u32)
    }

    fn outer_margins(&self) -> Margins {
        Margins::uniform(self.dims.margin as u16)
    }

    fn line_height(&self, _: TextClass) -> u32 {
        self.dims.line_height
    }

    fn prepare(&mut self, text: &mut PreparedText, _class: TextClass) {
        text.prepare(Vec2::INFINITY, self.dims.font_scale.into());
    }

    fn text_bound(
        &mut self,
        text: &mut PreparedText,
        class: TextClass,
        axis: AxisInfo,
    ) -> SizeRules {
        let line_height = self.dims.line_height;
        let mut bounds = Vec2::INFINITY;
        if let Some(size) = axis.size_other_if_fixed(false) {
            bounds.1 = size as f32;
        } else if let Some(size) = axis.size_other_if_fixed(true) {
            bounds.0 = size as f32;
        }
        text.prepare(bounds, self.dims.font_scale.into());
        let bounds = text.required_size();

        let margins = (self.dims.margin as u16, self.dims.margin as u16);
        if axis.is_horizontal() {
            let bound = bounds.0 as u32;
            let min = self.dims.min_line_length;
            let ideal = self.dims.ideal_line_length;
            let (min, ideal) = match class {
                TextClass::Edit | TextClass::EditMulti => (min, ideal),
                _ => (bound.min(min), bound.min(ideal)),
            };
            SizeRules::new(min, ideal, margins, StretchPolicy::LowUtility)
        } else {
            let min = match class {
                TextClass::EditMulti => line_height * 3,
                _ => line_height,
            };
            let ideal = (bounds.1 as u32).max(min);
            let stretch = match class {
                TextClass::Button | TextClass::Edit => StretchPolicy::Fixed,
                _ => StretchPolicy::Filler,
            };
            SizeRules::new(min, ideal, margins, stretch)
        }
    }

    fn button_surround(&self) -> (Size, Size) {
        let s = Size::uniform(self.dims.button_frame);
        (s, s)
    }

    fn edit_surround(&self) -> (Size, Size) {
        let s = Size::uniform(self.dims.frame as u32);
        (s, s)
    }

    fn checkbox(&self) -> Size {
        Size::uniform(self.dims.checkbox)
    }

    #[inline]
    fn radiobox(&self) -> Size {
        self.checkbox()
    }

    fn scrollbar(&self) -> (Size, u32) {
        let size = self.dims.scrollbar;
        (size, 2 * size.0)
    }

    fn slider(&self) -> (Size, u32) {
        let size = self.dims.slider;
        (size, 2 * size.0)
    }
}
