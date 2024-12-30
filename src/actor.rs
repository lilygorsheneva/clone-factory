use crate::direction::AbsoluteDirection;
use std::rc::Rc;
use crate::datatypes::{ActionQueue, Coordinate,Item};
use crate::action::Action;

// A recording will probably be a partially-defined actor.
#[derive(Clone)]
pub  struct Actor {
    pub facing: AbsoluteDirection,
    isplayer: bool,
    command_list: ActionQueue,
    command_idx: usize,
    inventory: Rc<Vec<Item>>,
    equipment: Rc<Vec<Item>>,
}


pub struct ActorRef {
    pub location: Coordinate
}

pub struct PlayerRef {
    pub actor_ref: ActorRef,
    pub current_recording: Vec<Action>
}

impl Actor {
    pub fn get_action(&self) -> &Action{
    match self.isplayer{
        false => &self.command_list.q[self.command_idx],
        true =>  panic!("Called get_action on player entity.")
        }
    }
    pub fn new() -> Actor{
        Actor {
            facing: AbsoluteDirection::N,
            isplayer: false,
            command_list: ActionQueue{q: Vec::new().into()},
            command_idx: 0,
            inventory: Vec::new().into(),
            equipment: Vec::new().into()            
        }
    }

    pub fn new_player() -> Actor{
        Actor {
            facing: AbsoluteDirection::N,
            isplayer: true,
            command_list: ActionQueue{q: Vec::new().into()},
            command_idx: 0,
            inventory: Vec::new().into(),
            equipment: Vec::new().into()            
        }
    }
}