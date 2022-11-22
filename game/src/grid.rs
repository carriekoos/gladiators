use std::collections::HashMap;

use bevy::{prelude::*, time::FixedTimestep};

use crate::{animation::*, gladiator::*, *};

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_grid).add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(GRID_EVALUATION_STEP as f64))
                .with_system(evaluate_grid),
        );
    }
}

/// Spawns the grid
fn spawn_grid() {}

fn evaluate_grid() {}

// TODO I think the grid is a resource, not a component

pub struct GridLocation {
    x: i32,
    y: i32,
}

/// ArenaGrid stores a Vec of Entitys that are located at each
/// GridLocation, which is a section of the whole arena.
/// The HashMap could either be reconstructed at a given rate or
/// it could be optimized to only be updated by moving Gladiators,
/// but that seems like a really complicated option. Initial impl
/// will be to reconstruct by looping over all Gladiators at about
/// half the interval as the MOVEMENT_STEPs are.
/// This is likely a performance bottleneck, but can be optimized
/// at a later time.
///
/// Possible option: Gladiators can ask for their current grid
/// location and store their previous grid location. On change,
/// an Event can be emitted that the ArenaGrid is listening for,
/// which hopefully can have the Entity and location data in it.
/// It would need previous coordinates as well as new, so that
/// it could be removed from the previous GridLocation key.
/// Gladiators may getting their current GridLocation anyways
/// for the purposes of figuring out what other Gladiators
/// are nearby so they can know where to move. Bit of a twofer
/// so this may be a pretty sensible option and allows for
/// carving out a chunkier grid as a performance lever.
///
/// I think minimizing the mutable reference to this guy
/// will be a performance win. I think that can be accomplished
/// by the events so that the Grid can mutate itself.
///
/// Gladiators will need to know their GridLocation and two
/// other things:
/// 1. Entity of Gladiator(unengaged) in the same GridLocation. (can engage)
/// 2. Entities of Gladiators(unengaged) in any of the adjacent
///  GridLocations. (will move towards)
pub struct ArenaGrid {
    grid_map: HashMap<GridLocation, Vec<Entity>>,
    // https://docs.rs/bevy/latest/bevy/prelude/struct.Query.html#method.get
    // Store the Entity in this hashmap. Then any query that would contain
    // this Entity, can just use query.get(Entity) instead of looping through
    // the entire query.
}
