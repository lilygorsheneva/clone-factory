//! A player or npc.
use std::cmp::min;

use crate::action::Action;
use crate::datatypes::Coordinate;
use crate::direction::AbsoluteDirection;
use crate::engine::tracking_worldlayer::{Trackable, TrackableId};
use crate::engine::update::{self, Delta, UpdatableContainer, UpdatableContainerDelta};
use crate::error::{Result, Status::Error};
use crate::game_state::db::ActorId;
use crate::game_state::game::{Game, GameUpdate};
use crate::inventory::BasicInventory;
use crate::recording::{db::RecordingId, Recording};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Actor {
    pub facing: AbsoluteDirection,
    pub isplayer: bool,
    pub actor_id: ActorId,
    pub inventory: BasicInventory,
    pub paradox_level: f64,
}

impl Actor {
    pub fn new() -> Actor {
        Actor {
            facing: AbsoluteDirection::N,
            isplayer: false,
            actor_id: ActorId::DEFAULT,
            inventory: Default::default(),
            paradox_level: 0.0,
        }
    }

    pub fn from_recording(recording: &Recording) -> Actor {
        Actor {
            facing: AbsoluteDirection::N,
            isplayer: false,
            actor_id: ActorId::DEFAULT,
            inventory: recording.inventory,
            paradox_level: 0.0,
        }
    }

    pub fn new_player() -> Actor {
        let mut actor = Actor::new();
        actor.isplayer = true;
        actor
    }
}

impl Trackable for Actor {
    fn get_id(&self) -> Option<crate::engine::tracking_worldlayer::TrackableId> {
        Some(TrackableId(self.actor_id.idx))
    }
}

