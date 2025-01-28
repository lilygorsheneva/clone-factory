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

use crate::engine::update::UpdatableContainer;

use crate::{
    action::Action, error::{
        OkOrPopup, Result,
        Status::{ActionFail, Error},
    }, game_state::game::ApplyOrPopup, interface::{
        menu::UILayer,
        widgets::generate_popup_layout,
    }
};
use crate::{
    devtools,
    game_state::game::{Game, GameUpdate},
    interface::menu::MenuTrait,
    inventory::Item,
};

use RecordingMenuOptions::*;

// TODO: implement update struct so that functions operating on this don't need a refcell, maybe?
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
    pub fn init_record(game:&mut Game, idx: usize) -> Result<()> {
        if game.recordings.current_recording.is_some() {
            return Err(Error("Attempted to initialize recording twice"));
        }

        let mut player = game.get_player_actor().cloned()?;
        let coords = *game.get_player_coords()?;

        let item = player.inventory.remove_idx(idx).ok_or(ActionFail("No item in slot"))?;
        // TODO clean up this check.
        if item.definition.name != "recorder" {
            return Err(ActionFail("Item is not an empty recorder"));
        }
        

        game.world.actors.mut_set(&coords,&Some(player))?;
        game.recordings.current_recording = Some(Recording::from_creator(&player));
        Ok(())
    }

    // End recording.
    pub fn end_record(game:&mut Game,  should_loop: bool) -> Result<()> {

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
    pub fn take_item(game:&mut Game) -> Result<GameUpdate> {

        let item = game
            .recordings
            .temp_item
            .ok_or(ActionFail("no cloner to take"))?;
        let location = game.get_player_coords()?;
        devtools::grant_item(item, *location, &game)
    }
}

pub struct RecordingMenu<'a> {
    parent: &'a dyn UILayer,
    game: Rc<RefCell<Game>>,
}

impl<'a> RecordingMenu<'a> {
    pub fn new(parent: &'a dyn UILayer, game: Rc<RefCell<Game>>) -> RecordingMenu<'a> {
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

impl UILayer for RecordingMenu<'_> {
    fn draw(&self, frame: &mut ratatui::Frame) {
        self.parent.draw(frame);

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

        let area = generate_popup_layout(frame);
        let layout = Layout::default()
            .direction(layout::Direction::Vertical)
            .constraints(vec![Constraint::Min(1); entries.len()])
            .split(area);

        for i in 0..entries.len() {
            frame.render_widget(&entries[i], layout[i]);
        }
    }
}

impl MenuTrait for RecordingMenu<'_> {
    type MenuOptions = RecordingMenuOptions;

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

    fn enter_menu(&mut self, terminal: &mut ratatui::DefaultTerminal) {
        loop {
            terminal.draw(|frame| self.draw(frame)).unwrap();
            let current_rec;
            let temp_item;
            {
                let recording_module = &self.game.borrow().recordings;
                current_rec = recording_module.current_recording.is_some();
                temp_item = recording_module.temp_item.is_some();
            }

            match self.read() {
                Some(Exit) => break,
                Some(Use(i)) if !current_rec => {
                    let res = RecordingModule::init_record(&mut self.game.borrow_mut(), i);
                    res.ok_or_popup(self, terminal);
                    break;
                }
                Some(Loop) if current_rec => {
                    let res = RecordingModule::end_record(&mut self.game.borrow_mut(), true);
                       res.ok_or_popup(self, terminal);
                }
                Some(Die) if current_rec => {
                    let res =  RecordingModule::end_record(&mut self.game.borrow_mut(), false);
                    res.ok_or_popup(self, terminal);
                }
                Some(Take) if temp_item => {
                    let res =  RecordingModule::take_item(&mut self.game.borrow_mut());
                    res.apply_or_popup(&self.game, self, terminal);
                    break;
                }
                _ => {}
            }
        }
    }
}
