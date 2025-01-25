use std::{cell::RefCell, rc::Rc};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};

use crate::{
    action::{Action, SubAction},
    error::OkOrPopup,
    game_state::game::Game,
    interface::widgets::{generate_main_layout, ItemBar, WorldWindowWidget},
    recording::interface::RecordingMenu,
};

use crate::direction::{
    AbsoluteDirection::{E, N, S, W},
    Direction::{Absolute, Relative},
    RelativeDirection::F,
};

use super::{craftingmenu::CraftingMenu, GameFn, MenuTrait, UILayer};

pub struct GameMenu {
    game: Rc<RefCell<Game>>,
}

pub enum GameMenuOptions {
    Exit,
    GameFn(Box<GameFn>),
    Craft,
    Record,
}
use GameMenuOptions::*;

impl GameMenu {
    pub fn new(game: Rc<RefCell<Game>>) -> GameMenu {
        GameMenu { game }
    }
}

impl UILayer for GameMenu {
    fn draw(&self, frame: &mut Frame) {
        let game = self.game.borrow();
        let window = WorldWindowWidget::new(&game);
        let item_widget = ItemBar::new(&game);
        let score = &game.score;

        let (main, _side, bottom, corner) = generate_main_layout(frame);

        frame.render_widget(item_widget, bottom);
        frame.render_widget(window, main);
        frame.render_widget(score, corner);
    }
}

impl MenuTrait for GameMenu {
    type MenuOptions = GameMenuOptions;

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
            KeyCode::Char('r') => Some(Record),
            KeyCode::Char('1') if key.modifiers == KeyModifiers::NONE => {
                Some(GameFn(Box::new(|game: &mut Game| {
                    game.player_action_and_turn(Action {
                        direction: Relative(F),
                        action: SubAction::Use(0),
                    })
                })))
            }
            KeyCode::Char('1') if key.modifiers.contains(KeyModifiers::ALT) => {
                Some(GameFn(Box::new(|game: &mut Game| {
                    game.player_action_and_turn(Action {
                        direction: Relative(F),
                        action: SubAction::Drop(0),
                    })
                })))
            }
            KeyCode::Char('2') if key.modifiers == KeyModifiers::NONE => {
                Some(GameFn(Box::new(|game: &mut Game| {
                    game.player_action_and_turn(Action {
                        direction: Relative(F),
                        action: SubAction::Use(1),
                    })
                })))
            }
            KeyCode::Char('2') if key.modifiers.contains(KeyModifiers::ALT) => {
                Some(GameFn(Box::new(|game: &mut Game| {
                    game.player_action_and_turn(Action {
                        direction: Relative(F),
                        action: SubAction::Drop(1),
                    })
                })))
            }
            KeyCode::Char('3') if key.modifiers == KeyModifiers::NONE => {
                Some(GameFn(Box::new(|game: &mut Game| {
                    game.player_action_and_turn(Action {
                        direction: Relative(F),
                        action: SubAction::Use(2),
                    })
                })))
            }
            KeyCode::Char('3') if key.modifiers.contains(KeyModifiers::ALT) => {
                Some(GameFn(Box::new(|game: &mut Game| {
                    game.player_action_and_turn(Action {
                        direction: Relative(F),
                        action: SubAction::Drop(2),
                    })
                })))
            }
            KeyCode::Char('4') if key.modifiers == KeyModifiers::NONE => {
                Some(GameFn(Box::new(|game: &mut Game| {
                    game.player_action_and_turn(Action {
                        direction: Relative(F),
                        action: SubAction::Use(3),
                    })
                })))
            }
            KeyCode::Char('4') if key.modifiers.contains(KeyModifiers::ALT) => {
                Some(GameFn(Box::new(|game: &mut Game| {
                    game.player_action_and_turn(Action {
                        direction: Relative(F),
                        action: SubAction::Drop(3),
                    })
                })))
            }
            KeyCode::Char('5') if key.modifiers == KeyModifiers::NONE => {
                Some(GameFn(Box::new(|game: &mut Game| {
                    game.player_action_and_turn(Action {
                        direction: Relative(F),
                        action: SubAction::Use(4),
                    })
                })))
            }
            KeyCode::Char('5') if key.modifiers.contains(KeyModifiers::ALT) => {
                Some(GameFn(Box::new(|game: &mut Game| {
                    game.player_action_and_turn(Action {
                        direction: Relative(F),
                        action: SubAction::Drop(4),
                    })
                })))
            }
            KeyCode::Char('t') => Some(GameFn(Box::new(|game: &mut Game| {
                game.player_action_and_turn(Action {
                    direction: Relative(F),
                    action: SubAction::Take,
                })
            }))),
            KeyCode::Char('u') => Some(GameFn(Box::new(|game: &mut Game| {
                game.player_action_and_turn(Action {
                    direction: Relative(F),
                    action: SubAction::ActivateBuilding,
                })
            }))),
            KeyCode::Char('c') => Some(Craft),
            KeyCode::Char(' ') => Some(GameFn(Box::new(|game: &mut Game| {
                game.player_action_and_turn(Action {
                    direction: Relative(F),
                    action: SubAction::Wait,
                })
            }))),
            KeyCode::Esc => Some(Exit),
            _ => None,
        }
    }

    fn enter_menu(&mut self, terminal: &mut DefaultTerminal) {
        loop {
            terminal.draw(|frame| self.draw(frame)).unwrap();

            match self.read() {
                None => {}
                Some(GameFn(fun)) => {
                    let res = fun(&mut self.game.borrow_mut());
                    res.ok_or_popup(self, terminal);
                }
                Some(Craft) => {
                    let mut cmenu = CraftingMenu::new(self, self.game.clone());
                    cmenu.enter_menu(terminal);
                }
                Some(Record) => {
                    let mut rmenu = RecordingMenu::new(self, self.game.clone());
                    rmenu.enter_menu(terminal);
                }

                Some(Exit) => break,
            }
        }
    }
}
