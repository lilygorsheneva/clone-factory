use crate::game_state::game::Game;
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use crate::error::Result;

pub mod mainmenu;
pub mod gamemenu;
pub mod craftingmenu;

pub type GameFn = dyn Fn(&mut Game) -> Result<()>;


pub trait MenuTrait {
    type MenuOptions;

    fn draw(&self, frame: &mut Frame);
    fn call(&mut self, terminal: &mut DefaultTerminal);

    fn parsekey(&self, key: KeyEvent) ->  Option<Self::MenuOptions> ;

    fn read(&self) -> Option<Self::MenuOptions> {
        match event::read() {
            Ok(Event::Key(key)) if key.kind != KeyEventKind::Release => {
                    self.parsekey(key)
            },
            _ => None
        }
    }
}
