use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::OriginDimensions,
    prelude::{PixelColor, Size},
    Pixel,
};
use std::convert::TryInto;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct FrameBuf<C: PixelColor, const X: usize, const Y: usize>(pub [[C; X]; Y]);

impl<C: PixelColor + Default, const X: usize, const Y: usize> FrameBuf<C, X, Y> {
    /// Set all pixels to black.
    pub fn clear_black(&mut self) {
        for x in 0..X {
            for y in 0..Y {
                self.0[y][x] = C::default();
            }
        }
    }
}

impl<'a, C: PixelColor, const X: usize, const Y: usize> IntoIterator for &'a mut FrameBuf<C, X, Y> {
    type Item = C;
    type IntoIter = FrameBufIntoIterator<'a, C, X, Y>;

    fn into_iter(self) -> Self::IntoIter {
        FrameBufIntoIterator {
            fbuf: self,
            index: 0,
        }
    }
}

impl<'a, C: PixelColor, const X: usize, const Y: usize> IntoIterator for &'a FrameBuf<C, X, Y> {
    type Item = C;
    type IntoIter = FrameBufIntoIterator<'a, C, X, Y>;

    fn into_iter(self) -> Self::IntoIter {
        FrameBufIntoIterator {
            fbuf: self,
            index: 0,
        }
    }
}
pub struct FrameBufIntoIterator<'a, C: PixelColor, const X: usize, const Y: usize> {
    fbuf: &'a FrameBuf<C, X, Y>,
    index: usize,
}

impl<'a, C: PixelColor, const X: usize, const Y: usize> Iterator
    for FrameBufIntoIterator<'a, C, X, Y>
{
    type Item = C;
    fn next(&mut self) -> Option<C> {
        let y = self.index / X;
        let x = self.index - y * X;

        if self.index >= X * Y {
            return None;
        }
        self.index += 1;
        Some(self.fbuf.0[y][x])
    }
}

impl<C: PixelColor, const X: usize, const Y: usize> OriginDimensions for &mut FrameBuf<C, X, Y> {
    fn size(&self) -> Size {
        Size::new(X as u32, Y as u32)
    }
}

impl<C: PixelColor, const X: usize, const Y: usize> DrawTarget for &mut FrameBuf<C, X, Y> {
    type Color = C;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            if let Ok(pos) = coord.try_into() {
                let (x, y): (u32, u32) = pos;
                self.0[y as usize][x as usize] = color;
            }
        }
        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        for x in 0..X {
            for y in 0..Y {
                self.0[y][x] = color;
            }
        }
        Ok(())
    }
}
