use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crate::datatypes::{AbsoluteDirection::{self, E, N, S, W}, Direction::Absolute, Action, SubAction};

pub enum InputResult {
    Act(Action),
    Redraw,
    Exit,
}

fn event_to_act(event: KeyEvent) -> Option<InputResult> {
match event.code {
    KeyCode::Left => Some(InputResult::Act(Action {direction: Absolute(W), action: SubAction::Move})),
    KeyCode::Right => Some(InputResult::Act(Action {direction: Absolute(E), action: SubAction::Move})),
    KeyCode::Up => Some(InputResult::Act(Action {direction: Absolute(N), action: SubAction::Move})),
    KeyCode::Down => Some(InputResult::Act(Action {direction: Absolute(S), action: SubAction::Move})),
    _ => None,
}
}

pub fn readinput() -> Option<InputResult> {
    match event::read() {
        Ok(Event::Key(event)) if event.code == KeyCode::Esc => Some(InputResult::Exit),
        Ok(Event::Key(event)) => event_to_act(event),
        Ok(Event::Resize(_, _)) => Some(InputResult::Redraw),
        _ => None,
    }
}
