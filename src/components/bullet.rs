use bevy::prelude::{Component, Timer, TimerMode};
use rand::random;
use crate::constants::PLAYER_STARTING_POSITION;

pub enum BulletState {
    Fired,
    Igniting,
    Exploding,
    Exploded,
    Smoke,
    Despawn,
}

#[derive(Eq, Hash, PartialEq)]
pub enum BulletType {
    Player,
    Enemy
}

#[derive(Component)]
pub struct Bullet {
    pub(crate) state: BulletState,
    pub(crate) timer: Timer,
    pub(crate) reach: f32,
    pub(crate) bullet_type: BulletType,
}

impl Bullet {
    pub fn fire(bullet_type: BulletType) -> Self {
        let offset: i32 = if random::<bool>() { 1 } else { -1 };
        let reach = match bullet_type {
            BulletType::Player => 200. + ((random::<u32>() % 50) as i32 * offset) as f32,
            BulletType::Enemy => PLAYER_STARTING_POSITION.y - ((random::<u32>() % 50) as i32 * offset) as f32
        };
        Self {
            state: BulletState::Fired,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            reach,
            bullet_type
        }
    }
    pub fn direction(&self) -> f32 {
        match self.bullet_type {
            BulletType::Player => 1.,
            BulletType::Enemy => -1.
        }
    }
}