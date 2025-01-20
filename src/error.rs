use crossterm::event::KeyCode;
use ratatui::{style::{Color, Stylize}, widgets::Paragraph, DefaultTerminal};

use crate::interface::{menu::{MenuTrait, UILayer}, widgets::generate_popup_layout};

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


pub struct StatusMenu<'a> {
    status: Status,
    parent: &'a dyn UILayer
}

pub enum StatusMenuOptions {
    Exit
}

impl<'a> StatusMenu<'a> {
   pub fn new(status: Status, parent: &'a dyn UILayer) -> StatusMenu<'a>{
        StatusMenu {
            status,
            parent
        }
    }
}

impl UILayer for StatusMenu<'_>  {
    fn draw(&self, frame: &mut ratatui::Frame) {
        self.parent.draw(frame);
        let widget = match self.status {
            Status::ActionFail(str) => Paragraph::new(str).fg(Color::Black).bg(Color::LightYellow),
            Status::OutOfBounds => Paragraph::new("Some operation returned out of bounds. This should not be player-visible.").fg(Color::Black).bg(Color::Red),
            Status::StateUpdateError => Paragraph::new("Error updating world state.").fg(Color::Black).bg(Color::Red),
            Status::Error(str) => Paragraph::new(str).fg(Color::Black).bg(Color::Red),
        };
        frame.render_widget(widget, generate_popup_layout(frame));
    }
}

impl MenuTrait for StatusMenu<'_> {
    type MenuOptions = StatusMenuOptions;

    fn enter_menu(&mut self, terminal: &mut ratatui::DefaultTerminal) {
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
    fn ok_or_popup(self, parent: &dyn UILayer, terminal: &mut DefaultTerminal);
}

impl OkOrPopup for Result<()> {
    fn ok_or_popup(self, parent: &dyn UILayer, terminal: &mut DefaultTerminal) {
        match self {
            Ok(()) => {},            
            Err(Status::ActionFail(msg)) => StatusMenu::new(Status::ActionFail(msg), parent).enter_menu(terminal),
            Err(status) => { StatusMenu::new(status, parent).enter_menu(terminal); panic!("Uncaught error when generating game update.")}
        };
    }
}
