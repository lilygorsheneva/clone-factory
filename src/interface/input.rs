//! Functions related to processing player input.
//! Any backend-specific input actions (crossterm) should be limited to this module.
use crate::action::{Action, SubAction};
use crate::direction::{
    AbsoluteDirection::{E, N, S, W},
    Direction::{Absolute, Relative},
    RelativeDirection::F,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::widgets::{List, ListItem};

#[derive(Clone, Copy)]
pub enum InputResult {
    Act(Action),
    Exit,
    Record,
    None,
}


pub fn readinput(menu: &Menu) -> InputResult {
    match event::read() {
        Ok(Event::Key(event)) => menu.decode(event),
        _ => InputResult::None,
    }
}

pub fn normal_menu() -> Menu {
    Menu {
        options: vec![
            MenuOption::new(
                KeyCode::Left,
                KeyModifiers::NONE,
                "move",
                InputResult::Act(Action {
                    direction: Absolute(W),
                    action: SubAction::Move,
                }),
            ),
            MenuOption::new(
                KeyCode::Right,
                KeyModifiers::NONE,
                "move",
                InputResult::Act(Action {
                    direction: Absolute(E),
                    action: SubAction::Move,
                }),
            ),
            MenuOption::new(
                KeyCode::Up,
                KeyModifiers::NONE,
                "move",
                InputResult::Act(Action {
                    direction: Absolute(N),
                    action: SubAction::Move,
                }),
            ),
            MenuOption::new(
                KeyCode::Down,
                KeyModifiers::NONE,
                "move",
                InputResult::Act(Action {
                    direction: Absolute(S),
                    action: SubAction::Move,
                }),
            ),
            MenuOption::new(
                KeyCode::Char('t'),
                KeyModifiers::NONE,
                "take",
                InputResult::Act(Action {
                    direction: Relative(F),
                    action: SubAction::Take,
                })
            ),
            MenuOption::new(
                KeyCode::Char('1'),
                KeyModifiers::NONE,
                "Use Item 1",
                InputResult::Act(Action {
                    direction: Relative(F),
                    action: SubAction::Use(0),
                })
            ),
            MenuOption::new(
                KeyCode::Char('2'),
                KeyModifiers::NONE,
                "Use Item 2",
                InputResult::Act(Action {
                    direction: Relative(F),
                    action: SubAction::Use(1),
                })
            ),
            MenuOption::new(
                KeyCode::Char('3'),
                KeyModifiers::NONE,
                "Use Item 3",
                InputResult::Act(Action {
                    direction: Relative(F),
                    action: SubAction::Use(2),
                })
            ),
            MenuOption::new(
                KeyCode::Char('4'),
                KeyModifiers::NONE,
                "Use Item 4",
                InputResult::Act(Action {
                    direction: Relative(F),
                    action: SubAction::Use(3),
                })
            ),
            MenuOption::new(
                KeyCode::Char('5'),
                KeyModifiers::NONE,
                "Use Item 5",
                InputResult::Act(Action {
                    direction: Relative(F),
                    action: SubAction::Use(4),
                })
            ),
            MenuOption::new(
                KeyCode::Char('r'),
                KeyModifiers::NONE,
                "Record",
                InputResult::Record
            ),
            MenuOption::new(
                KeyCode::Esc,
                KeyModifiers::NONE,
                "Quit",
                InputResult::Exit
            ),
        ],
    }
}

pub struct MenuOption {
    pub key: KeyEvent,
    pub description: &'static str,
    pub outcome: InputResult,
}

impl MenuOption {
    fn new(
        code: KeyCode,
        modifiers: KeyModifiers,
        description: &'static str,
        outcome: InputResult,
    ) -> MenuOption {
        MenuOption {
            key: KeyEvent::new(code, modifiers),
            description,
            outcome,
        }
    }
}

pub struct Menu {
    pub options: Vec<MenuOption>,
}

impl Menu {
    pub fn decode(&self, event: KeyEvent) -> InputResult {
        for option in &self.options {
            if option.key == event {
                return option.outcome;
            }
        }
        InputResult::None
    }
}

// pub fn select_inventory_slot
// pub fn select_recipe
