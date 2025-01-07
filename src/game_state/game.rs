//! Game state container, combining world state with other data containers.

use crate::action;
use crate::action::{Action, SubAction};
use crate::actor::{Actor, ActorRef};
use crate::static_data::Data;
use crate::inventory::Item;
use crate::datatypes::Recording;
use crate::game_state::db::{ActorDb, ActorDbUpdate, ActorId, RecordingDb};
use crate::direction::{Direction::Relative, RelativeDirection::F};
use crate::error::{
    Result,
    Status::{ActionFail, Error},
};
use crate::{
    datatypes::Coordinate,
    game_state::world::{World, WorldCell, WorldUpdate},
};
use std::collections::VecDeque;

// Move the WorldActors struct out to a dedicated module.
pub struct WorldActors {
    pub player: Option<PlayerRef>,
    turnqueue: VecDeque<ActorId>,
    nextturn: VecDeque<ActorId>,
    pub db: ActorDb,
}

pub struct PlayerRef {
    pub actor_id: ActorId,
}

impl WorldActors {
    pub fn new() -> WorldActors {
        WorldActors {
            player: None,
            turnqueue: VecDeque::new(),
            nextturn: VecDeque::new(),
            db: ActorDb::new(),
        }
    }

    pub fn get_player(&self) -> Result<ActorRef> {
        match &self.player {
            None => Err(Error("Player uninitialized")),
            Some(r) => Ok(self.get_actor(r.actor_id)),
        }
    }

    pub fn get_mut_player(&mut self) -> Result<&mut ActorRef> {
        match &self.player {
            None => Err(Error("Player uninitialized")),
            Some(r) => Ok(self.get_mut_actor(r.actor_id)),
        }
    }

    pub fn get_actor(&self, id: ActorId) -> ActorRef {
        self.db.get_actor(id)
    }
    pub fn get_mut_actor(&mut self, id: ActorId) -> &mut ActorRef {
        self.db.get_mut_actor(id)
    }

    pub fn mut_register_actor(&mut self, new_actor_ref: ActorRef) -> ActorId {
        let id = self.db.mut_register_actor(new_actor_ref);
        if !new_actor_ref.isplayer {
            self.turnqueue.push_front(id);
        }
        id
    }

    pub fn queue_new_actors(&mut self, update: &ActorDbUpdate) {
        for id in update.peek_new_actors() {
            self.turnqueue.push_front(*id);
        }
    }

    pub fn get_next_actor(&mut self) -> Option<&mut ActorRef> {
        while let Some(id) = self.turnqueue.pop_front() {
            let actor = self.db.get_actor(id);
            if actor.live & !actor.isplayer {
                self.nextturn.push_back(id);
                return Some(self.db.get_mut_actor(id));
            }
        }
        std::mem::swap(&mut self.turnqueue, &mut self.nextturn);
        None
    }
}

// Game state container.
pub struct Game {
    pub world: World,
    pub actors: WorldActors,
    pub recordings: RecordingDb,
    pub current_recording: Option<Recording>,

    // TODO: don't store data by value within a game.
    pub data: Data,
}

// A container that stores game updates.
// Most operations on the game can be performed with an immutable game and a mutable update.
// Muate game state with game.apply_update(update).
#[derive(Debug)]
pub struct GameUpdate {
    pub world: WorldUpdate,
    pub actors: ActorDbUpdate,
}

impl Game {
    pub fn new(dimensions: Coordinate) -> Game {
        Game {
            world: World::new(dimensions),
            actors: WorldActors::new(),
            recordings: RecordingDb::new(),
            current_recording: None,
            data: Data::default(),
        }
    }

    pub fn load_gamedata(&mut self) {
        self.data = Data::get_config();
    }

    #[cfg(test)]
    pub fn load_testdata(&mut self) {
        self.data = Data::get_test_config();
    }

    pub fn get_player_actor(&self) -> Result<&Actor> {
        let location = self.get_player_coords()?;
        match self.world.get(&location) {
            None => Err(Error("Player coordinates out of bounds")),
            Some(WorldCell { actor: None, .. }) => Err(Error("No actor at player coordinates")),
            Some(WorldCell {
                actor: Some(actor), ..
            }) => Ok(actor),
        }
    }

    pub fn get_player_coords(&self) -> Result<Coordinate> {
        let actor = self.actors.get_player()?;
        Ok(actor.location)
    }

    pub fn spawn(&mut self, location: &Coordinate) -> Result<()> {
        if self.actors.player.is_some() {
            return Err(Error("Player exists"));
        }

        match self.world.get(&location) {
            Some(target @ WorldCell { actor: None, .. }) => {
                let mut new_actor = Actor::new_player();
                let mut new_actor_ref =
                    ActorRef::new(*location, crate::direction::AbsoluteDirection::N);
                new_actor_ref.isplayer = true;
                let player_id = self.actors.mut_register_actor(new_actor_ref);
                new_actor.actor_id = player_id;

                self.actors.player = Some(PlayerRef {
                    actor_id: player_id,
                });
                let mut newcell = target.clone();
                newcell.actor = Some(new_actor);
                self.world.mut_set(location, Some(newcell))
            }
            _ => Err(Error("Invalid player spawn")),
        }
    }

    pub fn do_npc_turns(&mut self) -> Result<()> {
        while let Some(actor) = self.actors.get_next_actor() {
            let recording: &crate::datatypes::Recording = self.recordings.get(actor.recording);
            let action = recording.at(actor.command_idx);
            actor.command_idx += 1;
            let res = action::execute_action(*actor, action, self);
            match res {
                Ok(update) => self.apply_update(update)?,
                Err(ActionFail(_)) => (), // call fallback action
                Err(res) => return Err(res),
            }
        }
        Ok(())
    }

    // Start recording. 
    pub fn init_record(&mut self) -> Result<()> {
        match self.current_recording {
            Some(_) => Err(Error("Attempted to initialize recording twice")),
            None => {
                let actor = self.get_player_actor()?;
                self.current_recording = Some(Recording::from_creator(actor));
                Ok(())
            }
        }
    }

    // End recording and spawn a recording item.
    // TODO Currently bugged; items will stack.
    pub fn end_record(&mut self) -> Result<()> {
        match &self.current_recording {
            None => Err(Error("Attempted to initialize recording twice")),
            Some(rec) => {
                let id = self.recordings.register_recording(rec);
                let new_cloner = Item::new_cloner(4, id);
                self.current_recording = None;
                let actor_ref = self.actors.get_player()?;
                let update = action::execute_action(
                    actor_ref,
                    Action {
                        direction: Relative(F),
                        action: SubAction::GrantItem(new_cloner),
                    },
                    self,
                )?;
                self.apply_update(update)
            }
        }
    }

    // Process a player's actions.
    pub fn player_action(&mut self, action: action::Action) -> Result<()> {
        let actor_ref = self.actors.get_player()?;

        if let Some(rec) = self.current_recording.as_mut() {
            rec.append(action);
        }

        match action::execute_action(actor_ref, action, self) {
            Ok(update) => self.apply_update(update),
            Err(ActionFail(_)) => Ok(()), // Call fallback action.
            Err(res) => Err(res),
        }
    }

    pub fn new_update(&self) -> GameUpdate {
        GameUpdate {
            world: self.world.new_update(),
            actors: self.actors.db.new_update(),
        }
    }

    pub fn apply_update(&mut self, update: GameUpdate) -> Result<()> {
        self.world.apply_update(&update.world)?;
        self.actors.db.apply_update(&update.actors)?;
        self.actors.queue_new_actors(&update.actors);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::direction::{AbsoluteDirection, Direction::Absolute};

    use super::*;

    #[test]
    fn record() {
        let mut game = Game::new(Coordinate { x: 1, y: 2 });

        assert!(game.spawn(&Coordinate { x: 0, y: 0 }).is_ok());

        game.init_record().unwrap();

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

        game.end_record().unwrap();

        // This is really ugly. Perhaps recording needs a nicer API.
        let actor = game
            .world
            .get(&game.get_player_coords().unwrap())
            .unwrap()
            .actor
            .as_ref();
        let recorder = actor.unwrap().inventory.get_items()[0].unwrap();
        let recoding = game.recordings.get(recorder.recording.unwrap());
        assert_eq!(recoding.command_list, actions);
    }

    #[test]
    fn clone() {
        let mut game = Game::new(Coordinate { x: 1, y: 3 });
        game.load_testdata();

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
        .recordings
        .register_recording(&Recording{command_list: actions, inventory: Default::default()});
        let new_cloner = Item::new_cloner(4, sample_recording_id);
        let update = action::execute_action(
            game.actors.get_player().unwrap(),
            Action {
                direction: Relative(F),
                action: SubAction::GrantItem(new_cloner),
            },
            &mut game,
        ).unwrap();
        game.apply_update(update).unwrap();

        let update = action::execute_action(
            game.actors.get_player().unwrap(),
            Action {
                direction: Absolute(AbsoluteDirection::N),
                action: SubAction::Use(0),
            },
            &mut game,
        )
        .unwrap();
        game.apply_update(update).unwrap();

        let dest = game.world.get(&Coordinate{x:0, y:2});
        assert!(dest.is_some());
        assert!(dest.unwrap().actor.is_none());

        game.do_npc_turns().unwrap();

        let dest = game.world.get(&Coordinate{x:0, y:2});
        assert!(dest.is_some());
        assert!(dest.unwrap().actor.is_some());
    }
}
