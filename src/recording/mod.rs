//! A Recording is a sequence of replayable actions.

use crate::{action::Action, actor::Actor, inventory::BasicInventory};

pub mod db;
pub mod interface;

#[derive(Clone)]
pub struct Recording {
    pub command_list: Vec<Action>,
    pub inventory: BasicInventory,
    pub should_loop: bool
}

impl Recording {
    pub fn blank() -> Recording {
        Recording {
            command_list: Vec::new(),
            inventory: Default::default(),
            should_loop: false,
        }
    }

    pub fn from_creator(actor: &Actor) -> Recording {
        Recording {
            command_list: Vec::new(),
            inventory: actor.inventory,
            should_loop: false
        }
    }

    pub fn at(&self, idx: usize) -> Action {
        self.command_list[idx % self.command_list.len()]
    }

    pub fn append(&mut self, action: Action) {
        self.command_list.push(action);
    }
}

