//! A player or npc.
use crate::datatypes::{Recording, Coordinate};
use crate::game_state::db::{ActorId, RecordingId};
use crate::direction::AbsoluteDirection;
use crate::inventory::BasicInventory;

#[derive(PartialEq, Debug)]
#[derive(Copy, Clone)]
pub struct Actor {
    pub facing: AbsoluteDirection,
    pub isplayer: bool,
    pub actor_id: ActorId,
    pub inventory: BasicInventory,
}

// A way to locate an actor within a world.
// This could be done more cleanly with references,
// but a planned time-travel mechanic would make 
// normal references impossible to reason about.
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
