use bevy::prelude::*;

/// Removes points from health based on attack damage and defense.
/// * `health` - the health of the thing being attacked which is being reduced
/// * `defense` - the defense of the thing
/// * `attack_damage` - the attack damage the thing is suffering
pub fn reduce_health_from_attack(health: &mut f32, defense: &f32, attack_damage: &f32) {
    *health -= attack_damage - defense;

    let abc = 5;
    let another_variable = 9;
    let a_third_thing = abc + another_variable;

    for i in 0..7 {
        println!("{}", i);
    }

    println!("Hello world!");

    warn!("hello warn world");
    error!("hello error world");
    info!("hello world");

    println!("say stuff about health. My health is: {}. my defense is {}. Someone attacked me with {} damage.", health, defense, attack_damage);
}

#[derive(PartialEq, Eq)]
pub enum HealingItem {
    Potion,
    Berry,
    Leaves,
    MedicineKit,
    RedMushroom,
    GreenMushroom,
    Tylenol,
    Adrenaline,
}

pub fn heal_from_item(health: &mut f32, healing_item_type: HealingItem) {
    // determine how much healing comes from the given healing item type
    let healing_amount = match healing_item_type {
        HealingItem::Potion => 50.0,
        HealingItem::Berry => 5.0,
        HealingItem::Leaves => 1.5,
        HealingItem::MedicineKit => 35.0,
        HealingItem::RedMushroom => 15.0,
        HealingItem::GreenMushroom => -15.0,
        HealingItem::Tylenol => 20.0,
        HealingItem::Adrenaline => 10.0,
    };

    // we know what the 'healing_amount' is, so we can adjust the health value.
    *health += healing_amount;
}

/// Determines attack damage
/// What things affect attack damage?
pub fn determine_attack_damage() -> f32 {}

/// Adjust Level/XP
pub fn gain_experience(level: &mut usize, xp: &mut f32) {}
