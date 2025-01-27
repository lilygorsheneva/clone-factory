use std::{cell::RefCell, rc::Rc};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    widgets::{self, List, ListItem, Paragraph},
    DefaultTerminal, Frame,
};

use crate::{
    action::{Action, SubAction},
    error::OkOrPopup,
    game_state::{game::Game, world::WorldCell},
    interface::widgets::{generate_main_layout, ItemBar, WorldWindowWidget},
    recording::interface::RecordingMenu,
};

use crate::direction::{
    AbsoluteDirection::{E, N, S, W},
    Direction::{Absolute, Relative},
    RelativeDirection::F,
};

use super::{craftingmenu::CraftingMenu, lookmenu::{self, LookMenu}, GameFn, MenuTrait, UILayer};

pub struct GameMenu {
    game: Rc<RefCell<Game>>,
}

pub enum GameMenuOptions {
    Exit,
    GameFn(Box<GameFn>),
    Craft,
    Record,
    Look
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

        let cell = game.world.get_cell(game.get_player_coords().unwrap()).unwrap();

        let (main, side, bottom, corner) = generate_main_layout(frame);

        
        cell.render_as_list(side, frame.buffer_mut());
        //frame.render_widget(get_guide, side);

        frame.render_widget(item_widget, bottom);
        frame.render_widget(window, main);
        frame.render_widget(score, corner);
    }
}

fn get_guide() -> List<'static> {
    List::new([
        ListItem::new("arrow keys: move"),
        ListItem::new("Num keys: use item"),
        ListItem::new("Alt+Num keys: drop"),
        ListItem::new("T: take"),
        ListItem::new("U: interact with building"),
        ListItem::new("C: crafting menu"),
        ListItem::new("R: recording menu"),
        ListItem::new("L: Look"),
        ListItem::new("space: wait"),
        ])
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
            KeyCode::Char('l') => Some(Look),
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
                Some(Look) => {
                    let mut lmenu = LookMenu::new( self.game.clone());
                    lmenu.enter_menu(terminal);
                }
                Some(Exit) => break,
            }
        }
    }
}
