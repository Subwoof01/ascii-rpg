use std::io::{Write, stdout};

use crossterm::{
    QueueableCommand,
    cursor::{self, MoveTo},
    style::{Colors, Print, SetColors},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use crate::{WORLD, components::draw::Drawable};

pub struct DrawSystem {}
impl DrawSystem {
    pub fn execute() -> anyhow::Result<()> {
        WORLD.with_borrow_mut(|world| -> anyhow::Result<()> {
            for (id, drawable) in world.query_mut::<&Drawable>() {
                stdout()
                    .queue(MoveTo(drawable.position.x, drawable.position.y))?
                    .queue(Print(drawable.character))?
                    .queue(SetColors(Colors::new(drawable.fg, drawable.bg)))?;
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
