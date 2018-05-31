use std::cmp::{max, min};
use termion;
use termion::color;

pub struct Terminal {
    width: Option<u16>,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            width: termion::terminal_size().map(|(w, _)| w).ok(),
        }
    }

    pub fn print_separator<P: AsRef<str>>(&self, message: P) {
        let message = message.as_ref();
        if let Some(width) = self.width {
            print!("\n{}―― {} ", color::Fg(color::LightWhite), message);
            let border = max(2, min(80, width as usize) - 3 - message.len());
            for _ in 0..border {
                print!("―");
            }
            println!("{}", color::Fg(color::Reset));
        } else {
            println!("―― {} ――", message);
        }
    }
}