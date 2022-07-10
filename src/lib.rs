//! # Embedded Graphics FrameBuffer
//!
//! [embedded-graphics-framebuf](https://crates.io/crates/embedded-graphics-framebuf) is a
//! generalized frame buffer implementation for use with Rust's
//! [`embedded-graphics`](https://crates.io/crates/embedded-graphics) library.
//!
//! The framebuffer approach helps to deal with display flickering when you
//! update multiple parts of the display in separate operations. Instead, with
//! this approach, you're going to write to a in-memory display and push it all
//! at once into your hardware display when the whole picture is drawn.
//!
//! This technique is useful when you're updating large portions of screen
//! or just simply don't want to deal with partial display updates.
//! The downside is a higher RAM consumption for to the framebuffer.
//!
//! The approach has been tested on TTGO (esp32) with ST7789
//!
//! ## Usage example
//!
//! ```rust
//! use embedded_graphics::{
//!     draw_target::DrawTarget,
//!     mock_display::MockDisplay,
//!     pixelcolor::BinaryColor,
//!     prelude::{Point, Primitive},
//!     primitives::{Line, PrimitiveStyle},
//!     Drawable,
//! };
//! use embedded_graphics_framebuf::FrameBuf;
//! let mut data = [BinaryColor::Off; 12 * 11];
//! let mut fbuf = FrameBuf::new(&mut data, 12, 11);
//!
//! let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
//! Line::new(Point::new(2, 2), Point::new(10, 2))
//!     .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
//!     .draw(&mut fbuf)
//!     .unwrap();
//! display.draw_iter(fbuf.into_iter()).unwrap();
//! ```

#![no_std]
use embedded_dma::ReadBuffer;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::OriginDimensions,
    prelude::{PixelColor, Point, Size},
    Pixel,
};

pub mod backends;
use backends::{DMACapableFrameBufferBackend, FrameBufferBackend};

/// Constructs a frame buffer in memory. Lets you define the width(`X`), height
/// (`Y`) and pixel type your using in your display (RGB, Monochrome etc.)
///
/// # Example
/// ```
/// use embedded_graphics::{
///     mono_font::{ascii::FONT_10X20, MonoTextStyle},
///     pixelcolor::Rgb565,
///     prelude::*,
///     text::Text,
/// };
/// use embedded_graphics_framebuf::FrameBuf;
///
/// // Create a framebuffer for a 16-Bit 240x135px display
/// let mut data = [Rgb565::BLACK; 240 * 135];
/// let mut fbuff = FrameBuf::new(&mut data, 240, 135);
///
/// // write "Good luck" into the framebuffer.
/// Text::new(
///     &"Good luck!",
///     Point::new(10, 13),
///     MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE.into()),
/// )
/// .draw(&mut fbuff)
/// .unwrap();
/// ```
// TODO: Once https://github.com/rust-lang/rust/issues/76560 is resolved, change this to `pub struct
// FrameBuf<C: PixelColor, const X: usize, const Y: usize>(pub [C; X * Y]);`
pub struct FrameBuf<C: PixelColor, B: FrameBufferBackend<Color = C>> {
    pub data: B,
    width: usize,
    height: usize,
}

impl<C: PixelColor, B: FrameBufferBackend<Color = C>> FrameBuf<C, B> {
    /// Create a new [`FrameBuf`] on top of an existing memory slice.
    ///
    /// # Panic
    /// Panics if the size of the memory does not match the given width and
    /// height.
    ///
    /// # Example
    /// ```rust
    /// use embedded_graphics::{pixelcolor::Rgb565, prelude::RgbColor};
    /// use embedded_graphics_framebuf::FrameBuf;
    /// let mut data = [Rgb565::BLACK; 240 * 135];
    /// let mut fbuff = FrameBuf::new(&mut data, 240, 135);
    /// ```
    pub fn new(data: B, width: usize, height: usize) -> Self {
        assert_eq!(
            data.nr_elements(),
            width * height,
            "FrameBuf underlying data size does not match width ({}) * height ({}) = {} but is {}",
            width,
            height,
            width * height,
            data.nr_elements(),
        );
        Self {
            data,
            width,
            height,
        }
    }

    /// Get the framebuffers width.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get the framebuffers height.
    pub fn height(&self) -> usize {
        self.height
    }

    fn point_to_index(&self, p: Point) -> usize {
        self.width * p.y as usize + p.x as usize
    }

    /// Set a pixel's color.
    pub fn set_color_at(&mut self, p: Point, color: C) {
        self.data.set(self.point_to_index(p), color)
    }

    /// Get a pixel's color.
    pub fn get_color_at(&self, p: Point) -> C {
        self.data.get(self.point_to_index(p))
    }
}
impl<C: PixelColor + Default, B: FrameBufferBackend<Color = C>> FrameBuf<C, B> {
    pub fn reset(&mut self) {
        self.clear(C::default()).unwrap();
    }
}

impl<'a, C: PixelColor, B: FrameBufferBackend<Color = C>> IntoIterator for &'a FrameBuf<C, B> {
    type Item = Pixel<C>;
    type IntoIter = PixelIterator<'a, C, B>;

    /// Creates an iterator over all [Pixels](Pixel) in the frame buffer. Can be
    /// used for rendering the framebuffer to the physical display.
    ///
    /// # Example
    /// ```rust
    /// use embedded_graphics::{
    ///     draw_target::DrawTarget,
    ///     mock_display::MockDisplay,
    ///     pixelcolor::BinaryColor,
    ///     prelude::{Point, Primitive},
    ///     primitives::{Line, PrimitiveStyle},
    ///     Drawable,
    /// };
    /// use embedded_graphics_framebuf::FrameBuf;
    /// let mut data = [BinaryColor::Off; 12 * 11];
    /// let mut fbuf = FrameBuf::new(&mut data, 12, 11);
    /// let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
    /// Line::new(Point::new(2, 2), Point::new(10, 2))
    ///     .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
    ///     .draw(&mut fbuf)
    ///     .unwrap();
    /// display.draw_iter(fbuf.into_iter()).unwrap();
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        PixelIterator {
            fbuf: self,
            index: 0,
        }
    }
}

impl<C: PixelColor, B: FrameBufferBackend<Color = C>> OriginDimensions for FrameBuf<C, B> {
    fn size(&self) -> Size {
        Size::new(self.width as u32, self.height as u32)
    }
}

impl<C: PixelColor, B: FrameBufferBackend<Color = C>> DrawTarget for FrameBuf<C, B> {
    type Color = C;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            if coord.x >= 0
                && coord.x < self.width as i32
                && coord.y >= 0
                && coord.y < self.height as i32
            {
                self.set_color_at(coord, color);
            }
        }
        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_color_at(Point::new(x as i32, y as i32), color);
            }
        }
        Ok(())
    }
}

/// An iterator for all [Pixels](Pixel) in the framebuffer.
pub struct PixelIterator<'a, C: PixelColor, B: FrameBufferBackend<Color = C>> {
    fbuf: &'a FrameBuf<C, B>,
    index: usize,
}

impl<'a, C: PixelColor, B: FrameBufferBackend<Color = C>> Iterator for PixelIterator<'a, C, B> {
    type Item = Pixel<C>;
    fn next(&mut self) -> Option<Pixel<C>> {
        let y = self.index / self.fbuf.width;
        let x = self.index - y * self.fbuf.width;

        if self.index >= self.fbuf.width * self.fbuf.height {
            return None;
        }
        self.index += 1;
        let p = Point::new(x as i32, y as i32);
        Some(Pixel(p, self.fbuf.get_color_at(p)))
    }
}

unsafe impl<C: PixelColor, B: DMACapableFrameBufferBackend<Color = C>> ReadBuffer
    for FrameBuf<C, B>
{
    type Word = u8;
    unsafe fn read_buffer(&self) -> (*const Self::Word, usize) {
        (
            (self.data.data_ptr() as *const Self::Word),
            self.height
                * self.width
                * (core::mem::size_of::<C>() / core::mem::size_of::<Self::Word>()),
        )
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use embedded_graphics::mock_display::MockDisplay;
    use embedded_graphics::pixelcolor::BinaryColor;
    use embedded_graphics::prelude::Point;
    use embedded_graphics::prelude::Primitive;
    use embedded_graphics::primitives::Line;
    use embedded_graphics::primitives::PrimitiveStyle;
    use embedded_graphics::Drawable;
    use embedded_graphics::{pixelcolor::Rgb565, prelude::RgbColor};
    use std::collections::HashMap;
    use std::fmt::Debug;
    use std::hash::Hash;

    use super::*;

    fn get_px_nums<C: PixelColor, B: FrameBufferBackend<Color = C>>(
        fbuf: &FrameBuf<C, B>,
    ) -> HashMap<C, i32>
    where
        C: Hash,
        C: std::cmp::Eq,
    {
        let mut px_nums: HashMap<C, i32> = HashMap::new();
        for px in fbuf.into_iter() {
            //for px in col {
            match px_nums.get_mut(&px.1) {
                Some(v) => *v += 1,
                None => {
                    px_nums.insert(px.1, 1);
                }
            };
            //}
        }
        px_nums
    }

    #[test]
    fn clears_buffer() {
        let mut data = [Rgb565::WHITE; 5 * 10];
        let mut fbuf = FrameBuf::new(&mut data, 5, 10);
        fbuf.reset();

        let px_nums = get_px_nums(&fbuf);

        assert_eq!(px_nums.get(&Rgb565::BLACK).unwrap(), &50);
        assert_eq!(px_nums.get(&Rgb565::WHITE), None);
    }

    #[test]
    fn clears_with_color() {
        let mut data = [Rgb565::WHITE; 5 * 5];
        let mut fbuf = FrameBuf::new(&mut data, 5, 5);
        fbuf.clear(Rgb565::BLUE).unwrap();

        let px_nums = get_px_nums(&fbuf);

        assert_eq!(px_nums.get(&Rgb565::BLUE).unwrap(), &25);
        assert_eq!(px_nums.get(&Rgb565::RED), None);
    }

    #[test]
    fn draws_into_display() {
        let mut data = [BinaryColor::Off; 12 * 11];
        let mut fbuf = FrameBuf::new(&mut data, 12, 11);
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();

        // Horizontal line
        Line::new(Point::new(2, 2), Point::new(10, 2))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
            .draw(&mut fbuf)
            .unwrap();

        // Vertical line
        Line::new(Point::new(2, 5), Point::new(2, 10))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 3))
            .draw(&mut fbuf)
            .unwrap();

        display.draw_iter(fbuf.into_iter()).unwrap();
        display.assert_pattern(&[
            "............",
            "..#########.",
            "..#########.",
            "............",
            "............",
            ".###........",
            ".###........",
            ".###........",
            ".###........",
            ".###........",
            ".###........",
        ]);
    }

    fn draw_into_drawtarget<D>(mut dt: D)
    where
        D: DrawTarget<Color = BinaryColor>,
        D::Error: Debug,
    {
        Line::new(Point::new(2, 2), Point::new(10, 2))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
            .draw(&mut dt)
            .unwrap();
    }

    #[test]
    fn usable_as_draw_target() {
        let mut data = [BinaryColor::Off; 15 * 5];
        let fbuf = FrameBuf::new(&mut data, 15, 5);
        draw_into_drawtarget(fbuf)
    }

    #[test]
    fn raw_data() {
        let mut data = [Rgb565::new(1, 2, 3); 3 * 3];
        let mut fbuf = FrameBuf::new(&mut data, 3, 3);
        fbuf.set_color_at(Point { x: 1, y: 0 }, Rgb565::new(3, 2, 1));
        let mut raw_iter = fbuf.data.iter();
        assert_eq!(*raw_iter.next().unwrap(), Rgb565::new(1, 2, 3));
        assert_eq!(*raw_iter.next().unwrap(), Rgb565::new(3, 2, 1));
    }

    #[test]
    #[should_panic]
    fn wrong_data_size() {
        let mut data = [BinaryColor::Off; 5 * 5];
        let _ = &mut FrameBuf::new(&mut data, 12, 3);
    }
}
