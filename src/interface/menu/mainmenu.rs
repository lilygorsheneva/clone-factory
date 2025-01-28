use crate::worldgen;
use std::{cell::RefCell, rc::Rc};

use crate::{game_state::game::Game, static_data::Data};
use crossterm::event::KeyCode;
use ratatui::{widgets::Paragraph, DefaultTerminal, Frame};

use super::{gamemenu::GameMenu, MenuTrait, UILayer};

pub struct MainMenu {
    pub game: Option<Rc<RefCell<Game>>>,
    pub data: &'static Data,
}

impl MainMenu {
    pub fn new() -> Self {
        MainMenu {
            game: None,
            data: Data::get_config(),
        }
    }

    pub fn start_game(&mut self) {
        self.game = Some(worldgen::start_game(self.data));
    }

    pub fn start_or_continue(&mut self) {
        if self.game.is_none() {
            self.start_game();
        }
    }
}

pub enum MainMenuOptions {
    Quit,
    Start,
    Restart,
}

use MainMenuOptions::*;

impl UILayer for MainMenu {
    fn draw(&self, frame: &mut Frame) {
        let text = match self.game {
            None => Paragraph::new("Enter: start/continue.\n Esc: Quit"),
            Some(_) => Paragraph::new("Enter: start/continue.\n Esc: Quit\n R: Delete Save"),
        };
        frame.render_widget(text, frame.area());
    }
}

impl MenuTrait for MainMenu {
    type MenuOptions = MainMenuOptions;

    fn enter_menu(&mut self, terminal: &mut DefaultTerminal) {
        loop {
            terminal.draw(|frame| self.draw(frame)).unwrap();

            match self.read() {
                Some(Quit) => break,
                Some(Restart) => self.game = None,
                Some(Start) => {
                    self.start_or_continue();
                    let mut game_ui =
                        GameMenu::new(self.game.as_mut().expect("Game Object Missing").clone());
                    game_ui.enter_menu(terminal);
                }
                None => {}
            }
        }
    }

    fn parsekey(&self, key: crossterm::event::KeyEvent) -> Option<MainMenuOptions> {
        match key.code {
            KeyCode::Esc => Some(Quit),
            KeyCode::Enter => Some(Start),
            KeyCode::Char('r') => Some(Restart),
            _ => None,
        }
    }
}
