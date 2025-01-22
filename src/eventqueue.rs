use std::collections::VecDeque;

use crate::{action::Action, game_state::db::ActorId, recording::db::RecordingId};
use  crate::error::{Status, Result};

#[derive(Debug)]

pub struct Event{
    pub actor: ActorId,

}

#[derive(Debug)]

pub struct EventQueue{
    pub this_turn: VecDeque<Event>,
    pub next_turn: VecDeque<Event>
}

#[derive(Debug)]
pub struct EventQueueUpdate{
    pub this_turn: VecDeque<Event>,
    pub next_turn: VecDeque<Event>
}

impl EventQueueUpdate {
    pub fn new() -> Self {
        Self {
            this_turn: VecDeque::new(),
            next_turn: VecDeque::new()
        }
    }

    pub fn apply(self, target: &mut EventQueue) -> Result<()> {
        target.this_turn.extend(self.this_turn);
        target.next_turn.extend(self.next_turn);
        Ok(())
    } 
}


impl EventQueue {
    pub  fn new() -> Self {
        Self {
            this_turn: VecDeque::new(),
            next_turn: VecDeque::new()
        }
    }


pub fn get_next_event(&mut self) -> Option<Event> {
    self.this_turn.pop_front()
}

pub fn advance_turn(&mut self) -> Result<()>{
    if !self.this_turn.is_empty() {return  Err(Status::StateUpdateError)};
    std::mem::swap(&mut self.this_turn, &mut self.next_turn);
    Ok(())
}

}