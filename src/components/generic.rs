use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}
impl Position {
    #[allow(dead_code)]
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}
impl Default for Position {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}
impl Size {
    #[allow(dead_code)]
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}
impl Default for Size {
    fn default() -> Self {
        Self {
            width: 1,
            height: 1,
        }
    }
}
