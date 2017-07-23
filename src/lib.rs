#![warn(missing_docs)]

//! Screen recording, integrated with the rest of your video stuff!

extern crate framing;
extern crate scrap;

pub use scrap::Display;
use framing::video::{VideoFrame, Rgba};
use std::{io, mem, ptr};

/// A screen-capturing session.
///
/// This is just a small wrapper around the underlying library that adds width
/// and height information to each individual captured frame.
pub struct Capturer {
    capturer: scrap::Capturer
}

impl Capturer {
    /// Begins a capturing session.
    pub fn new(display: Display) -> io::Result<Self> {
        let capturer = scrap::Capturer::new(display)?;
        Ok(Capturer { capturer })
    }

    /// Gets the width (in pixels) of each captured frame.
    pub fn width(&self) -> usize { self.capturer.width() }

    /// Gets the height (in pixels) of each captured frame.
    pub fn height(&self) -> usize { self.capturer.height() }

    /// Captures a frame, but returns `Err(WouldBlock)` if the operating system
    /// doesn't have a frame for us yet. Only one frame can be borrowed at a
    /// time, since in most cases the memory is reused.
    pub fn frame(&mut self) -> io::Result<Frame> {
        Ok(Frame {
            width: self.width(),
            height: self.height(),
            frame: self.capturer.frame()?
        })
    }
}

/// A captured frame.
pub struct Frame<'a> {
    frame: scrap::Frame<'a>,
    width: usize,
    height: usize
}

impl<'a> VideoFrame for Frame<'a> {
    type Pixel = Rgba;

    fn width(&self) -> usize { self.width }
    fn height(&self) -> usize { self.height }

    unsafe fn pixel(&self, x: usize, y: usize) -> Self::Pixel {
        let mut pix: [u8; 4] = mem::uninitialized();
        let off = 4 * (y * self.width + x) as isize;

        ptr::copy_nonoverlapping(
            self.frame.as_ptr().offset(off),
            pix.as_mut_ptr(),
            4
        );

        let base = pix.as_mut_ptr();
        mem::swap(&mut *base, &mut *base.offset(2));
        pix.into()
    }
}

unsafe impl<'a> Sync for Frame<'a> {}
unsafe impl<'a> Send for Frame<'a> {}
