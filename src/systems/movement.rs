use std::iter;

use crate::{
    WORLD,
    components::{
        generic::Position,
        movement::{Move, MoveDirection},
        tiles::BlocksMovement,
    },
};

pub struct MovementSystem {}
impl MovementSystem {
    pub fn execute(width: u16, height: u16) -> anyhow::Result<()> {
        WORLD.with_borrow_mut(|world| -> anyhow::Result<()> {
            let mut to_remove = Vec::new();
            let blockers = world
                .query::<(&BlocksMovement, &Position)>()
                .iter()
                .map(|(_id, (&_b, &pos))| pos)
                .collect::<Vec<_>>();
            for (id, (position, move_to)) in world.query_mut::<(&mut Position, &Move)>() {
                let mut new_pos = Position {
                    x: position.x,
                    y: position.y,
                };
                // Calculate new position before actually moving
                match move_to.dir {
                    MoveDirection::Up => {
                        if position.y != 0 {
                            new_pos.y -= 1;
                        }
                    }
                    MoveDirection::Down => {
                        if position.y + 1 >= height {
                            new_pos.y = height;
                        } else {
                            new_pos.y += 1;
                        }
                    }
                    MoveDirection::Left => {
                        if position.x != 0 {
                            new_pos.x -= 1;
                        }
                    }
                    MoveDirection::Right => {
                        if position.x + 1 >= width {
                            new_pos.x = width;
                        } else {
                            new_pos.x += 1;
                        }
                    }
                }

                // Loop through all potential blockers to see if any would block our movement.
                if blockers
                    .iter()
                    .filter(|pos| pos.x == new_pos.x && pos.y == new_pos.y)
                    .collect::<Vec<_>>()
                    .len()
                    == 0
                {
                    *position = new_pos;
                }
                to_remove.push(id);
            }
            for e in to_remove {
                world.remove::<(Move,)>(e)?;
            }
            Ok(())
        })
    }
}
