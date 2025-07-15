mod components;
mod systems;

use std::{
    cell::RefCell,
    sync::atomic::{AtomicBool, Ordering},
    thread::sleep,
    time::Duration,
};

use bitcode::{Decode, Encode};
use crossterm::style::Color;
use hecs::World;

use crate::{
    components::{draw::Drawable, generic::Position},
    systems::{draw::DrawSystem, input::InputSystem},
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
                world.spawn((Drawable::new(
                    '.',
                    Position::new(x, y),
                    Color::White,
                    Color::Rgb { r: 0, g: 0, b: 0 },
                ),));
            }
        }
    });

    DrawSystem::setup()?;

    while RUNNING.load(Ordering::SeqCst) {
        InputSystem::execute()?;
        DrawSystem::execute()?;
        sleep(Duration::from_millis(16));
    }

    DrawSystem::breakdown()?;

    Ok(())
}

pub fn kill() {
    RUNNING.store(false, Ordering::SeqCst);
}
