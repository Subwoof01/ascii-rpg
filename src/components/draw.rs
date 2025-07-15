use crossterm::style::Color;

use crate::components::generic::Position;

#[derive(Debug)]
pub struct Drawable {
    pub character: char,
    pub position: Position,
    pub fg: Color,
    pub bg: Color,
}
impl Drawable {
    pub fn new(character: char, position: Position, fg: Color, bg: Color) -> Self {
        Self {
            character,
            position,
            fg,
            bg,
        }
    }
}
