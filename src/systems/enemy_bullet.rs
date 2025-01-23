use bevy::prelude::*;
use crate::components::EnemyBullet;
use crate::constants::{BULLET_VELOCITY, PLAYER_STARTING_POSITION};

pub fn move_enemy_bullets(mut commands: Commands, mut bullet_query: Query<(Entity, &mut Transform), With<EnemyBullet>>, time: Res<Time>) {
    for (entity, mut transform) in bullet_query.iter_mut() {
        if transform.translation.y > PLAYER_STARTING_POSITION.y - 50. {
            let new_position = transform.translation.y - (BULLET_VELOCITY * time.delta_secs());
            transform.translation.y = if new_position >= PLAYER_STARTING_POSITION.y - 50. {
                new_position
            } else {
                PLAYER_STARTING_POSITION.y - 50.
            };
        } else {
            commands.entity(entity).despawn();
        }
    }
}