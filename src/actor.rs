use crate::datatypes::{Recording, Coordinate, Item};
use crate::db::{ActorId, RecordingId};
use crate::direction::AbsoluteDirection;

#[derive(PartialEq, Debug)]
#[derive(Copy, Clone)]
pub struct Actor {
    pub facing: AbsoluteDirection,
    pub isplayer: bool,
    pub actor_id: ActorId,
    pub inventory: [Option<Item>; 5],
    //equipment:[Option<Item>; 1],
}

#[derive(Copy, Clone, Debug)]
pub struct ActorRef {
    pub location: Coordinate,
    pub orientation: AbsoluteDirection,
    pub isplayer: bool,
    pub live: bool,
    pub recording: RecordingId,
    pub command_idx: usize,
}

impl ActorRef {
    pub fn blank() -> ActorRef {
        ActorRef {
            location: Coordinate { x: 0, y: 0 },
            orientation: AbsoluteDirection::N,
            live: false,
            isplayer: false,
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
            live: true,
            isplayer: false,
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
        }
    }

    pub fn from_recording(recording: &Recording) -> Actor {
        Actor {
            facing: AbsoluteDirection::N,
            isplayer: false,
            actor_id: ActorId::DEFAULT,
            inventory:recording.inventory,
        }
    }

    pub fn new_player() -> Actor {
        let mut actor = Actor::new();
        actor.isplayer = true;
        actor
    }
}
