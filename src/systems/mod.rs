pub mod draw;
pub mod input;

#[derive(Debug)]
pub struct Position {
    x: u16,
    y: u16,
}
impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            x,
            y
        }
    }
}
