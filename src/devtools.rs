use crate::action::{Action, SubAction};
use crate::datatypes::{ActionQueue, Recording};
use crate::direction::AbsoluteDirection;

pub fn make_sample_recording() -> Recording {
    Recording {
        command_list: ActionQueue {
            q: vec![
                Action {
                    direction: crate::direction::Direction::Absolute(AbsoluteDirection::N),
                    action: SubAction::Move,
                },
                Action {
                    direction: crate::direction::Direction::Absolute(AbsoluteDirection::W),
                    action: SubAction::Move,
                },
                Action {
                    direction: crate::direction::Direction::Absolute(AbsoluteDirection::S),
                    action: SubAction::Move,
                },
                Action {
                    direction: crate::direction::Direction::Absolute(AbsoluteDirection::E),
                    action: SubAction::Move,
                },
            ],
        },
        equipment: Vec::new(),
    }
}
