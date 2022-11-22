use bevy::prelude::*;

use crate::grid::ArenaGrid;

/// Plan for this design.
/// We are going to run movement systems first.
/// Then we will have an engagement_watchdog system that identifies engagments,
/// spawning Engagement entities that contain the gladiator Entitys involved
/// as well as the EngagmentState (Component) of that engagement (Starting | Fighting | Complete).
/// Separate system running at a fixed interval

pub struct EngagementManagerPlugin;

impl Plugin for EngagementManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(engagement_lifecycle);
    }
}

// TODO: this is fucked, will generate an infinite amount of engagements
fn engagement_lifecycle(
    mut commands: Commands,
    mut griddy: ResMut<ArenaGrid>,
    mut engagements_query: Query<&Engagement>,
) {
    for (gladiator_a, gladiator_b) in griddy.get_colocated_gladiators() {
        commands.spawn(Engagement {
            state: EngagementState::Start,
            gladiator_a,
            gladiator_b,
        });
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