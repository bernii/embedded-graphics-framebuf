//! Backends for a framebuffer.
//!
//! One could use a simple array of [`PixelColor`], or some more elaborate proxy
//! backends.
//!
//! Example:
//! ```rust
//! use embedded_graphics::pixelcolor::BinaryColor;
//! use embedded_graphics_framebuf::FrameBuf;
//! let mut data = [BinaryColor::Off; 12 * 11]; // A potential backend
//! let mut fbuf = FrameBuf::new(&mut data, 12, 11);
//! ```

use embedded_graphics::pixelcolor::PixelColor;

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
/// requirements specified in [`ReadBuffer::read_buffer`] remain.
unsafe impl<'a, C: PixelColor, const N: usize> DMACapableFrameBufferBackend for &'a mut [C; N] {
    fn data_ptr(&self) -> *const C {
        self.as_ptr()
    }
}
