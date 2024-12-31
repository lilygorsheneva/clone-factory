use crate::datatypes::{Coordinate, Item};
use crate::db::{ActorId, RecordingId};
use crate::direction::AbsoluteDirection;

// A recording will probably be a partially-defined actor.
#[derive(Clone)]
pub struct Actor {
    pub facing: AbsoluteDirection,
    pub isplayer: bool,
    pub actor_id: ActorId,
    pub inventory: [Option<Item>; 5],
    //equipment:[Option<Item>; 1],
}

#[derive(Copy, Clone)]
pub struct ActorRef {
    pub location: Coordinate,
    pub orientation: AbsoluteDirection,
    pub liveness: bool,
    pub recording: RecordingId,
    pub command_idx: usize,
}

impl ActorRef {
    pub fn blank() -> ActorRef {
        ActorRef {
            location: Coordinate { x: 0, y: 0 },
            orientation: AbsoluteDirection::N,
            liveness: false,
            recording: RecordingId::DEFAULT,
            command_idx: 0,
        }
    }
}

impl ActorRef {
    pub fn new(coordinate: Coordinate, orientation: AbsoluteDirection) -> ActorRef {
        ActorRef {
            location: coordinate,
            orientation: orientation,
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
            inventory: Default::default(),
            //equipment: Default::default(),
        }
    }

    pub fn new_player() -> Actor {
        Actor {
            facing: AbsoluteDirection::N,
            isplayer: true,
            actor_id: ActorId::DEFAULT,
            inventory: Default::default(),
            //equipment: Default::default(),
        }
    }
}
