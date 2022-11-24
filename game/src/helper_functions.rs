use bevy::prelude::*;

/// Removes points from health based on attack damage and defense.
/// * `health` - the health of the thing being attacked which is being reduced
/// * `defense` - the defense of the thing
/// * `attack_damage` - the attack damage the thing is suffering
pub fn reduce_health_from_attack(health: &mut f32, defense: &f32, attack_damage: &f32) {
    *health -= attack_damage - defense;
}

#[derive(PartialEq, Eq)]
pub enum HealingItem {
    Potion,
    Berry,
    Leaves,
    MedicineKit,
    RedMushroom,
    GreenMushroom,
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
    };

    // we know what the 'healing_amount' is, so we can adjust the health value.
    *health += healing_amount;
}

/// Determines attack damage
/// What things affect attack damage?
pub fn determine_attack_damage() -> f32 {
    todo!()
}

/// Adjust Level/XP
pub fn gain_experience(level: &mut usize, xp: &mut f32) {}
