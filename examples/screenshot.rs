extern crate png_framing;
extern crate screen_framing;
extern crate framing;

use framing::video::ChunkyFrame;
use screen_framing::{Capturer, Display};
use png_framing::Png;
use std::io::ErrorKind::WouldBlock;
use std::thread;
use std::time::Duration;

fn main() {
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;

    let display = Display::primary().unwrap();
    let mut capturer = Capturer::new(display).unwrap();

    for i in 0..3 {
        println!("{}...", 3 - i);
        thread::sleep(one_second);
    }

    loop {
        let frame = match capturer.frame() {
            Ok(frame) => frame,
            Err(error) => {
                if error.kind() == WouldBlock {
                    thread::sleep(one_frame);
                    continue;
                } else {
                    println!("Capture error: {}.", error);
                    break;
                }
            }
        };
        
        println!("Captured! Saving...");
        match Png::from(ChunkyFrame::new(frame)).save("screenshot.png") {
            Ok(_) => println!("Image saved to `screenshot.png`."),
            Err(_) => println!("Couldn't save image to `screenshot.png`.")
        }
        break;
    }
}
