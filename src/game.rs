use crate::action;
use crate::action::{Action, SubAction};
use crate::actor::{Actor, ActorRef};
use crate::datatypes::Item;
use crate::datatypes::Recording;
use crate::db::{ActorDb, ActorId, RecordingDb};
use crate::direction::{
    AbsoluteDirection::{E, N, S, W},
    Direction::{Absolute, Relative},
    RelativeDirection::F,
};
use crate::error::{
    Result,
    Status::{ActionFail, Error},
};
use crate::{
    datatypes::Coordinate,
    world::{World, WorldCell},
};
use std::collections::VecDeque;

pub struct WorldActors {
    pub player: Option<PlayerRef>,
    turnqueue: VecDeque<ActorId>,
    nextturn: VecDeque<ActorId>,
    db: ActorDb,
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

    pub fn register_actor(&mut self, new_actor_ref: ActorRef) -> ActorId {
        let id = self.db.register_actor(new_actor_ref);
        if !new_actor_ref.isplayer {
            self.turnqueue.push_front(id);
        }
        id
    }

    pub fn get_next_actor(&mut self) -> Option<&mut ActorRef> {
        while let Some(id) = self.turnqueue.pop_front() {
            let actor = self.db.get_actor(id);
            if actor.live {
                self.nextturn.push_back(id);
                return Some(self.db.get_mut_actor(id));
            }
        }
        std::mem::swap(&mut self.turnqueue, &mut self.nextturn);
        None
    }
}

pub struct Game {
    pub world: World,
    pub actors: WorldActors,
    pub recordings: RecordingDb,
    pub current_recording: Option<Recording>,
}

impl Game {
    pub fn new(dimensions: Coordinate) -> Game {
        Game {
            world: World::new(dimensions),
            actors: WorldActors::new(),
            recordings: RecordingDb::new(),
            current_recording: None,
        }
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

    pub fn spawn(&mut self, location: &Coordinate) -> bool {
        if self.actors.player.is_some() {
            return false;
        }
        let target = self.world.get(&location);
        if target.is_none_or(|t| t.actor.is_some()) {
            return false;
        }
        let mut new_actor = Actor::new_player();
        let mut new_actor_ref = ActorRef::new(*location, crate::direction::AbsoluteDirection::N);
        new_actor_ref.isplayer = true;
        let player_id = self.actors.register_actor(new_actor_ref);
        new_actor.actor_id = player_id;

        let sample_recording_id = self
            .recordings
            .register_recording(&crate::devtools::make_sample_recording());
        let mut sample_recorder_item = Item::new(1, 1);
        sample_recorder_item.recording = Some(sample_recording_id);

        new_actor.inventory[1] = Some(sample_recorder_item);

        self.actors.player = Some(PlayerRef {
            actor_id: player_id,
        });
        let mut newcell = target.unwrap().clone();
        newcell.actor = Some(new_actor);
        self.world.set(location, Some(newcell));
        true
    }

    pub fn do_npc_turns(&mut self) -> Result<()> {
        while let Some(actor) = self.actors.get_next_actor() {
            let recording: &crate::datatypes::Recording = self.recordings.get(actor.recording);
            let action = recording.at(actor.command_idx);
            actor.command_idx += 1;
            let res = action::execute_action(*actor, action, self);
            match res {
                Ok(()) => (),
                Err(ActionFail) => (), // call fallback action
                res @ _ => return res,
            }
        }
        Ok(())
    }

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

    pub fn end_record(&mut self) -> Result<()> {
        match &self.current_recording {
            None => Err(Error("Attempted to initialize recording twice")),
            Some(rec) => {
                let id = self.recordings.register_recording(rec);
                let new_cloner = Item::new_cloner(id);
                self.current_recording = None;
                let actor_ref = self.actors.get_player()?;
                action::execute_action(
                    actor_ref,
                    Action {
                        direction: Relative(F),
                        action: SubAction::GrantItem(new_cloner),
                    },
                    self,
                )
            }
        }
    }

    pub fn player_action(&mut self, action: action::Action) -> Result<()> {
        let actor_ref = self.actors.get_player()?;

        if let Some(rec) = self.current_recording.as_mut() {
            rec.append(action);
        }

        match action::execute_action(actor_ref, action, self) {
            Err(ActionFail) => Ok(()), // Call fallback action.
            res @ _ => res,
        }
    }
}
