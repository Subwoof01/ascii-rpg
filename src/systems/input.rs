use std::time::Duration;

use crossterm::event::{Event, KeyCode, poll, read};

use crate::{
    WORLD,
    components::{
        movement::{Move, MoveDirection},
        player::Player,
    },
    kill,
    systems::get_player_entity,
};

pub struct InputSystem {}
impl InputSystem {
    pub fn execute() -> anyhow::Result<()> {
        // `poll()` waits for an `Event` for a given time period
        if !poll(Duration::from_millis(100))? {
            return Ok(());
        }
        // It's guaranteed that the `read()` won't block when the `poll()`
        // function returns `true`
        let event = read()?;
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Char('q') => {
                    kill();
                }
                KeyCode::Up => {
                    if let Some(player) = get_player_entity() {
                        WORLD.with_borrow_mut(|world| {
                            world
                                .insert(
                                    player,
                                    (Move {
                                        dir: MoveDirection::Up,
                                    },),
                                )
                                .unwrap();
                        });
                    }
                }
                KeyCode::Down => {
                    if let Some(player) = get_player_entity() {
                        WORLD.with_borrow_mut(|world| {
                            world
                                .insert(
                                    player,
                                    (Move {
                                        dir: MoveDirection::Down,
                                    },),
                                )
                                .unwrap();
                        });
                    }
                }
                KeyCode::Left => {
                    if let Some(player) = get_player_entity() {
                        WORLD.with_borrow_mut(|world| {
                            world
                                .insert(
                                    player,
                                    (Move {
                                        dir: MoveDirection::Left,
                                    },),
                                )
                                .unwrap();
                        });
                    }
                }
                KeyCode::Right => {
                    if let Some(player) = get_player_entity() {
                        WORLD.with_borrow_mut(|world| {
                            world
                                .insert(
                                    player,
                                    (Move {
                                        dir: MoveDirection::Right,
                                    },),
                                )
                                .unwrap();
                        });
                    }
                }
                _ => {}
            }
        }
        // NOTE: Leaving this here for future reference.
        // match read()? {
        //     Event::FocusGained => println!("FocusGained"),
        //     Event::FocusLost => println!("FocusLost"),
        //     Event::Key(event) => println!("{:?}", event),
        //     Event::Mouse(event) => println!("{:?}", event),
        //     Event::Resize(width, height) => println!("New size {}x{}", width, height),
        //     _ => todo!(),
        // }
        Ok(())
    }
}
