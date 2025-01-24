//! Game state container, combining world state with other data containers.

use ratatui::DefaultTerminal;

use crate::engine::tracking_worldlayer::TrackableId;
use crate::engine::update::{Delta, Updatable, UpdatableContainer};
use crate::error::StatusMenu;
use crate::interface::menu::{MenuTrait, UILayer};
use crate::recording::interface::RecordingModule;
use crate::score::{Score, ScoreDelta};
use crate::{action, devtools};
use crate::actor::{Actor};
use crate::static_data::StaticData;
use crate::recording::Recording;

use crate::game_state::db::{ ActorId};
use crate::error::{
    Result,
    Status::{ActionFail, Error},
};
use crate::{
    datatypes::Coordinate,
    game_state::world::{World, WorldUpdate},
};
use std::cell::RefCell;
use crate::eventqueue::{EventQueue, EventQueueUpdate};

// Move the WorldActors struct out to a dedicated module.
pub struct WorldActors {
    pub player: Option<PlayerRef>,
}

pub struct PlayerRef {
    pub actor_id: TrackableId,
}

impl WorldActors {
    pub fn new() -> WorldActors {
        WorldActors {
            player: None,
        }
    }

    pub fn get_player(&self) -> Result<TrackableId> {
        match &self.player {
            None => Err(Error("Player uninitialized")),
            Some(r) => Ok(r.actor_id),
        }
    }
}

// Game state container.
pub struct Game {
    pub world: World,
    pub actors: WorldActors,
    pub recordings: RecordingModule,
    pub event_queue: EventQueue, 
    pub data: &'static  StaticData,
    pub score: Score
}

impl Updatable for Game{}

// A container that stores game updates.
// Most operations on the game can be performed with an immutable game and a mutable update.
// Muate game state with game.apply_update(update).
#[derive(Debug)]
#[must_use = "Valid game update objects must be used."]
pub struct GameUpdate {
    pub world: WorldUpdate,
    pub eventqueue: EventQueueUpdate,
    pub score: ScoreDelta,
}

impl Delta for GameUpdate {
    type Target = Game;
    fn new() -> GameUpdate {
        GameUpdate {
            world: WorldUpdate::new(),
            eventqueue: EventQueueUpdate::new(),
            score: ScoreDelta(0)
        }
    }

    fn apply(&self, target: &mut Self::Target) -> Result<()> {
        self.world.apply(&mut target.world)?;
        self.eventqueue.apply(&mut target.event_queue)?;
        self.score.apply(&mut target.score)?;
        Ok(())
    }
}

pub trait ApplyOrPopup {
    fn apply_or_popup(self, game: &RefCell<Game>, parent: &dyn UILayer,  terminal: &mut DefaultTerminal);    }

impl ApplyOrPopup for Result<GameUpdate> {
    fn apply_or_popup(self, game: &RefCell<Game>, parent: &dyn UILayer,  terminal: &mut DefaultTerminal) {
        match self {
            Ok(update) => {
                let new_result = update.apply(&mut game.borrow_mut());
                if let Err(err) = new_result {
                    StatusMenu::new(err, parent).enter_menu(terminal);
                    panic!("Error applying game update.")
                }
            },
            Err(ActionFail(msg)) => StatusMenu::new(ActionFail(msg), parent).enter_menu(terminal),
            Err(status) => {StatusMenu::new(status, parent).enter_menu(terminal); panic!("Uncaught error when generating game update.")}
        };
    }
}

impl Game {
    pub fn new(dimensions: Coordinate, data: &'static StaticData) -> Game {
        Game {
            world: World::new(dimensions),
            actors: WorldActors::new(),
            recordings: RecordingModule::new(),
            event_queue: EventQueue::new(),
            data: data,
            score: Score(0)
        }
    }

    pub fn get_player_actor(&self) -> Result<&Actor> {
        let location = self.get_player_coords()?;
        match self.world.actors.get(&location) {
            Err(crate::error::Status::OutOfBounds) => Err(Error("Player coordinates out of bounds")),
            Err(foo) => Err(foo),
            Ok(None) => Err(Error("No actor at player coordinates")),
            Ok(Some(actor)) => Ok(actor),
         }
    }

    pub fn get_player_coords(&self) -> Result<&Coordinate> {
        let id = self.actors.get_player()?;
        self.world.actors.get_location(&id)
    }

    pub fn spawn(&mut self, location: &Coordinate) -> Result<()> {
        if self.actors.player.is_some() {
            return Err(Error("Player exists"));
        }

        let dest = self.world.actors.get(location)?;
        if dest.is_some() {
            return Err(Error("Destination Occupied"));
        }
        let mut new_actor = Actor::new_player();
    
        let player_id = self.world.actors.mut_get_next_id();
        new_actor.actor_id = ActorId{idx: player_id.0};

        self.actors.player = Some(PlayerRef {
            actor_id: player_id,
        });
        self.world.actors.mut_set(location,&Some(new_actor))
    }

    pub fn do_npc_turns(&mut self) -> Result<()> {
        while let Some(mut evt) = self.event_queue.get_next_event() {
            let recording: &Recording = self.recordings.get(evt.recording);
            // handle looping here.
            let action = recording.at(evt.recording_idx);

            let action_result = action::execute_action(evt.actor, action, self);


            let ret = match action_result {
                Ok(update) => update.apply(self),
                Err(ActionFail(_)) => Ok(()), // call fallback action
                Err(res) => Err(res),
            }?;

            let recording: &Recording = self.recordings.get(evt.recording);
            evt.recording_idx += 1;
            if evt.recording_idx >= recording.len() {
                if recording.should_loop {
                    evt.recording_idx = evt.recording_idx % recording.len();
                    self.event_queue.next_turn.push_back(evt);
                } else {
                    let update = devtools::despawn_actor(evt.actor, &self)?;
                    update.apply(self).unwrap();
                }
            } else {
                self.event_queue.next_turn.push_back(evt);
            }
        }
        Ok(())
    }

    // Process a player's actions.
    pub fn player_action(&mut self, action: action::Action) -> Result<()> {
        let actor_ref = self.actors.get_player()?;

        match action::execute_action(actor_ref, action, self) {
            // TODO: shove in an apply or popup here.
            Ok(update) => {self.recordings.append(action);  update.apply(self)},
            //Err(ActionFail(_)) => Ok(()), // Call fallback action.
            Err(res) => Err(res),
        }
    }

    pub fn player_action_and_turn(&mut self, action: action::Action) -> Result<()> {
        self.player_action(action)?;
        self.do_npc_turns()?;
        self.event_queue.advance_turn()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use action::{Action, SubAction};

    use crate::{devtools, direction::{AbsoluteDirection, Direction::Absolute}, inventory::Item};

    use super::*;

    #[test]
    fn record() {
        let data = StaticData::get_test_config();
        let mut game = Game::new(Coordinate { x: 1, y: 2 }, &data);

        assert!(game.spawn(&Coordinate { x: 0, y: 0 }).is_ok());

        let recorder_def = data.items.get(&"recorder".to_string()).unwrap();
        let item = Item::new(recorder_def, 1);
        let update = devtools::grant_item(item, *game.get_player_coords().unwrap(), &game).unwrap();
        
        update.apply(&mut game).unwrap();

        RecordingModule::init_record(&mut game, 0).unwrap();


        let actions = [
            Action {
                direction: Absolute(AbsoluteDirection::N),
                action: SubAction::Move,
            },
            Action {
                direction: Absolute(AbsoluteDirection::S),
                action: SubAction::Move,
            },
        ];

        game.player_action(actions[0]).unwrap();
        game.player_action(actions[1]).unwrap();

        RecordingModule::end_record(&mut game, false).unwrap();
        let update = RecordingModule::take_item(&mut game).unwrap();
        update.apply(&mut game).unwrap();


        // This is really ugly. Perhaps recording needs a nicer API.
        let actor = game
            .world
            .actors.get(&game.get_player_coords().unwrap()).unwrap();
        let recorder = actor.unwrap().inventory.get_items()[0].unwrap();
        let recoding = game.recordings.get(recorder.recording.unwrap());
        assert_eq!(recoding.command_list, actions);
    }

    #[test]
    fn clone() {
        let data = StaticData::get_test_config();
        let mut game = Game::new(Coordinate { x: 1, y: 3 }, &data);

        assert!(game.spawn(&Coordinate { x: 0, y: 0 }).is_ok());

        let actions = vec![
            Action {
                direction: Absolute(AbsoluteDirection::N),
                action: SubAction::Move,
            },
            Action {
                direction: Absolute(AbsoluteDirection::N),
                action: SubAction::Move,
            },
        ];

        let sample_recording_id = game
        .recordings.recordings
        .register_recording(Recording{command_list: actions, inventory: Default::default(), should_loop:true});
        let cloner_def = data.items.get(&"basic_cloner".to_string()).unwrap();
        let new_cloner = Item::new_cloner(cloner_def, sample_recording_id);
        let update = devtools::grant_item(new_cloner, *game.get_player_coords().unwrap(), &game).unwrap();
        update.apply(&mut game).unwrap();

        let update = action::execute_action(
            game.actors.get_player().unwrap(),
            Action {
                direction: Absolute(AbsoluteDirection::N),
                action: SubAction::Use(0),
            },
            &mut game,
        )
        .unwrap();
        update.apply(&mut game).unwrap();

        let dest = game.world.actors.get(&Coordinate{x:0, y:2}).unwrap();
        assert!(dest.is_none());

        game.do_npc_turns().unwrap();

        let dest = game.world.actors.get(&Coordinate{x:0, y:2}).unwrap();
        assert!(dest.is_some());
    }
}
