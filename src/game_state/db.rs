//! Containers for non-spatial data.
use std::collections::HashMap;

use crate::error::{Result, Status::StateUpdateError};
use std::collections::HashSet;
use std::usize;


#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct ActorId {
    pub idx: usize,
}

impl ActorId {
    pub const DEFAULT: ActorId = ActorId { idx: 0 };
}

