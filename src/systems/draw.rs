use std::io::{Write, stdout};

use crossterm::{
    QueueableCommand,
    cursor::{self, MoveTo},
    style::{Color, Colors, Print, SetColors},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use crate::{
    CAMERA_POSITION, WORLD,
    components::{draw::Drawable, generic::Position, movement::Move},
};

pub struct DrawSystem {}
impl DrawSystem {
    pub fn execute(cam_width: u16, cam_height: u16) -> anyhow::Result<()> {
        WORLD.with_borrow_mut(|world| -> anyhow::Result<()> {
            for (_id, (drawable, position)) in world.query_mut::<(&Drawable, &Position)>() {
                for w in 0..drawable.size.width {
                    for h in 0..drawable.size.height {
                        CAMERA_POSITION.with_borrow(|cam_pos| -> anyhow::Result<()> {
                            stdout().queue(MoveTo(position.x + w, position.y + h))?;
                            if position.x >= cam_pos.x - cam_width / 2
                                && position.x <= cam_pos.x + cam_width / 2
                                && position.y >= cam_pos.y - cam_height / 2
                                && position.y <= cam_pos.y + cam_height / 2
                            {
                                stdout().queue(Print(drawable.char))?.queue(SetColors(
                                    Colors::new(
                                        drawable.fg.as_crossterm_color(),
                                        drawable.bg.as_crossterm_color(),
                                    ),
                                ))?;
                            } else {
                                stdout().queue(Print(" "))?.queue(SetColors(Colors::new(
                                    Color::Rgb {
                                        r: 255,
                                        g: 255,
                                        b: 255,
                                    },
                                    Color::Rgb { r: 0, g: 0, b: 0 },
                                )))?;
                            }
                            Ok(())
                        })?;
                    }
                }
            }
            stdout().flush()?;
            Ok(())
        })?;

        Ok(())
    }

    pub fn setup() -> anyhow::Result<()> {
        enable_raw_mode()?;
        stdout().queue(EnterAlternateScreen)?.queue(cursor::Hide)?;
        Ok(())
    }

    pub fn breakdown() -> anyhow::Result<()> {
        disable_raw_mode()?;
        stdout().queue(LeaveAlternateScreen)?.queue(cursor::Show)?;
        Ok(())
    }
}
