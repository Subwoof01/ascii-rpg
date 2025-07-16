use std::{
    io::{Write, stdout},
    u16,
};

use crossterm::{
    QueueableCommand,
    cursor::{self, MoveTo},
    style::{Color, Colors, Print, SetColors},
    terminal::{
        self, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
    },
};

use crate::{
    CAMERA_POSITION, WORLD,
    components::{
        camera::Camera,
        draw::Drawable,
        generic::{Position, Size},
    },
};

pub struct DrawSystem {}
impl DrawSystem {
    pub fn execute() -> anyhow::Result<()> {
        WORLD.with_borrow_mut(|world| -> anyhow::Result<()> {
            // Use these defaults so if there's no camera present, we just render everything.
            let mut cam_size = Size::new(u16::MAX, u16::MAX);
            let mut cam_pos = Position::new(u16::MAX / 2, u16::MAX / 2);
            for (_id, (_cam, cam_s, cam_p)) in world.query_mut::<(&Camera, &Size, &Position)>() {
                cam_size = *cam_s;
                cam_pos = *cam_p;
                break;
            }
            for (_id, (drawable, position)) in world.query_mut::<(&Drawable, &Position)>() {
                // for w in 0..drawable.size.width {
                //     for h in 0..drawable.size.height {
                stdout().queue(MoveTo(position.x, position.y))?;
                if position.x >= cam_pos.x - cam_size.width / 2
                    && position.x <= cam_pos.x + cam_size.width / 2
                    && position.y >= cam_pos.y - cam_size.height / 2
                    && position.y <= cam_pos.y + cam_size.height / 2
                {
                    stdout()
                        .queue(Print(drawable.char))?
                        .queue(SetColors(Colors::new(
                            drawable.fg.as_crossterm_color(),
                            drawable.bg.as_crossterm_color(),
                        )))?;
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
                //     }
                // }
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
