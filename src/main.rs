mod components;
mod systems;

use std::{
    cell::RefCell,
    process::exit,
    sync::atomic::{AtomicBool, Ordering},
    thread::sleep,
    time::Duration,
};

use hecs::World;

use crate::{
    components::{
        draw::Drawable,
        generic::Position,
        player::Player,
        tiles::{BlocksMovement, BlocksVision},
    },
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

const MAP_WIDTH: u16 = 32;
const MAP_HEIGHT: u16 = 16;

fn main() -> anyhow::Result<()> {
    ctrlc::set_handler(move || {
        kill();
    })?;

    WORLD.with_borrow_mut(|world| {
        for x in 0..=MAP_WIDTH {
            for y in 0..=MAP_HEIGHT {
                if x == MAP_WIDTH / 2 && y == MAP_HEIGHT / 2 {
                    world.spawn((
                        Drawable {
                            char: '@',
                            ..Default::default()
                        },
                        Position::new(x, y),
                        Player {},
                    ));
                }
                if x == 0 || x == MAP_WIDTH || y == 0 || y == MAP_HEIGHT || (x == 4 && y == 4) {
                    world.spawn((
                        Drawable {
                            char: '#',
                            ..Default::default()
                        },
                        Position::new(x, y),
                        BlocksMovement {},
                        BlocksVision {},
                    ));
                } else {
                    world.spawn((
                        Drawable {
                            char: '.',
                            ..Default::default()
                        },
                        Position::new(x, y),
                    ));
                }
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
        MovementSystem::execute(MAP_WIDTH, MAP_HEIGHT)?;
        DrawSystem::execute()?;
        sleep(Duration::from_millis(16));
    }

    DrawSystem::breakdown()?;

    save_to_file()?;

    Ok(())
}

pub fn kill() {
    RUNNING.store(false, Ordering::SeqCst);
}
