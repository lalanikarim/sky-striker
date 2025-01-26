use bevy::prelude::{Component, Timer, TimerMode};
use crate::constants::{MAGAZINE_CAPACITY, RELOAD_TIME};

#[derive(Component)]
pub struct Player {
    pub(crate) target_position: f32,
    pub(crate) reload_timer: Timer,
    pub(crate) bullets: u32,
    pub(crate) health: u32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            target_position: 0.,
            reload_timer: Timer::from_seconds(RELOAD_TIME, TimerMode::Repeating),
            bullets: MAGAZINE_CAPACITY,
            health: 25,
        }
    }
}