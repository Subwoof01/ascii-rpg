mod components;
mod systems;

use std::{
    cell::RefCell,
    sync::atomic::{AtomicBool, Ordering},
    thread::sleep,
    time::Duration,
};

use hecs::World;
use rand::Rng;

use crate::{
    components::{draw::Drawable, generic::Position, player::Player},
    systems::{
        draw::DrawSystem,
        input::InputSystem,
        movement::MovementSystem,
        save_load::{load_from_file, save_to_file},
    },
};

thread_local! {
    static WORLD: RefCell<World> = RefCell::new(World::new());
}
static RUNNING: AtomicBool = AtomicBool::new(true);

fn main() -> anyhow::Result<()> {
    ctrlc::set_handler(move || {
        kill();
    })?;

    WORLD.with_borrow_mut(|world| {
        for x in 0..=32 {
            for y in 0..=16 {
                if x == 16 && y == 8 {
                    world.spawn((
                        Drawable {
                            char: '@',
                            ..Default::default()
                        },
                        Position::new(x, y),
                        Player {},
                    ));
                }
                world.spawn((
                    Drawable {
                        char: '.',
                        ..Default::default()
                    },
                    Position::new(x, y),
                ));
            }
        }
    });

    // WORLD.with_borrow_mut(|world| {
    //     *world = match load_from_file() {
    //         Ok(w) => w,
    //         Err(e) => {
    //             eprintln!("Something went wrong while trying to load the world: {e}");
    //             exit(1);
    //         }
    //     }
    // });

    DrawSystem::setup()?;

    while RUNNING.load(Ordering::SeqCst) {
        InputSystem::execute()?;
        MovementSystem::execute()?;
        DrawSystem::execute()?;
        sleep(Duration::from_millis(16));
    }

    DrawSystem::breakdown()?;

    // save_to_file()?;

    Ok(())
}

pub fn kill() {
    RUNNING.store(false, Ordering::SeqCst);
}
