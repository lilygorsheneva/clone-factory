use std::{cell::RefCell, rc::Rc};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{DefaultTerminal, Frame};

use crate::{
    action::{Action, SubAction},
    game_state::game::Game,
    interface::render::{generate_main_layout, ItemBar, WorldWindowWidget},
};

use crate::direction::{
    AbsoluteDirection::{E, N, S, W},
    Direction::{Absolute, Relative},
    RelativeDirection::F,
};

use super::{craftingmenu::CraftingMenu, GameFn, MenuTrait};

pub struct GameMenu {
    game: Rc<RefCell<Game>>,
}

pub enum GameMenuOptions {
    Exit,
    GameFn(Box<GameFn>),
    Craft,
}
use GameMenuOptions::*;

impl GameMenu {
    pub fn new(game:Rc<RefCell<Game>>) -> GameMenu {
        GameMenu { game }
    }
}

impl MenuTrait for GameMenu {
    type MenuOptions = GameMenuOptions;

    fn draw(&self, frame: &mut Frame) {
        let game = self.game.borrow();
        let window = WorldWindowWidget::new(&game);
        let item_widget = ItemBar::new(&game);

        let (main, side, bottom, _corner) = generate_main_layout(frame.area());

        frame.render_widget(item_widget, bottom);
        frame.render_widget(window, main);
    }

    fn parsekey(&self, key: KeyEvent) -> Option<Self::MenuOptions> {
        match key.code {
            KeyCode::Left => Some(GameFn(Box::new(|game: &mut Game| {
                game.player_action_and_turn(Action {
                    direction: Absolute(W),
                    action: SubAction::Move,
                })
            }))),
            KeyCode::Right => Some(GameFn(Box::new(|game: &mut Game| {
                game.player_action_and_turn(Action {
                    direction: Absolute(E),
                    action: SubAction::Move,
                })
            }))),
            KeyCode::Up => Some(GameFn(Box::new(|game: &mut Game| {
                game.player_action_and_turn(Action {
                    direction: Absolute(N),
                    action: SubAction::Move,
                })
            }))),
            KeyCode::Down => Some(GameFn(Box::new(|game: &mut Game| {
                game.player_action_and_turn(Action {
                    direction: Absolute(S),
                    action: SubAction::Move,
                })
            }))),
            KeyCode::Char('r') => Some(GameFn(Box::new(|game: &mut Game| {
                match game.current_recording {
                    Some(_) => game.end_record(),
                    None => game.init_record(),
                }
            }))),
            KeyCode::Char('1') => Some(GameFn(Box::new(|game: &mut Game| {
                game.player_action_and_turn(Action {
                    direction: Relative(F),
                    action: SubAction::Use(0),
                })
            }))),
            KeyCode::Char('2') => Some(GameFn(Box::new(|game: &mut Game| {
                game.player_action_and_turn(Action {
                    direction: Relative(F),
                    action: SubAction::Use(1),
                })
            }))),
            KeyCode::Char('3') => Some(GameFn(Box::new(|game: &mut Game| {
                game.player_action_and_turn(Action {
                    direction: Relative(F),
                    action: SubAction::Use(2),
                })
            }))),
            KeyCode::Char('4') => Some(GameFn(Box::new(|game: &mut Game| {
                game.player_action_and_turn(Action {
                    direction: Relative(F),
                    action: SubAction::Use(3),
                })
            }))),
            KeyCode::Char('5') => Some(GameFn(Box::new(|game: &mut Game| {
                game.player_action_and_turn(Action {
                    direction: Relative(F),
                    action: SubAction::Use(4),
                })
            }))),
            KeyCode::Char('t') => Some(GameFn(Box::new(|game: &mut Game| {
                game.player_action_and_turn(Action {
                    direction: Relative(F),
                    action: SubAction::Take,
                })
            }))),
            KeyCode::Char('c') => Some(Craft),
            KeyCode::Esc => Some(Exit),
            _ => None,
        }
    }

    fn call(&mut self, term: &mut DefaultTerminal) {
        loop {
            term.draw(|frame| self.draw(frame)).unwrap();

            match self.read() {
                None => {}
                Some(GameFn(fun)) => fun(&mut self.game.borrow_mut()).unwrap(),
                Some(Craft) => {
                    let mut cmenu = CraftingMenu::new(self, self.game.clone());
                    cmenu.call(term);
                }
                Some(Exit) => break,
            }
        }
    }
}
