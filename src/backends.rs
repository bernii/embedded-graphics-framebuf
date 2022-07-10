//! Backends for a framebuffer.
//!
//! One could use a simple array of [`PixelColor`], or some more elaborate proxy
//! backends.
//!
//! Example:
//! ```rust
//! use embedded_graphics::{pixelcolor::Rgb565, prelude::RgbColor};
//! use embedded_graphics_framebuf::{
//!     backends::{EndianCorrectedBuffer, EndianCorrection},
//!     FrameBuf,
//! };
//! let mut data = [Rgb565::BLACK; 12 * 11]; // A potential backend
//! let mut fbuf = FrameBuf::new(&mut data, 12, 11);
//!
//! let mut fbuf = FrameBuf::new(
//!     EndianCorrectedBuffer::new(&mut data, EndianCorrection::ToBigEndian),
//!     12,
//!     11,
//! );
//! ```

use embedded_graphics::pixelcolor::{raw::RawU16, IntoStorage, PixelColor};

/// This trait marks the requirements for backends for a
/// [`FrameBuf`](crate::FrameBuf).
///
/// In a basic scenario this is just some memory.
/// But one could implement more elaborate backends which allow manipulation of
/// the data on the fly.
pub trait FrameBufferBackend {
    type Color: PixelColor;
    /// Sets a pixel to the respective color
    fn set(&mut self, index: usize, color: Self::Color);

    /// Returns a pixels color
    fn get(&self, index: usize) -> Self::Color;

    /// Nr of elements in the backend
    fn nr_elements(&self) -> usize;
}

/// Backends implementing this Trait can be used for DMA.
///
/// # Safety
///
/// The same restrictions as for [`embedded_dma::ReadBuffer`] apply.
pub unsafe trait DMACapableFrameBufferBackend: FrameBufferBackend {
    fn data_ptr(&self) -> *const Self::Color;
}
impl<'a, C: PixelColor, const N: usize> FrameBufferBackend for &'a mut [C; N] {
    type Color = C;
    fn set(&mut self, index: usize, color: C) {
        self[index] = color
    }

    fn get(&self, index: usize) -> C {
        self[index]
    }

    fn nr_elements(&self) -> usize {
        self.len()
    }
}

/// # Safety:
///
/// The implementation of the trait for all lifetimes `'a` is safe. However,
/// this doesn't mean that the use of it is safe for all lifetimes. The
/// requirements specified in [`embedded_dma::ReadBuffer::read_buffer`] remain.
unsafe impl<'a, C: PixelColor, const N: usize> DMACapableFrameBufferBackend for &'a mut [C; N] {
    fn data_ptr(&self) -> *const C {
        self.as_ptr()
    }
}

/// Enum indicating how the bytes should be converted in the host's memory.
#[derive(PartialEq, Eq)]
pub enum EndianCorrection {
    ToLittleEndian,
    ToBigEndian,
}

/// A backend for [`FrameBuf`](crate::FrameBuf) which changes the underlying
/// byte order. This can be useful when using the buffer for DMA with
/// peripherals that have a different endianness than the host.
pub struct EndianCorrectedBuffer<'a, C: PixelColor> {
    data: &'a mut [C],
    endian: EndianCorrection,
}
impl<'a, C: PixelColor> EndianCorrectedBuffer<'a, C> {
    pub fn new(data: &'a mut [C], endian: EndianCorrection) -> Self {
        Self { data, endian }
    }
}
impl<'a, C> FrameBufferBackend for EndianCorrectedBuffer<'a, C>
where
    // TODO: Make this generic over other
    // types than u16 with associated
    // type bounds once they are stable
    C: IntoStorage<Storage = u16> + PixelColor,
    RawU16: From<C>,
    C: core::convert::From<RawU16>,
{
    type Color = C;
    fn set(&mut self, index: usize, color: C) {
        self.data[index] = match self.endian {
            EndianCorrection::ToBigEndian => RawU16::new(color.into_storage().to_be()).into(),
            EndianCorrection::ToLittleEndian => RawU16::new(color.into_storage().to_le()).into(),
        }
    }

    fn get(&self, index: usize) -> C {
        match self.endian {
            EndianCorrection::ToBigEndian => {
                C::from(RawU16::new(u16::from_be(self.data[index].into_storage())))
            }
            EndianCorrection::ToLittleEndian => {
                C::from(RawU16::new(u16::from_le(self.data[index].into_storage())))
            }
        }
    }

    fn nr_elements(&self) -> usize {
        self.data.len()
    }
}
unsafe impl<'a, C> DMACapableFrameBufferBackend for EndianCorrectedBuffer<'a, C>
where
    C: IntoStorage<Storage = u16> + PixelColor,
    RawU16: From<C>,
    C: core::convert::From<RawU16>,
{
    fn data_ptr(&self) -> *const C {
        self.data.as_ptr()
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use crate::FrameBuf;
    use embedded_graphics::pixelcolor::{raw::RawU16, Rgb565};
    use embedded_graphics::prelude::{Point, RawData, RgbColor};

    #[test]
    fn test_no_endian_correction() {
        let mut data = [Rgb565::BLUE; 2 * 3];
        let mut fbuf = FrameBuf::new(&mut data, 2, 3);
        fbuf.set_color_at(Point::new(1, 0), Rgb565::RED);
        fbuf.set_color_at(Point::new(2, 0), Rgb565::BLUE);
        // Blue in native endian
        assert_eq!(RawU16::from(fbuf.data[0]).into_inner(), 0b00000000_00011111);
        // Red in native endian
        assert_eq!(RawU16::from(fbuf.data[1]).into_inner(), 0b11111000_00000000);
        // Blue in native endian
        assert_eq!(RawU16::from(fbuf.data[2]).into_inner(), 0b00000000_00011111);
    }

    #[test]
    fn test_big_endian_correction() {
        let mut data = [Rgb565::BLUE; 2 * 3];
        let mut fbuf = FrameBuf::new(
            EndianCorrectedBuffer::new(&mut data, EndianCorrection::ToBigEndian),
            2,
            3,
        );
        fbuf.set_color_at(Point::new(1, 0), Rgb565::RED);
        fbuf.set_color_at(Point::new(2, 0), Rgb565::BLUE);

        // Access functions work as expected
        assert_eq!(fbuf.get_color_at(Point::new(1, 0)), Rgb565::RED);
        assert_eq!(fbuf.get_color_at(Point::new(2, 0)), Rgb565::BLUE);

        // Red in big endian
        assert_eq!(
            RawU16::from(fbuf.data.data[1]).into_inner(),
            0b00000000_11111000
        );
        // Blue in big endian
        assert_eq!(
            RawU16::from(fbuf.data.data[2]).into_inner(),
            0b00011111_00000000
        );
    }

    #[test]
    fn test_little_endian_correction() {
        let mut data = [Rgb565::BLUE; 2 * 3];
        let mut fbuf = FrameBuf::new(
            EndianCorrectedBuffer::new(&mut data, EndianCorrection::ToLittleEndian),
            2,
            3,
        );
        fbuf.set_color_at(Point::new(1, 0), Rgb565::RED);
        fbuf.set_color_at(Point::new(2, 0), Rgb565::BLUE);

        // Access functions work as expected
        assert_eq!(fbuf.get_color_at(Point::new(1, 0)), Rgb565::RED);
        assert_eq!(fbuf.get_color_at(Point::new(2, 0)), Rgb565::BLUE);

        // Red in little endian
        assert_eq!(
            RawU16::from(fbuf.data.data[1]).into_inner(),
            0b11111000_00000000
        );
        // Blue in little endian
        assert_eq!(
            RawU16::from(fbuf.data.data[2]).into_inner(),
            0b00000000_00011111
        );
    }
}
