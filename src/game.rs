use crate::actor::{Actor,ActorRef};
use crate::{world::{World, WorldCell}, datatypes::Coordinate};
use crate::actor::PlayerRef;
use std::collections::VecDeque;

pub struct WorldActors {
    pub player: Option<PlayerRef>,
    turnqueue: VecDeque<ActorRef>,
    nextturn: VecDeque<ActorRef>,
}

impl WorldActors {
    pub fn new() -> WorldActors {
        WorldActors {
            player: None,
            turnqueue: VecDeque::new(),
            nextturn: VecDeque::new(),
        }
    }
}

pub struct Game {
    pub world: World,
    pub actors: WorldActors,
}

impl Game {
    pub fn new(dimensions: Coordinate) -> Game{
        Game {
            world: World::new(dimensions),
            actors: WorldActors::new()
        }
    }

    pub fn get_player_coords(&self) -> Coordinate {
        self.actors.player.as_ref().unwrap().actor_ref.location
    }

    pub fn spawn(&mut self, location: &Coordinate) -> bool {
        if self.actors.player.is_some() {
            return false;
        }
        let target = self.world.get(&location);
        if target.is_none_or(|t| t.actor.is_some()) {
            return false;
        }
        self.actors.player = Some(PlayerRef
            {
                actor_ref: ActorRef{location:*location},
                current_recording: Vec::new()
            });
        self.world.set(
            location,
            Some(WorldCell {
                actor: Some(Actor::new_player()),
                building: target.unwrap().building.clone(),
                items: target.unwrap().items.clone(),
            }),
        );
        true
    }
}
