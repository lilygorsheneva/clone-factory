use crate::action::{Action, SubAction};
use crate::datatypes::Recording;
use crate::direction::RelativeDirection;

pub fn make_sample_recording() -> Recording {
    Recording {
        command_list: vec![
            Action {
                direction: crate::direction::Direction::Relative(RelativeDirection::F),
                action: SubAction::Move,
            },
            Action {
                direction: crate::direction::Direction::Relative(RelativeDirection::R),
                action: SubAction::Move,
            },
            Action {
                direction: crate::direction::Direction::Relative(RelativeDirection::R),
                action: SubAction::Move,
            },
            Action {
                direction: crate::direction::Direction::Relative(RelativeDirection::R),
                action: SubAction::Move,
            },
            Action {
                direction: crate::direction::Direction::Relative(RelativeDirection::R),
                action: SubAction::Move,
            },
        ],

        equipment: Vec::new(),
    }
}
