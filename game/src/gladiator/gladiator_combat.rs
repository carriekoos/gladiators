use bevy::prelude::*;

use crate::{
    animation::*,
    engagements::*,
    gladiator::{gladiator::*, gladiator_components::*, gladiator_events::*},
    helper_functions::*,
};

pub fn gladiator_attacks(
    time: Res<Time>,
    mut ev_attack: EventWriter<AttackEvent>,
    mut query: Query<(
        &Engagement,
        &Attack,
        &mut AttackTimer,
        &mut Animation,
        &GladiatorClass,
        Entity,
    )>,
) {
    for (engagement, attack, mut attack_timer, mut animation, class, entity) in &mut query {
        // determine correct attack animation
        let combat_animation_type = match &class.class {
            Class::Archer => AnimationType::Bow,
            Class::Mage => AnimationType::Staff,
            Class::Fighter => AnimationType::Sword,
        };

        // initialize animation type if switching from another animation.
        if animation.animation_type != combat_animation_type {
            animation.animation_type = combat_animation_type;
            animation.frame_index = 0;
        }

        attack_timer.tick(time.delta());
        if attack_timer.just_finished() {
            ev_attack.send(AttackEvent {
                target: engagement.target,
                attacker: entity,
                attack: *attack,
            });
        }
    }
}

pub fn gladiator_receive_attack(
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

pub fn gladiator_death_handler(
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
