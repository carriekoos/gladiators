use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    gladiator::{gladiator_combat::*, gladiator_events::DeathEvent},
    *, // game_lib
};

#[derive(Debug)]
pub struct GridChangeEvent {
    pub entity: Entity,
    pub prev_loc: GridLocation,
    pub curr_loc: GridLocation,
}

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GridChangeEvent>()
            .add_system(evaluate_grid)
            .add_system(
                prune_grid
                    .after(gladiator_receive_attack)
                    .before(gladiator_death_handler),
            )
            .init_resource::<ArenaGrid>();
    }
}

fn evaluate_grid(
    mut ev_grid_change: EventReader<GridChangeEvent>,
    mut arena_grid: ResMut<ArenaGrid>,
) {
    for event in ev_grid_change.iter() {
        arena_grid.update_entity_location(event.entity, &event.prev_loc, &event.curr_loc);
    }
}

/// Listens to DeathEvents and removes the gladiator that died from the arena grid.
fn prune_grid(
    mut arena_grid: ResMut<ArenaGrid>,
    mut ev_death: EventReader<DeathEvent>,
    query: Query<&Transform>,
) {
    for event in ev_death.iter() {
        let transform = query
            .get(event.slain)
            .expect("Slain gladiator should exist in ECS."); // this may be race condition with despawn? could handle by having grid go first or last always?
        let grid_location =
            ArenaGrid::get_grid_location(transform.translation[0], transform.translation[1]);
        let gladiators = arena_grid
            .grid_map
            .get_mut(&grid_location)
            .expect("Grid location of entity being removed should be present in grid_map.");
        gladiators.retain(|x| x != &event.slain);
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct GridLocation {
    pub x: i32,
    pub y: i32,
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
#[derive(Resource, Default, Debug)]
pub struct ArenaGrid {
    pub grid_map: HashMap<GridLocation, Vec<Entity>>,
    // https://docs.rs/bevy/latest/bevy/prelude/struct.Query.html#method.get
    // Store the Entity in this hashmap. Then any query that would contain
    // this Entity, can just use query.get(Entity) instead of looping through
    // the entire query.
}

impl ArenaGrid {
    /// Takes x,y coordinates and returns the GridLocation.
    /// Normalized for window size with given number of divisions.
    /// TODO This could probably take into account the gladiator size
    /// as well as the window size and not need so many constants.
    /// * `x`: pixel coordinate in x direction
    /// * `y`: pixel coordinate in y direction
    pub fn get_grid_location(x: f32, y: f32) -> GridLocation {
        // TODO - turn these into lazy statics so that we don't have to compute this so many times.
        let grid_cell_width = WINDOW_WIDTH / GRID_HORIZONTAL_DIVISIONS;
        let n_vertical_divisions = GRID_HORIZONTAL_DIVISIONS * (WINDOW_HEIGHT / WINDOW_WIDTH);
        let grid_cell_height = WINDOW_HEIGHT / n_vertical_divisions;

        let x_grid_num = ((x.abs() / grid_cell_width) + 0.5).floor();
        let horizontal_grid_location = (x.signum() * x_grid_num) as i32;

        let y_grid_num = ((y.abs() / grid_cell_height) + 0.5).floor();
        let vertical_grid_location = (y.signum() * y_grid_num) as i32;

        GridLocation {
            x: horizontal_grid_location,
            y: vertical_grid_location,
        }
    }

    /// Returns a Vec of Entitys that are currently located in the
    /// given grid location.
    /// * `loc`: grid location in question
    pub fn get_gladiators_in_grid_location(&self, loc: &GridLocation) -> Vec<Entity> {
        match self.grid_map.get(loc) {
            Some(vec) => vec.clone(),
            None => vec![],
        }
    }

    /// Returns a Vec off all adjacent grid locations to the given location
    /// * `loc`: grid location in question
    fn get_adjacent_grid_locations(loc: &GridLocation) -> Vec<GridLocation> {
        let mut grid_locations = Vec::new();
        // list the xs and the ys, do combinatorics, remove item == loc
        let possible_x = vec![loc.x - 1, loc.x, loc.x + 1];
        let possible_y = vec![loc.y - 1, loc.y, loc.y + 1];

        for x in possible_x {
            for y in &possible_y {
                // Don't add the starting grid location
                // if (x, y) == (loc.x, loc.y) { continue } // or add and remove later
                grid_locations.push(GridLocation { x, y: y.clone() }) // thought Copy would avoid the clone()
            }
        }

        // Removing the specific element from the Vec is probably slower, but
        // I'll have to do this pattern for removing an Entity from the map in
        // this struct, so I'm leaving this as an example for later. I will
        // uncomment the above if when I'm done having the example here.
        grid_locations.retain(|x| *x != *loc);

        grid_locations
    }

    /// Returns a HashMap indicating the Entitys in each of the adjacent grid locations
    /// * `loc`: grid location in question
    pub fn get_gladiators_in_adjacent_grid_locations(
        &self,
        loc: GridLocation,
    ) -> HashMap<GridLocation, Vec<Entity>> {
        let mut map = HashMap::new();

        // populate the map with the Vec of Entitys currently at each location
        let grid_locations = Self::get_adjacent_grid_locations(&loc);
        for location in grid_locations {
            let entities = match self.grid_map.get(&loc) {
                Some(vec) => vec.clone(),
                None => vec![],
            };

            map.insert(location, entities);
        }

        map
    }

    fn update_entity_location(
        &mut self,
        entity: Entity,
        prev_loc: &GridLocation,
        curr_loc: &GridLocation,
    ) {
        match self.grid_map.get_mut(prev_loc) {
            Some(entities_vec) => entities_vec.retain(|x| *x != entity),
            None => warn!("{:?} does not exist, only updating new location", prev_loc),
        }

        match self.grid_map.get_mut(curr_loc) {
            Some(entities_vec) => {
                entities_vec.push(entity);
            }
            None => {
                self.grid_map.insert(curr_loc.clone(), vec![entity]);
            }
        }
    }
}
