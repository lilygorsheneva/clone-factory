//! Functions related to processing player input.
//! Any backend-specific input actions (crossterm) should be limited to this module.
use crate::action::{Action, SubAction};
use crate::direction::{
    AbsoluteDirection::{E, N, S, W},
    Direction::{Absolute, Relative},
    RelativeDirection::F,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

pub enum InputResult {
    Act(Action),
    Craft,
    Redraw,
    Exit,
    Record,
    Numeral(u8),
}

fn event_to_act(event: KeyEvent) -> Option<InputResult> {
    match event.code {
        KeyCode::Left => Some(InputResult::Act(Action {
            direction: Absolute(W),
            action: SubAction::Move,
        })),
        KeyCode::Right => Some(InputResult::Act(Action {
            direction: Absolute(E),
            action: SubAction::Move,
        })),
        KeyCode::Up => Some(InputResult::Act(Action {
            direction: Absolute(N),
            action: SubAction::Move,
        })),
        KeyCode::Down => Some(InputResult::Act(Action {
            direction: Absolute(S),
            action: SubAction::Move,
        })),
        KeyCode::Char('t') => Some(InputResult::Act(Action {
            direction: Relative(F),
            action: SubAction::Take,
        })),
        KeyCode::Char('1') => Some(InputResult::Act(Action {
            direction: Relative(F),
            action: SubAction::Use(0),
        })),
        KeyCode::Char('2') => Some(InputResult::Act(Action {
            direction: Relative(F),
            action: SubAction::Use(1),
        })),
        KeyCode::Char('3') => Some(InputResult::Act(Action {
            direction: Relative(F),
            action: SubAction::Use(2),
        })),
        KeyCode::Char('4') => Some(InputResult::Act(Action {
            direction: Relative(F),
            action: SubAction::Use(3),
        })),
        KeyCode::Char('5') => Some(InputResult::Act(Action {
            direction: Relative(F),
            action: SubAction::Use(4),
        })),
        KeyCode::Char('c') => Some(InputResult::Craft),
        KeyCode::Char('r') => Some(InputResult::Record),
        _ => None,
    }
}

pub fn read_numeral() -> Option<InputResult> {
    match event::read() {
        Ok(Event::Key(KeyEvent{code: KeyCode::Char('1'),..})) => Some(InputResult::Numeral(1)),
        Ok(Event::Key(KeyEvent{code: KeyCode::Char('2'),..})) => Some(InputResult::Numeral(2)),
        Ok(Event::Key(KeyEvent{code: KeyCode::Char('3'),..})) => Some(InputResult::Numeral(3)),
        Ok(Event::Key(KeyEvent{code: KeyCode::Char('4'),..})) => Some(InputResult::Numeral(4)),
        Ok(Event::Key(KeyEvent{code: KeyCode::Char('5'),..})) => Some(InputResult::Numeral(5)),
        Ok(Event::Key(KeyEvent{code: KeyCode::Char('6'),..})) => Some(InputResult::Numeral(6)),
        Ok(Event::Key(KeyEvent{code: KeyCode::Char('7'),..})) => Some(InputResult::Numeral(7)),
        Ok(Event::Key(KeyEvent{code: KeyCode::Char('8'),..})) => Some(InputResult::Numeral(8)),
        Ok(Event::Key(KeyEvent{code: KeyCode::Char('9'),..})) => Some(InputResult::Numeral(9)),
        Ok(Event::Key(KeyEvent{code: KeyCode::Char('0'),..})) => Some(InputResult::Numeral(0)),
        _ => None,
    }
}

pub fn readinput() -> Option<InputResult> {
    match event::read() {
        Ok(Event::Key(event)) if event.kind == KeyEventKind::Release => None,
        Ok(Event::Key(event)) if event.code == KeyCode::Esc => Some(InputResult::Exit),
        Ok(Event::Key(event)) => event_to_act(event),
        Ok(Event::Resize(_, _)) => Some(InputResult::Redraw),
        _ => None,
    }
}

// pub fn select_inventory_slot
// pub fn select_recipe
