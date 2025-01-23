use bevy::prelude::{Query, Res, Time, Transform};
use crate::components::Enemy;
use crate::constants::ENEMY_VELOCITY;

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