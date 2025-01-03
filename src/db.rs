use crate::actor::ActorRef;
use crate::datatypes::Recording;
use crate::error::{Result, Status::StateUpdateError};
use std::collections::HashSet;

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
    changes: Vec<(ActorId, ActorRef)>,
    new_actors: Vec<(ActorId, ActorRef)>,
}

impl ActorDbUpdate {
    pub fn peek_new_actors(&self) -> &Vec<(ActorId, ActorRef)> {
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
        update.new_actors.push((new_id, actor));
        new_id
    }

    pub fn update_actor(&self, update: &mut ActorDbUpdate, id: ActorId, actor: ActorRef) {
        update.changes.push((id, actor));
    }

    pub fn new_update(&self) -> ActorDbUpdate {
        ActorDbUpdate {
            changes: Vec::new(),
            new_actors: Vec::new(),
        }
    }

    pub fn apply_update(&mut self, update: &ActorDbUpdate) -> Result<()> {
        let mut ids_set: HashSet<ActorId> = HashSet::new();

        for (id, actor_ref) in &update.changes {
            if id.idx >= self.actors.len() {
                return Err(StateUpdateError);
            }
            if !ids_set.insert(*id) {
                return Err(StateUpdateError);
            }
            self.actors[id.idx] = *actor_ref;
        }

        for (id, actor_ref) in &update.new_actors {
            if id.idx < self.actors.len() {
                return Err(StateUpdateError);
            }
            if !ids_set.insert(*id) {
                return Err(StateUpdateError);
            }
            self.actors.push(*actor_ref);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::datatypes::Coordinate;

    use super::*;

    #[test]
    fn reject_overlap() {
        let mut db = ActorDb::new();
        let mut update = db.new_update();
        let actor = ActorRef::new(Coordinate{x:0, y:0}, crate::direction::AbsoluteDirection::N);

        let id = db.register_actor(&mut update,actor);
        db.update_actor(&mut update, id, actor);
        
        assert!(db.apply_update(&update).is_err())
        
    }
}
    