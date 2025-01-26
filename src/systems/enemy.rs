use std::f32::consts::PI;
use bevy::math::{Quat, Vec3};
use bevy::prelude::{AssetServer, Commands, Query, Res, Time, Transform};
use bevy::sprite::Sprite;
use rand::random;
use crate::components::{Bullet, BulletType, Enemy};
use crate::constants::{BULLET_SPRITE_PATH, ENEMY_MAX_COUNT, ENEMY_SPRITE_PATH, ENEMY_STARTING_POSITION, ENEMY_VELOCITY, SPRITE_SCALE};

pub fn move_enemies(mut enemy_query: Query<(&mut Transform, &mut Enemy)>, time: Res<Time>) {
    for (mut position, mut enemy) in enemy_query.iter_mut() {
        let new_position = position.translation.y + ((enemy.direction.y * ENEMY_VELOCITY) * time.delta_secs());
        if new_position >= 150. && new_position < 250. {
            position.translation.y = new_position;
        } else {
            enemy.direction.y *= -1.;
        }
    }
}

pub fn spawn_enemy_bullets(mut commands: Commands, mut enemy_query: Query<(&Transform, &mut Enemy)>, time: Res<Time>, asset_server: Res<AssetServer>) {
    for(position,mut enemy) in enemy_query.iter_mut() {
        enemy.first_bullet_timer.tick(time.delta());
        if enemy.first_bullet_timer.paused() || enemy.first_bullet_timer.finished() {
            enemy.bullet_timer.tick(time.delta());
        }
        if enemy.bullet_timer.finished() || enemy.first_bullet_timer.finished() {
            commands.spawn((
                Sprite::from_image(asset_server.load(BULLET_SPRITE_PATH)),
                Transform {
                    translation: position.translation + Vec3::new(0.,0.,-1.),
                    rotation: Quat::from_rotation_z(180. * PI / 4.),
                    ..Default::default()
                },
                Bullet::fire(BulletType::Enemy)
            ));
        }
        if enemy.first_bullet_timer.finished() {
            enemy.first_bullet_timer.reset();
            enemy.first_bullet_timer.pause();
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
                translation: pos + Vec3::new(0.,y_offset, 1.),
                rotation: rotation.clone(),
                scale: Vec3::splat(SPRITE_SCALE),
            },
            Enemy::new(pos, Vec3::new(0.,direction,0.))
        ));
    }
}