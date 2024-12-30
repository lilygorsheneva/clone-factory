use crate::action::Action;
use crate::datatypes::{ActionQueue, Coordinate, Item};
use crate::db::{ActorId, RecordingId};
use crate::direction::AbsoluteDirection;
use std::rc::Rc;

// A recording will probably be a partially-defined actor.
#[derive(Clone)]
pub struct Actor {
    pub facing: AbsoluteDirection,
    isplayer: bool,
    pub actor_id: ActorId,
    pub inventory: Vec<Item>,
    equipment: Vec<Item>,
}

pub struct ActorRef {
    pub location: Coordinate,
    pub liveness: bool,
    recording: RecordingId,
    command_idx: usize,
}

impl ActorRef {
    pub fn new(coordinate: Coordinate) -> ActorRef {
        ActorRef {
            location: coordinate,
            liveness: true,
            recording: RecordingId::DEFAULT,
            command_idx: 0,
        }
    }
}

impl Actor {
    pub fn new() -> Actor {
        Actor {
            facing: AbsoluteDirection::N,
            isplayer: false,
            actor_id: ActorId::DEFAULT,
            inventory: Vec::new().into(),
            equipment: Vec::new().into(),
        }
    }

    pub fn new_player() -> Actor {
        Actor {
            facing: AbsoluteDirection::N,
            isplayer: true,
            actor_id: ActorId::DEFAULT,
            inventory: Vec::new().into(),
            equipment: Vec::new().into(),
        }
    }
}
