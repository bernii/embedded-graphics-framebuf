//! # Embedded Graphics FrameBuffer
//!
//! `embedded-graphics-framebuf` is a generalized frame buffer implementation
//! for use with Rust's `embedded-graphics` library.
//!
//! Framebuffer approach helps to deal with display flickering when you update
//! multiple parts of the display in separate operations. Instead, with this
//! approach, you're going to write to a in-memory display and push it all
//! at once into your hardware display when the whole picture is drawn.
//!
//! This technique is useful when you're updating large portions of screen
//! or just simply don't want to deal with partial display updates.
//!
//! The approach has been tested on TTGO (esp32) with ST7789
//!

#![no_std]

use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::OriginDimensions,
    prelude::{PixelColor, Point, Size},
    Pixel,
};

/// Constructs frame buffer in memory. Lets you define the size (width & height)
/// and pixel type your using in your display (RGB, Monochrome etc.)
///
/// # Example
/// ```
/// use embedded_graphics::mono_font::ascii::FONT_10X20;
/// use embedded_graphics_framebuf::FrameBuf;
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::mono_font::MonoTextStyle;
/// use embedded_graphics::text::Text;
/// use embedded_graphics::pixelcolor::Rgb565;
///
/// static mut FBUFF: FrameBuf<Rgb565, 240_usize, 135_usize> = FrameBuf([[Rgb565::BLACK; 240]; 135]);
/// let mut fbuff = unsafe { &mut FBUFF };
/// fbuff.reset();
/// Text::new(
///    &"Good luck!",
///    Point::new(10, 13),
///    MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE.into()),
/// )
/// .draw(&mut fbuff).unwrap();
/// ```
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct FrameBuf<C: PixelColor, const X: usize, const Y: usize>(pub [[C; X]; Y]);

impl<C: PixelColor + Default, const X: usize, const Y: usize> FrameBuf<C, X, Y> {
    /// Set all pixels to their [Default] value.
    pub fn reset(&mut self) {
        for y in 0..Y {
            for x in 0..X {
                self.0[y][x] = C::default();
            }
        }
    }
}
impl<C: PixelColor, const X: usize, const Y: usize> FrameBuf<C, X, Y> {
    /// Creates an iterator over all [Pixels](Pixel) in the frame buffer. Can be
    /// used e.g., for rendering the framebuffer to the physical display.
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
    /// let mut fbuf = &mut FrameBuf([[BinaryColor::Off; 12]; 11]);
    /// let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
    /// Line::new(Point::new(2, 2), Point::new(10, 2))
    ///     .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
    ///     .draw(&mut fbuf)
    ///     .unwrap();
    /// display.draw_iter(fbuf.pixels()).unwrap();
    /// ```
    pub fn pixels(&self) -> PixelIterator<C, X, Y> {
        PixelIterator {
            fbuf: self,
            index: 0,
        }
    }

    /// Provides an iterator over the underlying raw color data. Useful for
    /// optimized rendering on certain displays.
    ///
    /// # Example
    ///
    /// ```rust
    /// use embedded_graphics::pixelcolor::Rgb565;
    /// use embedded_graphics_framebuf::FrameBuf;
    /// let mut fbuf = FrameBuf([[Rgb565::new(1, 2, 3); 3]; 3]);
    /// let mut raw_iter = fbuf.raw_data();
    /// assert_eq!(raw_iter.next().unwrap(), Rgb565::new(1, 2, 3));
    /// ```
    ///
    /// One could use it for instance to directly write to a display like the [ST7735](https://docs.rs/st7735-lcd/0.8.1/st7735_lcd/struct.ST7735.html) for a more direct writing.
    ///
    /// ```ignore
    /// let mut fbuf = FrameBuf([[Rgb565::BLACK; 128]; 160]);
    /// display.set_address_window(0, 0, 128, 160);
    /// display.write_pixels(fbuf.raw_data().map(|px| RawU16::from(px).into_inner()));
    /// ```
    pub fn raw_data(&self) -> core::iter::Flatten<core::array::IntoIter<[C; X], Y>> {
        self.0.into_iter().flatten()
    }
}

impl<'a, C: PixelColor, const X: usize, const Y: usize> IntoIterator for &'a FrameBuf<C, X, Y> {
    type Item = Pixel<C>;
    type IntoIter = PixelIterator<'a, C, X, Y>;

    fn into_iter(self) -> Self::IntoIter {
        self.pixels()
    }
}

/// An iterator for all [Pixels](Pixel) in the framebuffer.
pub struct PixelIterator<'a, C: PixelColor, const X: usize, const Y: usize> {
    fbuf: &'a FrameBuf<C, X, Y>,
    index: usize,
}

impl<'a, C: PixelColor, const X: usize, const Y: usize> Iterator for PixelIterator<'a, C, X, Y> {
    type Item = Pixel<C>;
    fn next(&mut self) -> Option<Pixel<C>> {
        let y = self.index / X;
        let x = self.index - y * X;

        if self.index >= X * Y {
            return None;
        }
        self.index += 1;
        Some(Pixel(Point::new(x as i32, y as i32), self.fbuf.0[y][x]))
    }
}

impl<C: PixelColor, const X: usize, const Y: usize> OriginDimensions for FrameBuf<C, X, Y> {
    fn size(&self) -> Size {
        Size::new(X as u32, Y as u32)
    }
}

impl<C: PixelColor, const X: usize, const Y: usize> DrawTarget for FrameBuf<C, X, Y> {
    type Color = C;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            if coord.x >= 0 && coord.x < X as i32 && coord.y >= 0 && coord.y < Y as i32 {
                let Point { x, y } = coord;
                self.0[y as usize][x as usize] = color;
            }
        }
        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        for y in 0..Y {
            for x in 0..X {
                self.0[y][x] = color;
            }
        }
        Ok(())
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

    fn get_px_nums<'a, C: PixelColor, const X: usize, const Y: usize>(
        fbuf: &FrameBuf<C, X, Y>,
    ) -> HashMap<C, i32>
    where
        C: Hash,
        C: std::cmp::Eq,
    {
        let mut px_nums: HashMap<C, i32> = HashMap::new();
        for col in fbuf.0.iter() {
            for px in col {
                match px_nums.get_mut(px) {
                    Some(v) => *v += 1,
                    None => {
                        px_nums.insert(*px, 1);
                    }
                };
            }
        }
        px_nums
    }

    #[test]
    fn clears_buffer() {
        let mut fbuf = FrameBuf([[Rgb565::WHITE; 5]; 10]);
        fbuf.reset();

        let px_nums = get_px_nums(&fbuf);

        assert_eq!(px_nums.get(&Rgb565::BLACK).unwrap(), &50);
        assert_eq!(px_nums.get(&Rgb565::WHITE), None);
    }

    #[test]
    fn clears_with_color() {
        let mut fbuf = FrameBuf([[Rgb565::RED; 5]; 5]);
        fbuf.clear(Rgb565::BLUE).unwrap();

        let px_nums = get_px_nums(&fbuf);

        assert_eq!(px_nums.get(&Rgb565::BLUE).unwrap(), &25);
        assert_eq!(px_nums.get(&Rgb565::RED), None);
    }

    #[test]
    fn draws_into_display() {
        let mut fbuf = FrameBuf([[BinaryColor::Off; 12]; 11]);
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

        display.draw_iter(fbuf.pixels()).unwrap();
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
        let fbuf = FrameBuf([[BinaryColor::Off; 15]; 5]);
        draw_into_drawtarget(fbuf)
    }

    #[test]
    fn raw_data() {
        let mut fbuf = FrameBuf([[Rgb565::new(1, 2, 3); 3]; 3]);
        fbuf.0[0][1] = Rgb565::new(3, 2, 1);
        let mut raw_iter = fbuf.raw_data();
        assert_eq!(raw_iter.next().unwrap(), Rgb565::new(1, 2, 3));
        assert_eq!(raw_iter.next().unwrap(), Rgb565::new(3, 2, 1));
    }
}
