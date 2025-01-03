use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use crate::action::{Action, SubAction};
use crate::direction::{AbsoluteDirection::{E, N, S, W}, RelativeDirection::F, Direction::{Absolute,Relative}};

pub enum InputResult {
    Act(Action),
    Redraw,
    Exit,
    Record,
}

fn event_to_act(event: KeyEvent) -> Option<InputResult> {
match event.code {
    KeyCode::Left => Some(InputResult::Act(Action {direction: Absolute(W), action: SubAction::Move})),
    KeyCode::Right => Some(InputResult::Act(Action {direction: Absolute(E), action: SubAction::Move})),
    KeyCode::Up => Some(InputResult::Act(Action {direction: Absolute(N), action: SubAction::Move})),
    KeyCode::Down => Some(InputResult::Act(Action {direction: Absolute(S), action: SubAction::Move})),
    KeyCode::Char('t') => Some(InputResult::Act(Action {direction: Relative(F), action: SubAction::Take})),
    KeyCode::Char('u') => Some(InputResult::Act(Action {direction: Relative(F), action: SubAction::Use(1)})),
    KeyCode::Char('r') => Some(InputResult::Record),
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