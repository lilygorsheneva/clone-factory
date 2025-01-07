//! Item and Inventory datatype definitions.

use crate::game_state::db::RecordingId;
use crate::error::{Result, Status};
use crate::static_data::ItemDefiniton;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Item<'pseudostatic> {
    pub definition: &'pseudostatic ItemDefiniton,
    pub quantity: u16,
    pub recording: Option<RecordingId>,
}

impl<'pseudostatic> Item<'pseudostatic> {
    pub fn new(definition: &'pseudostatic ItemDefiniton, quantity: u16) -> Item<'pseudostatic> {
        Item {
            definition,
            quantity,
            recording: None,
        }
    }

    pub fn new_cloner(definition: &'pseudostatic ItemDefiniton,  recordingid: RecordingId) -> Item<'pseudostatic> {
        Item {
            definition,
            quantity: 1,
            recording: Some(recordingid),
        }
    }
}

// TODO: inventory trait

// A container for Items.
#[derive(PartialEq, Copy, Clone, Debug, Default)]
pub struct BasicInventory<'ps> {
    items: [Option<Item<'ps>>; 5],
}

impl<'ps> BasicInventory<'ps> {
    // When made generic, this should be a vec or something
    pub fn get_items(&self) -> &[Option<Item>; 5] {
        &self.items
    }

    pub fn insert(&mut self, new_item: Item<'ps>) -> Result<()> {
        for i in &mut self.items {
            if let Some(existing_item) = i {
                if existing_item.definition == new_item.definition {
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
                if existing_item.definition == target_item.definition {
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
