use bevy::prelude::{Commands, Component, Res, Sprite, Timer, TimerMode, Transform};
use bevy::math::{Quat, Vec3};
use bevy::asset::AssetServer;
use std::f32::consts::PI;
use rand::random;
use crate::constants::{ENEMY_MAX_COUNT, ENEMY_SPRITE_PATH, ENEMY_STARTING_POSITION, SPRITE_SCALE};

#[derive(Component)]
pub struct Enemy {
    pub health: i8,
    pub center: Vec3,
    pub direction: Vec3,
    pub movement_timer: Timer,
    pub bullet_timer: Timer,
}

impl Enemy {
    fn new(center: Vec3, direction: Vec3) -> Self {
        Self {
            health: i8::MAX,
            center,
            direction,
            movement_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            bullet_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        }
    }
}

pub fn spawn_enemies(mut commands: Commands, asset_server: Res<AssetServer>) {
    let rotation = Quat::from_axis_angle(Vec3::Z, 180. * PI / 4.);
    for c in 0..ENEMY_MAX_COUNT {
        let x = (c - (ENEMY_MAX_COUNT / 2)) as f32 * 75.;
        let direction:f32 = if random::<bool>() { -1. } else { 1. };
        let y_offset = (random::<u8>() % 20) as f32 * direction;
        let pos = Vec3::new(x, ENEMY_STARTING_POSITION.y, 0.);
        commands.spawn((
            Sprite::from_image(asset_server.load(ENEMY_SPRITE_PATH)),
            Transform {
                translation: pos + Vec3::new(0.,y_offset, 0.),
                rotation: rotation.clone(),
                scale: Vec3::splat(SPRITE_SCALE),
            },
            Enemy::new(pos, Vec3::new(0.,direction,0.))
        ));
    }
}