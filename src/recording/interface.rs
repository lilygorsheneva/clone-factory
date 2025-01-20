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
    action::{self, Action},
    engine::update,
    error::{
        Result,
        Status::{ActionFail, Error},
    },
    inventory,
    static_data::StaticData,
};
use crate::{
    devtools,
    game_state::game::{Game, GameUpdate},
    interface::menu::{gamemenu::GameMenu, MenuTrait},
    inventory::Item,
};

use RecordingMenuOptions::*;

pub struct RecordingModule {
    pub recordings: RecordingDb,
    pub current_recording: Option<Recording>,
    pub temp_item: Option<Item>,
}

impl RecordingModule {
    pub fn new() -> RecordingModule {
        RecordingModule {
            recordings: RecordingDb::new(),
            current_recording: None,
            temp_item: None,
        }
    }

    pub fn get(&self, id: RecordingId) -> &Recording {
        &self.recordings.get(id)
    }

    pub fn append(&mut self, action: Action) {
        if let Some(rec) = self.current_recording.as_mut() {
            rec.append(action);
        }
    }

    // Start recording.
    pub fn init_record(game: &mut Game, idx: usize) -> Result<GameUpdate> {
        if game.recordings.current_recording.is_some() {
            return Err(Error("Attempted to initialize recording twice"));
        }

        let player = game.get_player_actor()?;
        let coords = game.get_player_coords()?;

        let mut item = player.inventory.get_items()[idx].ok_or(ActionFail("No item"))?;
        // TODO clean up this check.
        if item.definition.name != "Empty Recorder" {
            return Err(ActionFail("Item is not an emtpy recorder"));
        }
        item.quantity = 1;

        let ret = devtools::remove_item(item, coords, game);
        if ret.is_ok() {
            game.recordings.current_recording = Some(Recording::from_creator(player));
        }
        ret
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
    pub fn take_item(game: &mut Game) -> Result<GameUpdate> {
        let item = game
            .recordings
            .temp_item
            .ok_or(ActionFail("no cloner to take"))?;
        let location = game.get_player_coords()?;
        devtools::grant_item(item, location, game)
    }
}

pub struct RecordingMenu<'a> {
    parent: &'a GameMenu,
    game: Rc<RefCell<Game>>,
}

impl<'a> RecordingMenu<'a> {
    pub fn new(parent: &'a GameMenu, game: Rc<RefCell<Game>>) -> RecordingMenu<'a> {
        RecordingMenu {
            parent: parent,
            game: game.clone(),
        }
    }
}

pub enum RecordingMenuOptions {
    Exit,
    Loop,
    Die,
    Take,
    Use(usize),
}

impl MenuTrait for RecordingMenu<'_> {
    type MenuOptions = RecordingMenuOptions;

    fn draw(&self, frame: &mut ratatui::Frame) {
        let mut entries = Vec::new();
        let recording_module = &self.game.borrow().recordings;

        entries.push(
            Paragraph::new("Welcome to the record-o-matic recording system.")
                .block(Block::bordered()),
        );
        if recording_module.current_recording.is_some() {
            entries.push(
                Paragraph::new("L: loop recording.\n D: do not loop recording.")
                    .block(Block::bordered()),
            );
        } else {
            entries.push(
                Paragraph::new("1-5: start a recording using this empty recorder.")
                    .block(Block::bordered()),
            );
        }

        if recording_module.temp_item.is_some() {
            entries.push(
                Paragraph::new("T: add latest recording to inventory.").block(Block::bordered()),
            );
        }

        let layout = Layout::default()
            .direction(layout::Direction::Vertical)
            .constraints(vec![Constraint::Min(1); entries.len()])
            .split(frame.area());

        for i in 0..entries.len() {
            frame.render_widget(&entries[i], layout[i]);
        }
    }

    fn parsekey(&self, key: crossterm::event::KeyEvent) -> Option<Self::MenuOptions> {
        match key.code {
            KeyCode::Esc => Some(Exit),
            KeyCode::Char('l') => Some(Loop),
            KeyCode::Char('d') => Some(Die),
            KeyCode::Char('t') => Some(Take),
            KeyCode::Char('1') => Some(Use(0)),
            KeyCode::Char('2') => Some(Use(1)),
            KeyCode::Char('3') => Some(Use(2)),
            KeyCode::Char('4') => Some(Use(3)),
            KeyCode::Char('5') => Some(Use(4)),
            _ => None,
        }
    }

    fn call(&mut self, terminal: &mut ratatui::DefaultTerminal) {
        loop {
            terminal.draw(|frame| self.draw(frame)).unwrap();
            let mut game = self.game.borrow_mut();
            let recording_module = &game.recordings;

            match self.read() {
                Some(Exit) => break,
                Some(Use(i)) if recording_module.current_recording.is_none() => {
                    let update = RecordingModule::init_record(&mut game, i).unwrap();
                    game.apply_update(update);
                    break;
                }
                Some(Loop) if recording_module.current_recording.is_some() => {
                    RecordingModule::end_record(&mut game, true).unwrap();
                }
                Some(Die) if recording_module.current_recording.is_some() => {
                    RecordingModule::end_record(&mut game, false).unwrap();
                }
                Some(Take) if recording_module.temp_item.is_some() => {
                    let update = RecordingModule::take_item(&mut game).unwrap();
                    game.apply_update(update);
                    break;
                }
                _ => {}
            }
        }
        // match recorder.current_recording {
        //     None => game.apply_update(RecordingModule::init_record(&mut game)),
        //     Some(_) => {
        //         RecordingModule::end_record(&mut game, true).unwrap();
        //         if let Ok(update) = RecordingModule::take_item(&mut game) {
        //             game.apply_update(update)
        //         } else {
        //             Ok(())
        //         }
        //     }
        // }.unwrap();
    }
}
