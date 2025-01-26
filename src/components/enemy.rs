use bevy::prelude::{Component, Timer, TimerMode};
use bevy::math::Vec3;
use rand::random;
use crate::constants::ENEMY_BULLET_RESPAWN_DURATION;

#[derive(Component)]
pub struct Enemy {
    pub health: i8,
    pub center: Vec3,
    pub direction: Vec3,
    pub bullet_timer: Timer,
    pub first_bullet_timer: Timer,
}

impl Enemy {
    pub(crate) fn new(center: Vec3, direction: Vec3) -> Self {
        Self {
            health: i8::MAX,
            center,
            direction,
            bullet_timer: Timer::from_seconds(ENEMY_BULLET_RESPAWN_DURATION, TimerMode::Repeating),
            first_bullet_timer: Timer::from_seconds(random::<f32>().abs() * 5., TimerMode::Once),
        }
    }
}

