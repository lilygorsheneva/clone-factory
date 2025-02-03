//! Recording-related APIs.

// TODO:
// A function that can be a plugin into player_action
// A function that can be a plugin into npc turns
// A menu
//  Start record (select and consume a blank recorder; different recorders have different features)
//  End record (select loop/die), spawns into inventory
//  View saved recordings
//  Duplicate saved recording (consumes a crystal of equal or greater tier.)

use std::{cell::RefCell, rc::Rc};

use crossterm::event::KeyCode;
use ratatui::{
    layout::{self, Constraint, Layout},
    widgets::{Block, Paragraph},
};

use super::{
    db::{RecordingDb, RecordingId},
    Recording,
};

use crate::{
    direction::{AbsoluteDirection, Direction},
    engine::update::{Delta, UpdatableContainer},
};

use crate::{
    action::Action,
    error::{
         Result,
        Status::{ActionFail, Error},
    },
};
use crate::{
    devtools,
    game_state::game::{Game, GameUpdate},
    inventory::Item,
};


// TODO: implement update struct so that functions operating on this don't need a refcell, maybe?
pub struct RecordingModule {
    pub recordings: RecordingDb,
    pub current_recording: Option<Recording>,
    pub temp_item: Option<Item>,
    last_player_facing: AbsoluteDirection,
}

impl RecordingModule {
    pub fn new() -> RecordingModule {
        RecordingModule {
            recordings: RecordingDb::new(),
            current_recording: None,
            temp_item: None,
            // Could pose an issue with save/load, as this is state. player must perform an action before recording anything.
            last_player_facing: AbsoluteDirection::N,
        }
    }

    pub fn get(&self, id: RecordingId) -> &Recording {
        &self.recordings.get(id)
    }

    pub fn append(&mut self, action: Action) {
        let mut rotated_action = action;

        match action.direction {
            Direction::Absolute(d) => {
                rotated_action.direction =
                    Direction::Relative(d.difference(&self.last_player_facing));
            }
            Direction::Relative(d) => {}
        }
        self.last_player_facing = self.last_player_facing.rotate(&action.direction);

        if let Some(rec) = self.current_recording.as_mut() {
            rec.append(rotated_action);
        }
    }

    pub fn load_recording(&mut self, recording: Recording) -> RecordingId {
        self.recordings.register_recording(recording)
    }

    // Start recording.
    pub fn init_record(game: &mut Game, idx: usize) -> Result<()> {
        if game.recordings.current_recording.is_some() {
            return Err(Error("Attempted to initialize recording twice"));
        }

        let mut player = game.get_player_actor().cloned()?;
        let coords = *game.get_player_coords()?;

        let item = player
            .inventory
            .remove_idx(idx)
            .ok_or(ActionFail("No item in slot"))?;
        // TODO clean up this check.
        if item.definition.name != "recorder" {
            return Err(ActionFail("Item is not an empty recorder"));
        }

        game.world.actors.mut_set(&coords, &Some(player))?;
        game.recordings.current_recording = Some(Recording::from_creator(&player));
        Ok(())
    }

    // End recording.
    pub fn end_record(game: &mut Game, should_loop: bool) -> Result<()> {
        let recording = game
            .recordings
            .current_recording
            .as_ref()
            .ok_or(Error("Called end_record without a recording"))?;
        let cloner_def = game
            .data
            .items
            .get(&"basic_cloner".to_string())
            .ok_or(Error("unable to get basic cloner definition"))?;
        let mut recording = recording.clone();
        recording.should_loop = should_loop;

        let id = game.recordings.recordings.register_recording(recording);
        let new_cloner = Item::new_cloner(cloner_def, id);
        game.recordings.temp_item = Some(new_cloner);
        game.recordings.current_recording = None;
        Ok(())
    }

    // TODO Currently bugged; items will stack.
    pub fn take_item(game: &mut Game) -> Result<()> {
        let item = game
            .recordings
            .temp_item
            .ok_or(ActionFail("no cloner to take"))?;
        let location = game.get_player_coords()?;
        devtools::grant_item(item, *location, &game)?.apply(game)
    }
}
