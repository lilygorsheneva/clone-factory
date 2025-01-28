use std::collections::VecDeque;

use crate::engine::tracking_worldlayer::TrackableId;
use crate::engine::update::{Delta, Updatable};
use crate::recording::db::RecordingId;
use  crate::error::{Status, Result};

#[derive(Debug, Clone, Copy)]
pub struct ActorEvent{
    pub actor: TrackableId,
    pub recording: RecordingId,
    pub recording_idx: usize
}

#[derive(Debug)]

pub struct EventQueue{
    pub this_turn: VecDeque<ActorEvent>,
    pub next_turn: VecDeque<ActorEvent>
}

#[derive(Debug)]
pub struct EventQueueUpdate{
    pub this_turn: VecDeque<ActorEvent>,
    pub next_turn: VecDeque<ActorEvent>
}

impl Updatable for EventQueue{}

impl Delta for EventQueueUpdate {
    type Target = EventQueue;
    fn new() -> Self {
        Self {
            this_turn: VecDeque::new(),
            next_turn: VecDeque::new()
        }
    }

    fn apply(&self, target: &mut EventQueue) -> Result<()> {
        target.this_turn.extend(&self.this_turn);
        target.next_turn.extend(&self.next_turn);
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


pub fn get_next_event(&mut self) -> Option<ActorEvent> {
    self.this_turn.pop_front()
}

pub fn advance_turn(&mut self) -> Result<()>{
    if !self.this_turn.is_empty() {return  Err(Status::StateUpdateError)};
    std::mem::swap(&mut self.this_turn, &mut self.next_turn);
    Ok(())
}

}