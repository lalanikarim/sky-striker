use bevy::prelude::{Component, Timer, TimerMode};
use rand::random;

pub enum BulletState {
    Fired,
    Igniting,
    Exploding,
    Exploded,
    Smoke,
    Despawn,
}

#[derive(Component)]
pub struct Bullet {
    pub(crate) state: BulletState,
    pub(crate) timer: Timer,
    pub(crate) reach: f32,
}

impl Default for Bullet {
    fn default() -> Self {
        let direction: i32 = if random::<bool>() { 1 } else { -1 };
        Self {
            state: BulletState::Fired,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            reach: 200. + ((random::<u32>() % 50) as i32 * direction) as f32,
        }
    }
}