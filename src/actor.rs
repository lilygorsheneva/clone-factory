//! A player or npc.
use crate::direction::AbsoluteDirection;
use crate::engine::tracking_worldlayer::{Trackable, TrackableId};
use crate::inventory::BasicInventory;
use crate::recording::Recording;
use crate::static_data::ObjectDescriptor;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Actor {
    pub facing: AbsoluteDirection,
    pub descriptor: &'static ObjectDescriptor,
    pub actor_id: TrackableId,
    pub inventory: BasicInventory,
    pub paradox_level: f64,
}

impl Actor {
    pub fn new(descriptor: &'static ObjectDescriptor, actor_id: TrackableId) -> Actor {
        Actor {
            facing: AbsoluteDirection::N,
            descriptor,
            actor_id: actor_id,
            inventory: Default::default(),
            paradox_level: 0.0,
        }
    }

    pub fn from_recording(descriptor: &'static ObjectDescriptor, actor_id: TrackableId, recording: &Recording) -> Actor {
        Actor {
            facing: AbsoluteDirection::N,
            descriptor,
            actor_id: actor_id,
            inventory: recording.inventory,
            paradox_level: 0.0,
        }
    }
}

impl Trackable for Actor {
    fn get_id(&self) -> Option<crate::engine::tracking_worldlayer::TrackableId> {
        Some(self.actor_id)
    }
}

