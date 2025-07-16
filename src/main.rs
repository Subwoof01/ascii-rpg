mod components;
mod systems;

use std::{
    cell::RefCell,
    io::{Write, stdout},
    process::exit,
    sync::atomic::{AtomicBool, Ordering},
    thread::sleep,
    time::{Duration, Instant},
};

use crossterm::{ExecutableCommand, QueueableCommand, cursor::MoveTo, style::Print};
use hecs::World;

use crate::{
    components::{
        camera::Camera,
        draw::{Colour, Drawable},
        generic::{Position, Size},
        player::Player,
        tiles::{BlocksMovement, BlocksVision},
    },
    systems::{
        camera::CameraSystem,
        draw::DrawSystem,
        input::InputSystem,
        movement::MovementSystem,
        save_load::{load_from_file, save_to_file},
    },
};

const MAP_WIDTH: u16 = 64;
const MAP_HEIGHT: u16 = 32;
const CAMERA_WIDTH: u16 = 32;
const CAMERA_HEIGHT: u16 = 16;
const DRAW_FPS: u64 = 25;
const PROCESS_FPS: u64 = 300;
const SHOW_FPS: bool = false;

thread_local! {
    static WORLD: RefCell<World> = RefCell::new(World::new());
    static CAMERA_POSITION: RefCell<Position> = RefCell::new(Position::new(MAP_WIDTH / 2, MAP_HEIGHT / 2));
}
static RUNNING: AtomicBool = AtomicBool::new(true);

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
                if x == 0 || x == MAP_WIDTH || y == 0 || y == MAP_HEIGHT {
                    world.spawn((
                        Drawable {
                            char: '█',
                            fg: Colour::new(255, 255, 255),
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
        world.spawn((
            Camera {},
            Position::new(MAP_WIDTH / 2, MAP_HEIGHT / 2),
            Size::new(CAMERA_WIDTH, CAMERA_HEIGHT),
        ));
    });

    // WORLD.with_borrow_mut(|world| {
    //     *world = match load_from_file() {
    //         Ok(w) => {
    //             for entity in w.iter() {
    //                 println!("{:?}", entity.entity());
    //             }
    //             w
    //         }
    //         Err(e) => {
    //             eprintln!("Something went wrong while trying to load the world: {e}");
    //             exit(1);
    //         }
    //     }
    // });

    DrawSystem::setup()?;

    let mut last_time = Instant::now();
    let mut frames_processed = 0;
    let mut frames_drawn = 0;
    let mut last_draw_time = Instant::now();
    while RUNNING.load(Ordering::SeqCst) {
        InputSystem::execute()?;
        MovementSystem::execute(MAP_WIDTH, MAP_HEIGHT)?;
        CameraSystem::execute(MAP_WIDTH, MAP_HEIGHT)?;
        if Instant::now().duration_since(last_draw_time)
            >= Duration::from_micros(1_000_000 / DRAW_FPS)
        {
            DrawSystem::execute()?;
            frames_drawn += 1;
            last_draw_time = Instant::now();
        }
        frames_processed += 1;
        if Instant::now().duration_since(last_time) >= Duration::from_secs(1) && SHOW_FPS {
            last_time = Instant::now();
            let size = crossterm::terminal::size()?;
            let to_print_proc = format!("Process FPS: {frames_processed}");
            let to_print_draw = format!("Draw FPS: {frames_drawn}");
            stdout()
                .queue(MoveTo(size.0 - to_print_proc.len() as u16 - 1, 0))?
                .queue(Print(to_print_proc))?
                .queue(MoveTo(size.0 - to_print_draw.len() as u16 - 1, 1))?
                .queue(Print(to_print_draw))?
                .flush()?;
            frames_processed = 0;
            frames_drawn = 0;
        }
        sleep(Duration::from_micros(1_000_000 / PROCESS_FPS));
    }

    DrawSystem::breakdown()?;

    save_to_file()?;

    Ok(())
}

pub fn kill() {
    RUNNING.store(false, Ordering::SeqCst);
}
