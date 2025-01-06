use crate::db::RecordingId;


#[derive(PartialEq, Debug)]
#[derive(Clone, Copy)]
pub struct Item {
    pub name: &'static str,
    pub quantity: u16,
    pub recording: Option<RecordingId>
}

impl Item {
    pub fn new(name: &'static str, quantity: u16) -> Item {
        Item {
            name,
            quantity,

            recording: None,
        }
    }

    pub fn new_cloner(name: &'static str, recordingid: RecordingId) -> Item {
        Item {
            name,
            quantity: 1,
            recording: Some(recordingid),
        }
    }
}

