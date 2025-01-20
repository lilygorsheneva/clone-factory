use crossterm::event::KeyCode;
use ratatui::{style::{Color, Stylize}, widgets::Paragraph, DefaultTerminal};

use crate::interface::menu::MenuTrait;

#[derive(Clone, Copy,Debug, PartialEq)]
pub enum Status {
    // An Actor failed to perform an action. Triggers a fallback.
    ActionFail(&'static str),
    OutOfBounds, 
    // A world state update failed to apply. This breaks any semblance of atomicity.
    StateUpdateError,
    // An unexpected error. Panic.
    Error(&'static str),
}

pub type Result<T> = std::result::Result<T, Status>;

pub enum StatusMenuOptions {
    Exit
}

impl MenuTrait for Status {
    type MenuOptions = StatusMenuOptions;

    fn draw(&self, frame: &mut ratatui::Frame) {
        let widget = match self {
            Self::ActionFail(str) => Paragraph::new(*str).fg(Color::Black).bg(Color::LightYellow),
            Self::OutOfBounds => Paragraph::new("Some operation returned out of bounds. This should not be player-visible.").fg(Color::Black).bg(Color::Red),
            Self::StateUpdateError => Paragraph::new("Error updating world state.").fg(Color::Black).bg(Color::Red),
            Self::Error(str) => Paragraph::new(*str).fg(Color::Black).bg(Color::Red),
        };
        frame.render_widget(widget, frame.area());
    }

    fn call(&mut self, terminal: &mut ratatui::DefaultTerminal) {
        terminal.draw(|frame| self.draw(frame)).unwrap();
        loop {
            match self.read() {
                None => {}
                Some(StatusMenuOptions::Exit) => break,
            }
        }
    }

    fn parsekey(&self, key: crossterm::event::KeyEvent) ->  Option<Self::MenuOptions>  {
        match key.code {
            KeyCode::Esc => Some(StatusMenuOptions::Exit),
            _=>None
        }
    }
}


pub trait OkOrPopup {
    fn ok_or_popup(self, terminal: &mut DefaultTerminal);
}

impl OkOrPopup for Result<()> {
    fn ok_or_popup(self, terminal: &mut DefaultTerminal) {
        match self {
            Ok(()) => {},            
            Err(Status::ActionFail(msg)) => Status::ActionFail(msg).call(terminal),
            Err(status) => { status.clone().call(terminal); panic!("Uncaught error when generating game update.")}
        };
    }
}
