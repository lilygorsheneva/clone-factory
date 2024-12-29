use crossterm::event::{self, Event, KeyCode};

pub enum InputResult {
    Nothing,
    Act,
    Redraw,
    Exit,
}

pub fn readinput() -> std::io::Result<InputResult> {
    match event::read()? {
        Event::Key(event) if event.code == KeyCode::Esc => Ok(InputResult::Exit),
        Event::Key(event) => Ok(InputResult::Act),
        Event::Resize(_, _) => Ok(InputResult::Redraw),
        _ => Ok(InputResult::Nothing),
    }
}
