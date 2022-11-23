use bevy::prelude::*;

use crate::{grid::ArenaGrid, gladiator::*};

/// Plan for this design.
/// We are going to run movement systems first.
/// Then we will have an engagement_watchdog system that identifies engagments,
/// spawning Engagement entities that contain the gladiator Entitys involved
/// as well as the EngagmentState (Component) of that engagement (Starting | Fighting | Complete).
/// Separate system running at a fixed interval

pub struct EngagementManagerPlugin;

impl Plugin for EngagementManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(engagement_builder);
    }
}

// TODO: Yikes, this is a complicated one, but I think it may work? Should squint at it a lot.
fn engagement_builder(
    mut commands: Commands,
    griddy: Res<ArenaGrid>,
    // mut engagements_query: Query<&Engagement>, // why do I need this?
    mut gladiator_query: Query<&GladiatorEngagement, With<Gladiator>>,
) {
    let new_engagements = Vec::new();
    for group in griddy.grid_map.values() {
        let mut colocated_and_unengaged = Vec::new();
        for entity in group {
            match gladiator_query.get(*entity).expect("Gladiator in grid should exist.").status {
                Engaged => {},
                Unengaged => colocated_and_unengaged.push(entity),
            }
        }

        let n_engagements = colocated_and_unengaged.len() / 2;
        for idx in 0..n_engagements {
            let &gladiator_a = colocated_and_unengaged.get(2 * idx).expect("Already checked that this index should exist.");
            let &gladiator_b = colocated_and_unengaged.get(2 * idx + 1).expect("Already checked that this index should exist.");
            let new_engagement = Engagement {
                state: EngagementState::Start,
                gladiator_a: *gladiator_a,
                gladiator_b: *gladiator_b,
            };
            new_engagements.push(new_engagement);
            gladiator_query.get(*gladiator_a).expect("Gladiators should all have GladiatorEngagement").status = GladiatorEngagementStatus::Engaged;
        }
    }

    for engagement in new_engagements {
        println!("I built an engagement between {:?} and {:?}", engagement.gladiator_a, engagement.gladiator_b);
        commands.spawn(engagement);
    }
}
pub enum EngagementState {
    Start,
    Active,
    End,
}

#[derive(Component)]
pub struct Engagement {
    pub state: EngagementState,
    pub gladiator_a: Entity,
    pub gladiator_b: Entity,
}
