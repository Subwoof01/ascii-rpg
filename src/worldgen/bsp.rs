use hecs::World;
use rand::random_range;

use crate::{
    components::{
        draw::{Colour, Drawable},
        generic::{Position, Size},
    },
    worldgen::GenerationStrategy,
};

#[derive(Debug, Default, Clone, Copy)]
struct Room {
    pub x1: u16,
    pub y1: u16,
    pub x2: u16,
    pub y2: u16,
}

enum SplitDirection {
    Horizontal,
    Vertical,
}

pub struct BinarySpacePartitionStrategy {
    rooms: Vec<Room>,
}
impl BinarySpacePartitionStrategy {
    pub fn new(x: u16, y: u16, w: u16, h: u16) -> Self {
        Self {
            rooms: vec![Room {
                x1: x,
                y1: y,
                x2: w,
                y2: h,
            }],
        }
    }
    pub fn split_space(&mut self) {
        let mut new_rooms = Vec::new();
        for room in &self.rooms {
            let direction = match rand::random_range(0..2) {
                0 => SplitDirection::Horizontal,
                _ => SplitDirection::Vertical,
            };
            let mut room_a = *room;
            let mut room_b = *room;
            match direction {
                SplitDirection::Horizontal => {
                    let yr = rand::random_range(room.y1..room.y2);
                    room_a.y2 = yr;
                    room_b.y1 = yr;
                }
                SplitDirection::Vertical => {
                    let xr = rand::random_range(room.x1..room.x2);
                    room_a.x2 = xr;
                    room_b.x1 = xr;
                }
            }
            new_rooms.push(room_a);
            new_rooms.push(room_b);
        }
        println!("{new_rooms:#?}");
        self.rooms = new_rooms;
    }
}

impl GenerationStrategy for BinarySpacePartitionStrategy {
    fn step(&mut self, current_step: u16) {
        self.split_space();
    }

    fn finish(&mut self) -> World {
        let mut world = World::new();
        let mut i = 0;
        for room in &self.rooms {
            let colour = Colour::new(
                random_range(0..=255),
                random_range(0..=255),
                random_range(0..=255),
            );
            for x in room.x1..room.x2 {
                for y in room.y1..room.y2 {
                    let pos = Position { x, y };
                    if x == 0 && y == 0 {
                        println!("ID: {i}");
                        println!("{pos:?}");
                        println!("{colour:?}");
                    }
                    let draw = Drawable {
                        char: '█',
                        size: Size::default(),
                        fg: colour,
                        bg: Colour::default(),
                    };
                    world.spawn((pos, draw));
                }
            }
            i += 1;
        }
        world
    }
}
