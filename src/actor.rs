//! A player or npc.
use crate::direction::AbsoluteDirection;
use crate::engine::tracking_worldlayer::{Trackable, TrackableId};
use crate::game_state::db::ActorId;
use crate::inventory::BasicInventory;
use crate::recording::Recording;
use crate::static_data::ObjectDescriptor;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Actor {
    pub facing: AbsoluteDirection,
    pub descriptor: &'static ObjectDescriptor,
    pub actor_id: ActorId,
    pub inventory: BasicInventory,
    pub paradox_level: f64,
}

impl Actor {
    pub fn new(descriptor: &'static ObjectDescriptor) -> Actor {
        Actor {
            facing: AbsoluteDirection::N,
            descriptor,
            actor_id: ActorId::DEFAULT,
            inventory: Default::default(),
            paradox_level: 0.0,
        }
    }

    pub fn from_recording(descriptor: &'static ObjectDescriptor, recording: &Recording) -> Actor {
        Actor {
            facing: AbsoluteDirection::N,
            descriptor,
            actor_id: ActorId::DEFAULT,
            inventory: recording.inventory,
            paradox_level: 0.0,
        }
    }
}

impl Trackable for Actor {
    fn get_id(&self) -> Option<crate::engine::tracking_worldlayer::TrackableId> {
        Some(TrackableId(self.actor_id.idx))
    }
}

