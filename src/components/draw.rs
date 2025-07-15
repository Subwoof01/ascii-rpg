use crossterm::style::Color;
use serde::{Deserialize, Serialize};

use crate::components::generic::{Position, Size};

#[derive(Debug, Serialize, Deserialize)]
pub struct Colour {
    r: u8,
    g: u8,
    b: u8,
}
impl Colour {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn as_crossterm_color(&self) -> Color {
        Color::Rgb {
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }
}
impl Default for Colour {
    fn default() -> Self {
        Self { r: 0, b: 0, g: 0 }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Drawable {
    pub char: char,
    pub size: Size,
    pub fg: Colour,
    pub bg: Colour,
}
impl Drawable {
    #[allow(dead_code)]
    pub fn new(char: char, size: Size, fg: Colour, bg: Colour) -> Self {
        Self { char, size, fg, bg }
    }
}
impl Default for Drawable {
    fn default() -> Self {
        Self {
            char: ' ',
            size: Size::default(),
            fg: Colour::new(255, 255, 255),
            bg: Colour::new(0, 0, 0),
        }
    }
}
