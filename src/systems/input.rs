use std::time::Duration;

use crossterm::event::{Event, KeyCode, poll, read};

use crate::{
    WORLD,
    components::movement::{Move, MoveDirection},
    kill,
    systems::get_player_entity,
};

pub struct InputSystem {}
impl InputSystem {
    pub fn execute() -> anyhow::Result<()> {
        // `poll()` waits for an `Event` for a given time period
        if !poll(Duration::from_millis(1))? {
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
                KeyCode::Up | KeyCode::Char('k') => {
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
                KeyCode::Down | KeyCode::Char('j') => {
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
                KeyCode::Left | KeyCode::Char('h') => {
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
                KeyCode::Right | KeyCode::Char('l') => {
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
