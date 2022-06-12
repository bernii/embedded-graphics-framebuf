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
/// let mut data = [Rgb565::BLACK; 240 * 135];
/// let mut fbuff = &mut FrameBuf::new(&mut data, 240, 135);
/// fbuff.clear_black();
/// Text::new(
///    &"Good luck!",
///    Point::new(10, 13),
///    MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE.into()),
/// )
/// .draw(&mut fbuff).unwrap();
/// ```
// TODO: Once https://github.com/rust-lang/rust/issues/76560 is resolved, change this to `pub struct
// FrameBuf<C: PixelColor, const X: usize, const Y: usize>(pub [C; X * Y]);`
pub struct FrameBuf<'a, C: PixelColor> {
    pub data: &'a mut [C],
    pub width: usize,
    pub height: usize,
}

impl<'a, C: PixelColor + Default> FrameBuf<'a, C> {
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
    pub fn new(data: &'a mut [C], width: usize, height: usize) -> Self {
        assert_eq!(
            data.len(),
            width * height,
            "FrameBuf underlying data size does not match width ({}) * height ({}) = {} but is {}",
            width,
            height,
            width * height,
            data.len()
        );
        Self {
            data,
            width,
            height,
        }
    }

    /// Set all pixels to black.
    pub fn clear_black(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self[Point::new(x as i32, y as i32)] = C::default();
            }
        }
    }
}

impl<'a, C: PixelColor + Sized> core::ops::Index<Point> for FrameBuf<'a, C> {
    type Output = C;
    fn index(&self, p: Point) -> &Self::Output {
        &self.data[self.width * p.y as usize + p.x as usize]
    }
}
impl<'a, C: PixelColor + Sized> core::ops::IndexMut<Point> for FrameBuf<'a, C> {
    fn index_mut(&mut self, p: Point) -> &mut Self::Output {
        &mut self.data[self.width * p.y as usize + p.x as usize]
    }
}

impl<'a, C: PixelColor> IntoIterator for &'a mut FrameBuf<'a, C> {
    type Item = C;
    type IntoIter = FrameBufIntoIterator<'a, C>;

    fn into_iter(self) -> Self::IntoIter {
        FrameBufIntoIterator {
            fbuf: self,
            index: 0,
        }
    }
}

impl<'a, C: PixelColor> IntoIterator for &'a FrameBuf<'a, C> {
    type Item = C;
    type IntoIter = FrameBufIntoIterator<'a, C>;

    fn into_iter(self) -> Self::IntoIter {
        FrameBufIntoIterator {
            fbuf: self,
            index: 0,
        }
    }
}

/// Gives you ability to convert the `FrameBuf` data into an iterator. This is
/// commonly used when iterating over pixels in order to send the pixel data
/// into the hardware display.
pub struct FrameBufIntoIterator<'a, C: PixelColor> {
    fbuf: &'a FrameBuf<'a, C>,
    index: usize,
}

impl<'a, C: PixelColor> Iterator for FrameBufIntoIterator<'a, C> {
    type Item = C;
    fn next(&mut self) -> Option<C> {
        let y = self.index / self.fbuf.width;
        let x = self.index - y * self.fbuf.width;

        if self.index >= self.fbuf.width * self.fbuf.height {
            return None;
        }
        self.index += 1;
        Some(self.fbuf[Point::new(x as i32, y as i32)])
    }
}

impl<'a, C: PixelColor> OriginDimensions for &mut FrameBuf<'a, C> {
    fn size(&self) -> Size {
        Size::new(self.width as u32, self.height as u32)
    }
}

impl<'a, C: PixelColor> DrawTarget for &mut FrameBuf<'a, C> {
    type Color = C;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            self[coord] = color;
        }
        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        for x in 0..self.width {
            for y in 0..self.height {
                self[Point::new(x as i32, y as i32)] = color;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
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

    fn get_px_nums<'a, C: PixelColor>(fbuf: &FrameBuf<C>) -> HashMap<C, i32>
    where
        C: Hash,
        C: std::cmp::Eq,
    {
        let mut px_nums: HashMap<C, i32> = HashMap::new();
        for px in fbuf.data.iter() {
            //for px in col {
            match px_nums.get_mut(px) {
                Some(v) => *v += 1,
                None => {
                    px_nums.insert(*px, 1);
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
        fbuf.clear_black();

        let px_nums = get_px_nums(&fbuf);

        assert_eq!(px_nums.get(&Rgb565::BLACK).unwrap(), &50);
        assert_eq!(px_nums.get(&Rgb565::WHITE), None);
    }

    #[test]
    fn clears_with_color() {
        let mut data = [Rgb565::WHITE; 5 * 5];
        let mut fbuf = &mut FrameBuf::new(&mut data, 5, 5);
        fbuf.clear(Rgb565::BLUE).unwrap();

        let px_nums = get_px_nums(&fbuf);

        assert_eq!(px_nums.get(&Rgb565::BLUE).unwrap(), &25);
        assert_eq!(px_nums.get(&Rgb565::RED), None);
    }

    #[test]
    fn draws_into_display() {
        let mut data = [BinaryColor::Off; 12 * 11];
        let mut fbuf = &mut FrameBuf::new(&mut data, 12, 11);
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

        let pixels = fbuf.into_iter().enumerate().map(|(i, px)| {
            let y = (i / 12) as i32;
            let x = (i as i32 - y * 12) as i32;
            let point = Point { x, y };
            Pixel(point, px)
        });
        display.draw_iter(pixels).unwrap();
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
        let fbuf = &mut FrameBuf::new(&mut data, 15, 5);
        draw_into_drawtarget(fbuf)
    }
    #[test]
    #[should_panic]
    fn wrong_data_size() {
        let mut data = [BinaryColor::Off; 5 * 5];
        let _ = &mut FrameBuf::new(&mut data, 12, 3);
    }
}
