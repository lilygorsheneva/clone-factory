use crate::datatypes::Coordinate;
use crate::engine::update::{Delta, UpdatableContainer, UpdatableContainerDelta};
use crate::engine::worldlayer::WorldLayer;
use crate::error::{Result, Status::Error};
use crate::game_state::game::Game;
use crate::{engine::tracking_worldlayer::TrackableId, game_state::game::GameUpdate};

#[derive(PartialEq, Debug, Clone)]
pub struct Paradox(pub f64);

pub fn update_actor_paradox(
    actor: TrackableId,
    increment: f64,
    game: &Game,
) -> Result<(GameUpdate, bool)> {
    let mut update = GameUpdate::new();
    let location = *update
        .world
        .actor_updates
        .get_location(&game.world.actors, &actor)?;
    let actor = update
        .world
        .actor_updates
        .get(&game.world.actors, &location)?
        .as_ref()
        .ok_or(Error("No actor at expected coordinates"))
        .cloned()?;

    let background = update
        .world
        .paradox_updates
        .get(&game.world.paradox, &location)?;

    let mut new_background = increment + background.0;
    if new_background < 0.0 {
        new_background = 0.0;
    }

    let survive;
    if let Some(threshold) = actor.descriptor.hp {
        survive = threshold > new_background as i64;
    } else {
        survive = true;
    }

    update
        .world
        .paradox_updates
        .set(&location, &Paradox(new_background))?;

    update.world.actor_updates.set(&location, &Some(actor))?;
    Ok((update, survive))
}

pub fn diffuse_paradox(layer: &mut WorldLayer<Paradox>) {
    let dimensions = layer.get_dimensions();
    let old_layer = layer.clone();

    for x in 0..dimensions.x {
        for y in 0..dimensions.y {
            let mut tmp = 0.0;
            let mut area = 0.0;
            for i in -1..2 {
                for j in -1..2 {
                    let coord = Coordinate { x: x + i, y: y + j };
                    if layer.in_bounds(&coord) {
                        tmp += old_layer.get(&coord).unwrap().0;
                        area += 1.0;
                    }
                }
            } // end inner loop pair

            tmp = tmp / area;
            if tmp < 0.0 {
                tmp = 0.0
            }
            layer.mut_set(&Coordinate { x, y }, &Paradox(tmp)).unwrap();
        }
    }
}
