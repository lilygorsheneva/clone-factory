use crate::actor::{Actor,ActorRef};
use crate::{world::{World, WorldCell}, datatypes::Coordinate};
use std::collections::VecDeque;
use crate::db::{ActorDb, ActorId, RecordingDb};

pub struct WorldActors {
    pub player: Option<PlayerRef>,
    turnqueue: VecDeque<ActorRef>,
    nextturn: VecDeque<ActorRef>,
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
    
    pub fn get_player(&self) -> &ActorRef{
        let id = self.player.as_ref().unwrap().actor_id;
        self.db.get_actor(id)
    }

    pub fn get_mut_player(&mut self) -> &mut ActorRef{
        let id = self.player.as_ref().unwrap().actor_id;
        self.db.get_mut_actor(id)
    }
}

pub struct Game {
    pub world: World,
    pub actors: WorldActors,
    pub recordings: RecordingDb
}

impl Game {
    pub fn new(dimensions: Coordinate) -> Game{
        Game {
            world: World::new(dimensions),
            actors: WorldActors::new(),
            recordings: RecordingDb::new()
        }
    }

    pub fn get_player_coords(&self) -> Coordinate {
        let actor = self.actors.get_player();
        return actor.location
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
        let new_actor_ref = ActorRef::new(*location);
        let player_id = self.actors.db.register_actor(new_actor_ref);
        new_actor.actor_id = player_id;

        self.actors.player = Some(PlayerRef
            {
                actor_id: player_id,
            });
        self.world.set(
            location,
            Some(WorldCell {
                actor: Some(new_actor),
                building: target.unwrap().building.clone(),
                items: target.unwrap().items.clone(),
            }),
        );
        true
    }


}
