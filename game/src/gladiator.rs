use bevy::{prelude::*, time::FixedTimestep};
use rand::{self, distributions::WeightedIndex, prelude::*, rngs::ThreadRng, Rng};

use crate::{
    animation::*,
    engagements::*,
    grid::{ArenaGrid, GridChangeEvent},
    helper_functions::*,
    player::*,
    *,
};

pub struct GladiatorPlugin;

impl Plugin for GladiatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_gladiators)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(MOVEMENT_STEP as f64))
                    .with_system(gladiator_movement),
            )
            .add_system(gladiator_attacks)
            .add_system(gladiator_receive_attack)
            .add_system(gladiator_death_handler)
            .add_event::<AttackEvent>()
            .add_event::<DeathEvent>();
    }
}

#[derive(Debug)]
pub struct AttackEvent {
    pub target: Entity,
    pub attacker: Entity,
    pub attack: Attack,
}

#[derive(Debug)]
pub struct DeathEvent {
    pub victor: Entity,
    pub xp_earned: f32,
    pub slain: Entity,
}

fn gladiator_attacks(
    time: Res<Time>,
    mut ev_attack: EventWriter<AttackEvent>,
    mut query: Query<(&Engagement, &Attack, &mut AttackTimer, Entity)>,
) {
    for (engagement, attack, mut timer, entity) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            ev_attack.send(AttackEvent {
                target: engagement.target,
                attacker: entity,
                attack: *attack,
            });
        }
    }
}

fn gladiator_receive_attack(
    mut ev_attack: EventReader<AttackEvent>,
    mut ev_death: EventWriter<DeathEvent>,
    mut query: Query<(&mut Health, &Defense, &Level)>,
) {
    for attack in ev_attack.iter() {
        let (mut health, defense, level) = query
            .get_mut(attack.target)
            .expect("The target of an attack should have Health and Defense.");

        println!(
            "{:?} attacking {:?} for {} damage!",
            attack.attacker, attack.target, attack.attack.damage
        );
        reduce_health_from_attack(&mut health.value, &defense.value, &attack.attack.damage);

        // The reader for DeathEvents will despawn the gladiator that died and award XP to the
        // gladiator that made the kill.
        if health.value < 0.0 {
            ev_death.send(DeathEvent {
                victor: attack.attacker,
                xp_earned: level.convert_to_xp(),
                slain: attack.target,
            })
        }
    }
}

fn gladiator_death_handler(
    mut commands: Commands,
    mut ev_death: EventReader<DeathEvent>,
    mut query: Query<&mut Level, With<Gladiator>>,
) {
    for event in ev_death.iter() {
        let mut victor_level = query
            .get_mut(event.victor)
            .expect("Victor of engagement should exist in ECS.");
        victor_level.gain_xp(event.xp_earned);
        commands.entity(event.victor).remove::<Engagement>();
        println!("{:?} is dead!", event.slain);

        commands.entity(event.slain).despawn();
    }
}

/// We could set up the engagement builder to tag gladiators as engaged with the Entity
/// of their target. Then combat is done by looping through all gladiators that are engaged
/// and emitting AttackEvents that contain the target entity. Call this gladiator_attacks().
/// The timing of attacks can differ per gladiator here by having a Timer, similar to the
/// animation timer
/// A separate system here has an event reader that loops through all of the attack events,
/// applying the damage to the health based on the defense. In this service, if the health
/// is dropped below zero, we can emit a DeathEvent. The death event will send the level
/// of the gladiator that died (used for rewarding experience) as well as the Entity who
/// should recieve the experience. In order to have this data, the attacker Entity will
/// need to be data contained in the AttackEvents. Death events will despawn the dead
/// Gladiator and the Gladiator that receives the experience will despawn the Engagement
/// on itself so that they become an eligible bachelor yet again.
// fn gladiator_combat(
//     engagement_query: Query<&Engagement>,
//     mut gladiator_query: Query<(&mut Health, &mut Level, &Attack, &Defense), With<Gladiator>>,
// ) {
//     for engagement in &engagement_query {
//         let (mut health_a, mut level_a, attack_a, defense_a) = gladiator_query
//             .get_mut(engagement.gladiator_a)
//             .expect("A gladiator in an engagement should exist in the ECS");
//         let (mut health_b, mut level_b, attack_b, defense_b) = gladiator_query
//             .get_mut(engagement.gladiator_b)
//             .expect("A gladiator in an engagement should exist in the ECS");

//         // TODO So I can't do both querys at the same time with mutable reference. I could just get the data, perform logic, returning the new data, then
//         //  get mutable reference again to one at a time mutate, bounded by little mini scopes. hmmmm, don't love it.
//         // do_combat(
//         //     health_a,
//         //     level_a,
//         //     attack_a,
//         //     defense_a,
//         //     &mut *health_b,
//         //     &mut *level_b,
//         //     attack_b,
//         //     defense_b,
//         // );
//     }
// }

// fn do_combat(
//     &mut health_a: &mut Health,
//     level_a: &mut Level,
//     attack_a: &Attack,
//     defense_a: &Defense,
//     health_b: &mut Health,
//     level_b: &mut Level,
//     attack_b: &Attack,
//     defense_b: &Defense,
// ) {
//     // do stuff
// }

fn spawn_gladiators(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for i in 0..N_GLADIATORS {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-WINDOW_WIDTH..WINDOW_WIDTH);
        let y = rng.gen_range(-WINDOW_HEIGHT..WINDOW_HEIGHT);
        spawn_one_gladiator(
            Vec2::new(x, y),
            i as usize,
            &mut commands,
            &asset_server,
            &mut texture_atlases,
        );
    }
}

/// Spawns a gladiator not controlled by the player
fn spawn_one_gladiator(
    location: Vec2,
    gladiator_idx: usize,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    // grab a different spritesheet based on gladiator_idx
    let path = format!(
        "{}{}",
        GLADIATOR_SPRITES_PATH,
        GLADIATOR_SPRITES[(gladiator_idx % GLADIATOR_SPRITES.len()) as usize]
    );

    let texture_handle = asset_server.load(&path);
    // The values used in the next function are specific to the Puny Characters sprite sheets
    // TODO lazy static this?
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(24.0, 24.0),
        24,
        8,
        Some(Vec2::new(8.0, 8.0)),
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // set size and starting location of the gladiator
    let mut transform = Transform::from_scale(Vec3::splat(GLADIATOR_SIZE));
    transform.translation = location.extend(1.0);

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform,
            ..default()
        })
        .insert(GladiatorBundle::new());
}

/// Moves gladiators not controlled by the player
/// TODO we can have a disjoint query here. One that is With<Engagement>
/// and the other that is Without<Engagement> and handle the movement
/// differently.
/// It would be helpful to break this out into two functions to do that.
/// For now just going to filter query to remove engaged Gladiators
fn gladiator_movement(
    mut query: Query<
        (&mut Transform, &Movement, &mut Animation, Entity),
        (With<Gladiator>, Without<Player>, Without<Engagement>),
    >,
    mut ev_grid_change: EventWriter<GridChangeEvent>,
) {
    for (mut transform, movement, mut animation, entity) in &mut query {
        animation.animation_type = AnimationType::Walk;

        let mut direction_probability = GladiatorDirectionProbability::new();

        // determine previous direction to weight the probability.
        let prev_direction_weight = 100;
        match animation.animation_direction {
            GladiatorDirection::Down => direction_probability.down = prev_direction_weight,
            GladiatorDirection::DownRight => {
                direction_probability.down_right = prev_direction_weight
            }
            GladiatorDirection::Right => direction_probability.right = prev_direction_weight,
            GladiatorDirection::UpRight => direction_probability.up_right = prev_direction_weight,
            GladiatorDirection::Up => direction_probability.up = prev_direction_weight,
            GladiatorDirection::UpLeft => direction_probability.up_left = prev_direction_weight,
            GladiatorDirection::Left => direction_probability.left = prev_direction_weight,
            GladiatorDirection::DownLeft => direction_probability.down_left = prev_direction_weight,
        }

        let new_direction = direction_probability.get_direction();
        let (mut x_movement, mut y_movement) = new_direction.to_movement();
        animation.animation_direction = new_direction;

        // maintain either left or right, otherwise default to left
        // This movement is just a placeholder until they get path planning.
        // let mut x_movement: i16 = -1;
        // (animation.animation_direction, x_movement) = match animation.animation_direction {
        //     GladiatorDirection::Down => (GladiatorDirection::Left, -1),
        //     GladiatorDirection::DownRight => (GladiatorDirection::Left, -1),
        //     GladiatorDirection::Right => (GladiatorDirection::Right, 1),
        //     GladiatorDirection::UpRight => (GladiatorDirection::Left, -1),
        //     GladiatorDirection::Up => (GladiatorDirection::Left, -1),
        //     GladiatorDirection::UpLeft => (GladiatorDirection::Left, -1),
        //     GladiatorDirection::Left => (GladiatorDirection::Left, -1),
        //     GladiatorDirection::DownLeft => (GladiatorDirection::Left, -1),
        // };

        // if too far left, go right
        if (transform.translation[0] + x_movement) < (-WINDOW_WIDTH / 2.) {
            animation.animation_direction = GladiatorDirection::Right;
            x_movement = 1.0;
        }

        // if too far right, go left
        if (transform.translation[0] + x_movement) > (WINDOW_WIDTH / 2.) {
            animation.animation_direction = GladiatorDirection::Left;
            x_movement = -1.0;
        }

        // if too far down, go up
        if (transform.translation[1] + y_movement) < (-WINDOW_HEIGHT / 2.) {
            animation.animation_direction = GladiatorDirection::Up;
            y_movement = 1.0;
        }

        // if too far up, go down
        if (transform.translation[1] + y_movement) > (WINDOW_HEIGHT / 2.) {
            animation.animation_direction = GladiatorDirection::Left;
            y_movement = -1.0;
        }

        // determine previous grid location
        let prev_grid_location =
            ArenaGrid::get_grid_location(transform.translation[0], transform.translation[1]);

        // apply the movement
        let translation_delta =
            Vec3::new(x_movement.into(), y_movement.into(), 0.0) * movement.speed;
        transform.translation += translation_delta;

        // determine current grid location
        let current_grid_location =
            ArenaGrid::get_grid_location(transform.translation[0], transform.translation[1]);

        // emit event if entering a new grid location
        if current_grid_location != prev_grid_location {
            ev_grid_change.send(GridChangeEvent {
                entity,
                prev_loc: prev_grid_location,
                curr_loc: current_grid_location,
            });
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct AttackTimer(Timer);

#[derive(Component)]
pub struct Movement {
    pub speed: f32,
}

#[derive(Component)]
pub struct Gladiator;

#[derive(Component)]
pub struct Health {
    pub value: f32,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Attack {
    damage: f32,
}

#[derive(Component)]
pub struct Defense {
    value: f32,
}

#[derive(Component)]
pub struct Level {
    level: usize,
    xp: f32,
}

impl Level {
    pub fn convert_to_xp(&self) -> f32 {
        // TODO placeholder math
        let level_xp_base: f32 = 2.;
        self.xp + level_xp_base.powf(self.level as f32)
    }

    pub fn gain_xp(&mut self, xp_earned: f32) {
        // TODO placeholder math
        let level_base: f32 = 3.;
        let next_level_xp = level_base.powf(self.level as f32);
        if self.xp + xp_earned >= next_level_xp {
            self.xp += xp_earned - next_level_xp;
            self.level += 1;
        } else {
            self.xp += xp_earned;
        }
    }
}

pub enum GladiatorEngagementStatus {
    Engaged,
    Unengaged,
}

#[derive(Component)]
pub struct GladiatorEngagement {
    pub status: GladiatorEngagementStatus,
}

#[derive(Bundle)]
pub struct GladiatorBundle {
    // needs Health, Level, Attack, and Defense
    gladiator: Gladiator,
    movement: Movement,
    animation: Animation,
    animation_timer: AnimationTimer,
    attack_timer: AttackTimer,
    health: Health,
    level: Level,
    attack: Attack,
    defense: Defense,
    engagement: GladiatorEngagement,
}

impl GladiatorBundle {
    pub fn new() -> Self {
        // the default values of the animation will quickly get overwritten
        Self {
            gladiator: Gladiator,
            movement: Movement {
                speed: GLADIATOR_SPEED,
            },
            animation: Animation {
                animation_type: AnimationType::Idle,
                animation_direction: GladiatorDirection::Down,
                frame_index: 0,
            },
            animation_timer: AnimationTimer(Timer::from_seconds(
                ANIMATION_STEP,
                TimerMode::Repeating,
            )),
            attack_timer: AttackTimer(Timer::from_seconds(ATTACK_STEP, TimerMode::Repeating)),
            health: Health { value: 10.0 },
            level: Level { level: 1, xp: 0. },
            attack: Attack { damage: 1.0 },
            defense: Defense { value: 0.1 },
            engagement: GladiatorEngagement {
                status: GladiatorEngagementStatus::Unengaged,
            },
        }
    }
}

struct GladiatorDirectionProbability {
    pub down: u32,
    pub down_right: u32,
    pub right: u32,
    pub up_right: u32,
    pub up: u32,
    pub up_left: u32,
    pub left: u32,
    pub down_left: u32,
    categorical_distribution: WeightedIndex<u32>,
    directions: Vec<GladiatorDirection>,
    rng: ThreadRng,
}

impl GladiatorDirectionProbability {
    pub fn new() -> Self {
        let directions = vec![
            GladiatorDirection::Down,
            GladiatorDirection::DownRight,
            GladiatorDirection::Right,
            GladiatorDirection::UpRight,
            GladiatorDirection::Up,
            GladiatorDirection::UpLeft,
            GladiatorDirection::Left,
            GladiatorDirection::DownLeft,
        ];

        // bad default
        let categorical_distribution = WeightedIndex::new(&[1, 1, 1, 1, 1, 1, 1, 1]).unwrap();
        Self {
            down: 1,
            down_right: 1,
            right: 1,
            up_right: 1,
            up: 1,
            up_left: 1,
            left: 1,
            down_left: 1,
            categorical_distribution,
            directions,
            rng: rand::thread_rng(),
        }
    }

    pub fn update_categorical_distribution(&mut self) {
        self.categorical_distribution = WeightedIndex::new(&[
            self.down,
            self.down_right,
            self.right,
            self.up_right,
            self.up,
            self.up_left,
            self.left,
            self.down_left,
        ])
        .unwrap();
    }

    pub fn get_direction(&mut self) -> GladiatorDirection {
        self.update_categorical_distribution();
        // let mut rng = rand::thread_rng();
        self.directions[self.categorical_distribution.sample(&mut self.rng)]
    }
}
