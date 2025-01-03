use hashbrown::HashMap;

use crate::actor::{Actor, ActorRef};
use crate::datatypes::Recording;
use crate::error::{Result, Status::StateUpdateError};
use std::collections::HashSet;
use std::usize;

pub struct RecordingDb {
    recordings: Vec<Recording>,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct RecordingId {
    idx: usize,
}

impl RecordingId {
    pub const DEFAULT: RecordingId = RecordingId { idx: 0 };
}

impl RecordingDb {
    pub fn register_recording(&mut self, recording: &Recording) -> RecordingId {
        self.recordings.push(recording.clone());
        RecordingId {
            idx: self.recordings.len() - 1,
        }
    }

    pub fn new() -> RecordingDb {
        let mut db = RecordingDb {
            recordings: Vec::new(),
        };
        db.recordings.push(Recording::blank());
        db
    }

    pub fn get(&self, id: RecordingId) -> &Recording {
        &self.recordings[id.idx]
    }
}

pub struct ActorDb {
    actors: Vec<ActorRef>,
}
#[derive(Debug)]
pub struct ActorDbUpdate {
    changes: Vec<ActorId>,
    new_actors: Vec<ActorId>,
    map: HashMap<ActorId, ActorRef>,
}

impl ActorDbUpdate {
    pub fn peek_new_actors(&self) -> &Vec<ActorId> {
        &self.new_actors
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct ActorId {
    idx: usize,
}

impl ActorId {
    pub const DEFAULT: ActorId = ActorId { idx: 0 };
}

impl ActorDb {
    pub fn mut_register_actor(&mut self, actor: ActorRef) -> ActorId {
        let idx = self.actors.len();
        self.actors.push(actor);
        ActorId { idx: idx }
    }

    pub fn get_actor(&self, id: ActorId) -> ActorRef {
        self.actors[id.idx]
    }

    pub fn get_mut_actor(&mut self, id: ActorId) -> &mut ActorRef {
        &mut self.actors[id.idx]
    }

    pub fn new() -> ActorDb {
        let mut db = ActorDb { actors: Vec::new() };
        db.mut_register_actor(ActorRef::blank());
        db
    }

    pub fn register_actor(&self, update: &mut ActorDbUpdate, actor: ActorRef) -> ActorId {
        let idx = self.actors.len() + update.new_actors.len();
        let new_id = ActorId { idx: idx };
        update.new_actors.push(new_id);
        update.map.insert(new_id, actor);
        new_id
    }


    pub fn read_actor<'a> (&self, update: &'a mut ActorDbUpdate, id: &ActorId) -> Option<&'a mut ActorRef> {
        if update.map.contains_key(id) {
            update.map.get_mut(id)
        } else {
            update.changes.push(*id);
            update.map.insert(*id, self.get_actor(*id));
            update.map.get_mut(id)
        }
    }

    pub fn read_actors<'a, const N: usize>(&self, update: &'a mut ActorDbUpdate, ids: [&ActorId;N]) ->[Option<&'a mut ActorRef>;N] {
        for id in ids {
            self.read_actor(update, &id);
        }
        update.map.get_many_mut(ids)
    }

    pub fn new_update(&self) -> ActorDbUpdate {
        ActorDbUpdate {
            changes: Vec::new(),
            new_actors: Vec::new(),
            map: HashMap::new(),
        }
    }

    pub fn apply_update(&mut self, update: &ActorDbUpdate) -> Result<()> {
        let mut ids_set: HashSet<ActorId> = HashSet::new();

        for id in &update.changes {
            if id.idx >= self.actors.len() {
                return Err(StateUpdateError);
            }
            if !ids_set.insert(*id) {
                return Err(StateUpdateError);
            }
            self.actors[id.idx] = update.map[id];
        }

        for id in &update.new_actors {
            if id.idx < self.actors.len() {
                return Err(StateUpdateError);
            }
            if !ids_set.insert(*id) {
                return Err(StateUpdateError);
            }
            self.actors.push( update.map[id]);
        }
        Ok(())
    }
}
