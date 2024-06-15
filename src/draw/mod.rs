pub mod point;
pub mod rect;
pub mod text_box;

pub use point::*;
pub use rect::*;
pub use text_box::TextBox;

use crate::color::Color;

use lazy_static::lazy_static;
use rusttype::{Font, Scale};

pub static FONT_DATA: &[u8] = include_bytes!("../../fonts/FiraCodeNerdFontMono-Regular.ttf");

lazy_static! {
    pub static ref FONT: Font<'static> =
        Font::try_from_bytes(FONT_DATA as &[u8]).expect("error constructing FiraCodeMono");
}

pub fn font_width(scale: Scale) -> f32 {
    FONT.glyph('0').scaled(scale).h_metrics().advance_width
}

// which edge to align to
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub enum Align {
    Start,
    #[default]
    Center,
    End,
}

use smithay_client_toolkit::shm::slot::Buffer;
pub struct DrawCtx<'ctx> {
    pub damage: &'ctx mut Vec<Rect<u32>>,
    pub buffer: &'ctx Buffer,
    pub canvas: &'ctx mut [u8],
    pub rect: Rect<u32>,
    pub full_redraw: bool,
}

impl DrawCtx<'_> {
    pub fn put(&mut self, pnt: Point<u32>, color: Color) {
        debug_assert!(self.rect.contains(pnt));

        let idx: usize = 4 * (pnt.x + pnt.y * self.rect.width()) as usize;

        let array: &mut [u8; 4] = (&mut self.canvas[idx..idx + 4]).try_into().unwrap();
        *array = color.argb8888();
    }
}