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
use ratatui::widgets::Paragraph;

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
    pub fn init_record(game: &mut Game) -> Result<()> {
        match game.recordings.current_recording {
            Some(_) => Err(Error("Attempted to initialize recording twice")),
            None => {
                let actor = game.get_player_actor()?;
                game.recordings.current_recording = Some(Recording::from_creator(actor));
                Ok(())
            }
        }
    }

    // End recording.
    pub fn end_record(game: &mut Game) -> Result<()> {
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
}

impl MenuTrait for RecordingMenu<'_> {
    type MenuOptions = RecordingMenuOptions;

    fn draw(&self, frame: &mut ratatui::Frame) {
        let widget = Paragraph::new("L: loop recording.\n D: do not loop recording.");
        frame.render_widget(widget, frame.area());
    }

    fn parsekey(&self, key: crossterm::event::KeyEvent) -> Option<Self::MenuOptions> {
        match key.code {
            KeyCode::Esc => Some(Exit),
            KeyCode::Char('l') => Some(Loop),
            KeyCode::Char('d') => Some(Die),
            _ => None,
        }
    }

    fn call(&mut self, terminal: &mut ratatui::DefaultTerminal) {
        let mut game = self.game.borrow_mut();
        let recorder = &game.recordings;
        match recorder.current_recording {
            None => RecordingModule::init_record(&mut game),
            Some(_) => {
                RecordingModule::end_record(&mut game).unwrap();
                if let Ok(update) = RecordingModule::take_item(&mut game) {
                    game.apply_update(update)
                } else {
                    Ok(())
                }
            }
        }.unwrap();
    }
}

