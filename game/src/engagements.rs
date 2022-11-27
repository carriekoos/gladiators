use bevy::prelude::*;

use crate::grid::*;

/// Plan for this design.
/// We are going to run movement systems first.
/// Then we will have an engagement_watchdog system that identifies engagments,
/// spawning Engagement entities that contain the gladiator Entitys involved
/// as well as the EngagmentState (Component) of that engagement (Starting | Fighting | Complete).
/// Separate system running at a fixed interval
///

pub struct EngagementManagerPlugin;

impl Plugin for EngagementManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(engagement_builder);
    }
}

// fn engagement_attacks(
//     engagement_query: Query<&Engagement>,
//     gladiator_query: Query<(&Attack), With<Gladiator>>,
//     mut ev_attack: EventWriter<AttackEvent>,
// ) {
//     for engagement in &engagement_query {

//         ev_attack.send(AttackEvent { target: (), attack: () });
//     }
// }

// TODO: Good docstrings on how/why this is used in this way.
fn engagement_builder(
    mut commands: Commands,
    arena_grid: Res<ArenaGrid>,
    gladiator_query: Query<&Engagement>,
) {
    for group in arena_grid.grid_map.values() {
        let mut colocated_and_unengaged = Vec::new();
        for entity in group {
            match gladiator_query.get(*entity) {
                Ok(_engagement) => {} // already engaged, not going to engage additionally
                Err(_) => colocated_and_unengaged.push(entity), // this gladiator is not engaged, so they are eligible
            }
        }

        let n_engagements = colocated_and_unengaged.len() / 2;
        for idx in 0..n_engagements {
            let &gladiator_a = colocated_and_unengaged
                .get(2 * idx)
                .expect("Already checked that this index should exist.");
            let &gladiator_b = colocated_and_unengaged
                .get(2 * idx + 1)
                .expect("Already checked that this index should exist.");
            println!(
                "New engagement between {:?} and {:?}!",
                gladiator_a, gladiator_b
            );
            // spawn the pair of Engagement(Targets) on the relevant entities
            // this time not with commands.spawn but instead entity.insert()
            commands.entity(*gladiator_a).insert(Engagement {
                target: *gladiator_b,
            });
            commands.entity(*gladiator_b).insert(Engagement {
                target: *gladiator_a,
            });
        }
    }
}

#[derive(Component)]
pub struct Engagement {
    pub target: Entity,
}
