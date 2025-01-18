use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{DefaultTerminal, Frame};

use crate::{
    action::{Action, SubAction},
    game_state::game::Game,
    interface::render,
};

use crate::direction::{
    AbsoluteDirection::{E, N, S, W},
    Direction::{Absolute, Relative},
    RelativeDirection::F,
};

use super::{GameFn, MenuTrait};

pub struct GameMenu<'a> {
    game: &'a mut Game,
}

pub enum GameMenuOptions {
    Exit,
    GameFn(Box<GameFn>),
    Craft,
}
use GameMenuOptions::*;

impl GameMenu<'_> {
    pub fn new(game: &mut Game) -> GameMenu {
        GameMenu { game }
    }
}

impl MenuTrait for GameMenu<'_> {
    type MenuOptions = GameMenuOptions;

    fn draw(&self, frame: &mut Frame) {
        render::draw_game_window(self.game, frame);
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

            KeyCode::Esc => Some(Exit),
            _ => None,
        }
    }

    fn call(&mut self, term: &mut DefaultTerminal) {
        loop {
            term.draw(|frame| self.draw(frame)).unwrap();

            match self.read() {
                None => {}
                Some(GameFn(fun)) => fun(self.game).unwrap(),
                Some(Craft) => {}
                Some(Exit) => break,
            }
        }
    }
}
