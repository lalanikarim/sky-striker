use bevy::prelude::{Commands, EventReader, KeyCode, Res, Single, Sprite, Time, Transform};
use bevy::input::keyboard::KeyboardInput;
use bevy::math::Vec3;
use bevy::asset::AssetServer;
use crate::components::{Bullet, BulletType, Player};
use crate::constants::{BULLET_SPRITE_PATH, MAGAZINE_CAPACITY, PLAYER_STARTING_POSITION, PLAYER_VELOCITY};

pub fn steer_aircraft(
    player_query: Single<(&Transform, &mut Player)>,
    mut char_input_events: EventReader<KeyboardInput>,
) {
    let (transform, mut player) = player_query.into_inner();
    for event in char_input_events.read() {
        if event.state.is_pressed() {
            let direction: f32 = match event.key_code {
                KeyCode::ArrowLeft => -1.0,
                KeyCode::ArrowRight => 1.0,
                _ => 0.0,
            };
            if direction != 0. {
                let new_position = transform.translation.x + (direction * PLAYER_VELOCITY);
                if new_position >= -500. && new_position <= 500. {
                    player.target_position = new_position;
                }
            }
        }
    }
}

pub fn move_aircraft(
    time: Res<Time>,
    player_query: Single<(&mut Transform, &Player)>,
) {
    let (mut position, player) = player_query.into_inner();
    let dest = Vec3::new(player.target_position, PLAYER_STARTING_POSITION.y, 0.);
    position.translation.x += (dest.x - position.translation.x) * time.delta_secs();
}

pub fn shoot_bullets(
    mut commands: Commands,
    player_query: Single<(&Transform, &mut Player)>,
    mut char_input_events: EventReader<KeyboardInput>,
    asset_server: Res<AssetServer>,
) {
    let (position, mut player) = player_query.into_inner();
    for event in char_input_events.read() {
        if !event.state.is_pressed() && event.key_code == KeyCode::Space {
            let transform = Transform {
                translation: Vec3::new(position.translation.x, position.translation.y, -1.),
                ..Default::default()
            };
            if player.bullets > 0 {
                player.bullets -= 1;
                commands.spawn((
                    Sprite::from_image(asset_server.load(BULLET_SPRITE_PATH)),
                    transform,
                    Bullet::fire(BulletType::Player),
                ));
            }
        }
    }
}

pub fn reload(player_query: Single<&mut Player>, time: Res<Time>) {
    let mut player = player_query.into_inner();
    player.reload_timer.tick(time.delta());
    if player.reload_timer.finished() {
        if player.bullets < MAGAZINE_CAPACITY {
            player.bullets = MAGAZINE_CAPACITY;
        }
    }
}