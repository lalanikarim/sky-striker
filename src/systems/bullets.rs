use bevy::prelude::{Commands, Entity, Query, Res, Sprite, Time, Transform};
use bevy::asset::AssetServer;
use crate::components::{Bullet, BulletState};
use crate::constants::{EXPLOSION_SPRITE_01_PATH, EXPLOSION_SPRITE_02_PATH, EXPLOSION_SPRITE_03_PATH, EXPLOSION_SPRITE_04_PATH};

pub fn move_bullets(mut bullet_query: Query<(&mut Transform, &mut Bullet)>, time: Res<Time>) {
    for (mut position, mut bullet) in bullet_query.iter_mut() {
        if position.translation.y < bullet.reach.into() {
            position.translation.y += 100. * time.delta_secs();
        } else {
            bullet.timer.tick(time.delta());
            if bullet.timer.finished() {
                bullet.state = match bullet.state {
                    BulletState::Fired => BulletState::Igniting,
                    BulletState::Igniting => BulletState::Exploding,
                    BulletState::Exploding => BulletState::Exploded,
                    BulletState::Exploded => BulletState::Smoke,
                    _ => BulletState::Despawn,
                }
            }
        }
    }
}

pub fn animate_bullets(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &mut Sprite, &mut Bullet)>,
    asset_server: Res<AssetServer>,
) {
    for (entity, mut sprite_handle, bullet) in bullet_query.iter_mut() {
        match bullet.state {
            BulletState::Fired => {}
            BulletState::Igniting => {
                sprite_handle.image = asset_server.load(EXPLOSION_SPRITE_01_PATH);
            }
            BulletState::Exploding => {
                sprite_handle.image = asset_server.load(EXPLOSION_SPRITE_02_PATH);
            }
            BulletState::Exploded => {
                sprite_handle.image = asset_server.load(EXPLOSION_SPRITE_03_PATH);
            }
            BulletState::Smoke => {
                sprite_handle.image = asset_server.load(EXPLOSION_SPRITE_04_PATH);
            }
            BulletState::Despawn => {
                commands.entity(entity).despawn();
            }
        }
    }
}