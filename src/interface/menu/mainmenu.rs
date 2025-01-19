use crate::{
    datatypes::Coordinate, engine::update::Updatable, game_state::game::Game,
    inventory::Item, static_data::StaticData,
};
use crossterm::event::KeyCode;
use ratatui::{widgets::Paragraph, DefaultTerminal, Frame};

use super::{gamemenu::GameMenu, MenuTrait};

pub struct MainMenu {
    pub game: Option<Game>,
    pub data: &'static StaticData,
}

impl MainMenu {
    pub fn new() -> Self {
        MainMenu {
            game: None,
            data: StaticData::get_config(),
        }
    }

    pub fn start_game(&mut self) {
        let mut game = Game::new(Coordinate { x: 20, y: 10 }, &self.data);

        game.spawn(&Coordinate { x: 1, y: 1 }).unwrap();

        let item_def = self.data.items.get(&"raw_crystal".to_string()).unwrap();
        let foo = Item::new(item_def, 1);

        game.world
            .items
            .mut_set(&Coordinate { x: 10, y: 5 }, &[Some(foo)])
            .unwrap();

        self.game = Some(game);
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

impl MenuTrait for MainMenu {
    type MenuOptions = MainMenuOptions;

    fn draw(&self, frame: &mut Frame) {
        let text = match self.game {
            None => Paragraph::new("Enter: start/continue.\n Esc: Quit"),
            Some(_) => Paragraph::new("Enter: start/continue.\n Esc: Quit\n R: Delete Save")
        };
        frame.render_widget(text, frame.area());
    }
    fn call(&mut self, terminal: &mut DefaultTerminal) {
        loop {
            terminal.draw(|frame| self.draw(frame)).unwrap();

            match self.read() {
                Some(Quit) => break,
                Some(Restart) => self.game = None,
                Some(Start) => {
                    self.start_or_continue();
                    let mut game_ui =
                        GameMenu::new(self.game.as_mut().expect("Game Object Missing"));
                    game_ui.call(terminal);
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
