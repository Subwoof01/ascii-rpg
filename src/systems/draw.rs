use std::io::{Write, stdout};

use crossterm::{
    QueueableCommand,
    cursor::{self, MoveTo},
    style::{Colors, Print, SetColors},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use crate::{
    WORLD,
    components::{draw::Drawable, generic::Position},
};

pub struct DrawSystem {}
impl DrawSystem {
    pub fn execute() -> anyhow::Result<()> {
        WORLD.with_borrow_mut(|world| -> anyhow::Result<()> {
            for (_id, (drawable, position)) in world.query_mut::<(&Drawable, &Position)>() {
                for w in 0..drawable.size.width {
                    for h in 0..drawable.size.height {
                        stdout()
                            .queue(MoveTo(position.x + w, position.y + h))?
                            .queue(Print(drawable.char))?
                            .queue(SetColors(Colors::new(
                                drawable.fg.as_crossterm_color(),
                                drawable.bg.as_crossterm_color(),
                            )))?;
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
