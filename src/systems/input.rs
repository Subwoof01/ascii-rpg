use std::time::Duration;

use crossterm::event::{Event, KeyCode, poll, read};

use crate::kill;

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
