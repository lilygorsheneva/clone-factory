
use crate::datatypes::Recording;
use crate::actor::ActorRef;

pub struct RecordingDb {
    recordings: Vec<Recording> 
}

#[derive(Copy,Clone)]
pub struct RecordingId {
    idx: usize,
}

impl RecordingId {
    pub const DEFAULT: RecordingId = RecordingId{idx: 0};
}


impl RecordingDb {
    pub fn register_recording(&mut self, recording: Recording) -> RecordingId{
        self.recordings.push(recording);
        RecordingId{idx: self.recordings.len()-1}
    }

    pub fn new() -> RecordingDb {
        RecordingDb{recordings: Vec::new()}
    }
}

pub struct ActorDb {
    actors: Vec<ActorRef>
}

#[derive(Clone, Copy)]
pub struct ActorId{
    idx: usize,
}

impl ActorId {
    pub const DEFAULT: ActorId = ActorId{idx: 0};
}

impl ActorDb {
    pub fn register_actor(&mut self, actor: ActorRef) -> ActorId{
        self.actors.push(actor);
        ActorId{idx: self.actors.len()-1}
    }

    pub fn get_actor(&self, id: ActorId) -> &ActorRef {
       &self.actors[id.idx]
    }


    pub fn get_mut_actor(&mut self, id: ActorId) -> &mut ActorRef {
        &mut self.actors[id.idx]
    }

    pub fn new() -> ActorDb {
        ActorDb{actors: Vec::new()}
    }

}