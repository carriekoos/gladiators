use bevy::prelude::*;

use crate::gladiator::gladiator::Class;

#[derive(Component)]
pub struct Gladiator;

#[derive(Component)]
pub struct GladiatorClass {
    pub class: Class,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct AttackTimer(pub Timer);

#[derive(Component)]
pub struct Movement {
    pub speed: f32,
}

#[derive(Component)]
pub struct Health {
    pub value: f32,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Attack {
    pub damage: f32,
}

#[derive(Component)]
pub struct Defense {
    pub value: f32,
}

#[derive(Component)]
pub struct Level {
    pub level: usize,
    pub xp: f32,
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
