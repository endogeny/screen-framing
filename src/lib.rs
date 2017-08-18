#![warn(missing_docs)]

//! Screen recording, integrated with the rest of your video stuff!

extern crate framing;
extern crate scrap;

pub use scrap::Display;
use framing::{Chunky, Bgra};
use std::io;

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
    pub fn frame(&mut self) -> io::Result<Chunky<Bgra, Screenshot>> {
        Ok(Chunky::from_bytes(
            self.capturer.width(),
            self.capturer.height(),
            Screenshot { frame: self.capturer.frame()? }
        ))
    }
}

/// A captured frame.
pub struct Screenshot<'a> {
    frame: scrap::Frame<'a>
}

impl<'a> AsRef<[u8]> for Screenshot<'a> {
    fn as_ref(&self) -> &[u8] {
        self.frame.as_ref()
    }
}

unsafe impl<'a> Sync for Screenshot<'a> {}
unsafe impl<'a> Send for Screenshot<'a> {}
