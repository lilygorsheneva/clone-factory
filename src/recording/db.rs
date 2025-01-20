//! Datastore for recordings.

// This probably doesn't need an Updateable, as only the player will ever modify it
// and therefore will never be multithreaded.

use super::Recording;


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
    pub fn register_recording(&mut self, recording: Recording) -> RecordingId {
        self.recordings.push(recording);
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