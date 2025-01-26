use bevy::prelude::*;
use bevy::asset::AssetServer;
use crate::components::{Bullet, BulletState, BulletType, Enemy, Player, ScoreBoard};
use crate::constants::{BULLET_VELOCITY, EXPLOSION_SPRITE_01_PATH, EXPLOSION_SPRITE_02_PATH, EXPLOSION_SPRITE_03_PATH, EXPLOSION_SPRITE_04_PATH};

pub fn move_bullets(mut bullet_query: Query<(&mut Transform, &mut Bullet)>, time: Res<Time>) {
    for (mut position, mut bullet) in bullet_query.iter_mut() {
        let moving = bullet.state == BulletState::Fired && match bullet.bullet_type {
            BulletType::Player => position.translation.y <= bullet.reach.into(),
            BulletType::Enemy => position.translation.y >= bullet.reach.into(),
        };
        if moving {
            position.translation.y += BULLET_VELOCITY * bullet.direction() * time.delta_secs();
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

pub fn check_collisions(mut commands: Commands,
                        mut bullet_query: Query<(&mut Bullet, &Transform)>,
                        mut player_query: Single<(Entity, &Transform, &Sprite, &mut Player)>,
                        mut enemy_query: Query<(Entity, &Transform, &Sprite), With<Enemy>>,
                        mut score_query: Single<&mut ScoreBoard>,
                        assets: Res<Assets<Image>>,
) {
    let (player_entity, player_position, player_image, mut player) = player_query.into_inner();
    let player_box = get_bounding_box(player_position, player_image, &assets);
    let mut score = score_query.into_inner();
    for (mut bullet, transform) in bullet_query.iter_mut() {
        if bullet.state == BulletState::Fired {
            match bullet.bullet_type {
                BulletType::Enemy => {
                    if collided(&transform.translation, &player_box) {
                        bullet.state = BulletState::Igniting;
                        player.health -= 1;
                        if player.health == 0 {
                            commands.entity(player_entity).despawn();
                        }
                    }
                },
                BulletType::Player => {
                    for (enemy_entity, enemy_position, enemy_sprite) in enemy_query.iter_mut() {
                        let enemy_box = get_bounding_box(enemy_position,enemy_sprite, &assets);
                        if collided(&transform.translation,&enemy_box){
                            bullet.state = BulletState::Igniting;
                            commands.entity(enemy_entity).despawn();
                            score.0 += 1;
                        }
                    }
                }
            }
        }
    }
}

fn get_bounding_box(transform: &Transform, sprite: &Sprite,
                    assets: &Res<Assets<Image>>,
) -> Rect {
    let image_dimensions = assets.get(&sprite.image).unwrap().size().as_vec2();
    let scaled_image_dimensions = image_dimensions * transform.scale.truncate();
    Rect::from_center_size(transform.translation.truncate(), scaled_image_dimensions)
}

fn collided(bullet_position: &Vec3, bounding_rect: &Rect) -> bool {
    let bullet_v2 = Vec2::new(bullet_position.x, bullet_position.y);
    bounding_rect.contains(bullet_v2)
}