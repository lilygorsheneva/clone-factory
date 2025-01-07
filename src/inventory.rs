//! Item and Inventory datatype definitions.

use crate::db::RecordingId;
use crate::error::{Result, Status};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Item {
    pub id: i64,
    pub quantity: u16,
    pub recording: Option<RecordingId>,
}

impl Item {
    pub fn new(id: i64, quantity: u16) -> Item {
        Item {
            id,
            quantity,
            recording: None,
        }
    }

    pub fn new_cloner(id: i64,  recordingid: RecordingId) -> Item {
        Item {
            id,
            quantity: 1,
            recording: Some(recordingid),
        }
    }
}

// TODO: inventory trait

// A container for Items.
#[derive(PartialEq, Copy, Clone, Debug, Default)]
pub struct BasicInventory {
    items: [Option<Item>; 5],
}

impl BasicInventory {
    // When made generic, this should be a vec or something
    pub fn get_items(&self) -> &[Option<Item>; 5] {
        &self.items
    }

    pub fn insert(&mut self, new_item: Item) -> Result<()> {
        for i in &mut self.items {
            if let Some(existing_item) = i {
                if existing_item.id == new_item.id {
                    existing_item.quantity += new_item.quantity;
                    return Ok(());
                }
            }
        }
        for i in &mut self.items {
            if i.is_none() {
                *i = Some(new_item);
                return Ok(());
            }
        }
        Err(Status::ActionFail("no space in inventory"))
    }

    pub fn remove(&mut self, target_item: Item) -> Result<()> {
        for i in &mut self.items {
            if let Some(existing_item) = i {
                if existing_item.id == target_item.id {
                    if existing_item.quantity > target_item.quantity {
                        existing_item.quantity -= target_item.quantity;
                        return Ok(());
                    } else if existing_item.quantity == target_item.quantity {
                        *i = None;
                        return Ok(());
                    } else {
                        return Err(Status::ActionFail("attempting to remove too many items"));
                    }
                }
            }
        }
        Err(Status::ActionFail("no such item in inventory"))
    }
}
