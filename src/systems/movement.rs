use crate::{
    WORLD,
    components::{
        generic::Position,
        movement::{Move, MoveDirection},
    },
};

pub struct MovementSystem {}
impl MovementSystem {
    pub fn execute() -> anyhow::Result<()> {
        WORLD.with_borrow_mut(|world| -> anyhow::Result<()> {
            let mut to_remove = Vec::new();
            for (id, (position, move_to)) in world.query_mut::<(&mut Position, &Move)>() {
                match move_to.dir {
                    MoveDirection::Up => {
                        position.y -= 1;
                    }
                    MoveDirection::Down => {
                        position.y += 1;
                    }
                    MoveDirection::Left => {
                        position.x -= 1;
                    }
                    MoveDirection::Right => {
                        position.x += 1;
                    }
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
